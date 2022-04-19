use std::{fmt, fs, io::{self, Write}, ops::{Index, IndexMut, RangeInclusive}, mem, process::{Command, ExitStatus, Stdio}, iter::Rev, cmp};
use crate::{Color, matrix::{Const2D, ParallelGrid, EdgeMatrix, PolygonMatrix, Dynamic2D}, Vector3D};

const TEMPDIR: &str = "temp/";
const TESTDIR: &str = "test_images/";
#[derive(Clone, Debug)]
pub struct Image<const WIDTH: usize, const HEIGHT: usize> {
    name: Option<String>,
    data: Box<Const2D<Color, WIDTH, HEIGHT>>,
    zbuffer: Dynamic2D<f64>,
    y_invert: bool
}

/**
 * Used for enum dispatch on ranges for bresenham
 */
enum CoordIter {
    YUp(RangeInclusive<i32>),
    YDown(Rev<RangeInclusive<i32>>),
    XRight(RangeInclusive<i32>)
}

impl Iterator for CoordIter {
    type Item = i32;
    fn next(&mut self) -> Option<Self::Item> {
        match self {
            CoordIter::YUp(r) => r.next(),
            CoordIter::YDown(r) => r.next(),
            CoordIter::XRight(r) => r.next()
        }
    }
}

impl<const WIDTH: usize, const HEIGHT: usize> Default for Image<WIDTH, HEIGHT> {
    fn default() -> Self {
        Self { 
            name: None, 
            data: Default::default(), 
            zbuffer: Dynamic2D::fill(f64::NEG_INFINITY, WIDTH, HEIGHT),
            y_invert: true 
        }
    }
}

impl<const WIDTH: usize, const HEIGHT: usize> Image<WIDTH, HEIGHT> {
    pub fn new(name: String) -> Self {
        Image {
            name: Some(name),
            data: Default::default(),
            zbuffer: Dynamic2D::fill(f64::NEG_INFINITY, WIDTH, HEIGHT),
            y_invert: true
        }
    }

    pub fn new_flip(name: String, y_invert: bool) -> Self {
        Image {
            name: Some(name),
            data: Default::default(),
            zbuffer: Dynamic2D::fill(f64::NEG_INFINITY, WIDTH, HEIGHT),
            y_invert
        }
    }

    pub fn get_width(&self) -> usize {
        self.data.get_width()
    }

    pub fn get_height(&self) -> usize {
        self.data.get_height()
    } 

    pub fn set_y_invert(&mut self, inverted: bool) {
        self.y_invert = inverted;
    } 

    pub fn clear(&mut self) {
        self.data = Default::default();
    }

    pub fn save(&self) -> io::Result<()> {
        let name = self.name.as_ref().unwrap_or_else(|| panic!("No provided name field to write to"));
        let ppmname = format!("{}{}.ppm", TEMPDIR, name);
        let pngname = format!("{}.png", name);

        let convert_syntax = format!("convert {} {}", &ppmname, &pngname);
        let remove_syntax = format!("rm {}", &ppmname);

        fs::create_dir_all(TEMPDIR)?;
        fs::write(&ppmname, self.to_string())?;
        
        for command in [&convert_syntax, &remove_syntax] {
            Command::new("sh")
                .args(&["-c", command])
                .spawn()?
                .wait()?;
        }

        println!("Image can be found at {}.", &pngname);
        Ok(())
    }

    pub fn save_test(&self) -> io::Result<()> {
        let name = self.name.as_ref().unwrap_or_else(|| panic!("No provided name field to write to"));
        let ppmname = format!("{}{}.ppm", TEMPDIR, name);
        let pngname = format!("{}{}.png", TESTDIR, name);

        let convert_syntax = format!("convert {} {}", &ppmname, &pngname);
        let remove_syntax = format!("rm {}", &ppmname);

        fs::create_dir_all(TEMPDIR)?;
        fs::create_dir_all(TESTDIR)?;
        fs::write(&ppmname, self.to_string())?;
        
        for command in [&convert_syntax, &remove_syntax] {
            Command::new("sh")
                .args(&["-c", command])
                .spawn()?
                .wait()?;
        }

        println!("Test image can be found at {}{}.", TESTDIR, &pngname);
        Ok(())
    }

    pub fn save_name(&self, filename: &str) -> io::Result<()> {
        let ppmname = format!("{}{}.ppm", TEMPDIR, filename);
        let pngname = format!("{}.png", filename);

        let convert_syntax = format!("convert {} {}", &ppmname, &pngname);
        let remove_syntax = format!("rm {}", &ppmname);

        fs::create_dir_all(&ppmname.rsplit_once("/").unwrap_or((".", "")).0)?;
        fs::create_dir_all(&pngname.rsplit_once("/").unwrap_or((".", "")).0)?;
        fs::write(&ppmname, self.to_string())?;
        
        for command in [&convert_syntax, &remove_syntax] {
            Command::new("sh")
                .args(&["-c", command])
                .spawn()?
                .wait()?;
        }

        println!("Image can be found at {}.", &pngname);
        Ok(())
    }

    pub fn display(&self) -> io::Result<ExitStatus> {
        
        let mut display_command = Command::new("sh")
                .env("DISPLAY", ":0")
                .args(["-c", "display"])
                .stdin(Stdio::piped())
                .spawn()?;

        display_command.stdin.as_mut().unwrap().write_all(self.to_string().as_bytes())?;
        display_command.wait()
    }

    pub fn draw_matrix(&mut self, matrix: &EdgeMatrix, c: Color) {
        matrix.into_iter().for_each(|(p0, p1)| {
            self.draw_line((p0.0 as i32, p0.1 as i32, 0.0), (p1.0 as i32, p1.1 as i32, 0.0), c); 
        });
    }

    pub fn draw_polygons(&mut self, matrix: &PolygonMatrix) {
        matrix.into_iter()
            .filter(|(p0, p1, p2)| -> bool {
                let normal = Vector3D::from_points(*p0, *p1).cross(&Vector3D::from_points(*p0, *p2));
                normal.dot(&Vector3D::new(0.0, 0.0, 1.0)) >= 0.0
            })
            .for_each(|(p0, p1, p2)| {
                let c = Color::rand();

                let mut v = [p0, p1, p2];
                // Sort by y value
                v.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());



                let mut x_straight_top = v[0].0;
                let mut x_two_part = v[0].0;

                let mut z_straight_top = v[0].2;
                let mut z_two_part = v[0].2;

                let dx_straight_top = (v[2].0 - v[0].0) / (v[2].1 as i32 - v[0].1 as i32 + 1) as f64;
                let dx_bot_to_mid =   (v[1].0 - v[0].0) / (v[1].1 as i32 - v[0].1 as i32 + 1) as f64;
                let dx_mid_to_top =   (v[2].0 - v[1].0) / (v[2].1 as i32 - v[1].1 as i32 + 1) as f64;

                let dz_straight_top = (v[2].2 - v[0].2) / (v[2].1 as i32 - v[0].1 as i32 + 1) as f64;
                let dz_bot_to_mid =   (v[1].2 - v[0].2) / (v[1].1 as i32 - v[0].1 as i32 + 1) as f64;
                let dz_mid_to_top =   (v[2].2 - v[1].2) / (v[2].1 as i32 - v[1].1 as i32 + 1) as f64;

                let mut curr_two_part_dx = dx_bot_to_mid;
                let mut curr_two_part_dz = dz_bot_to_mid;
                (v[0].1 as i32..=v[2].1 as i32).for_each(|y| {
                    if y == v[1].1 as i32 {
                        x_two_part = v[1].0;
                        curr_two_part_dx = dx_mid_to_top;

                        z_two_part = v[1].2;
                        curr_two_part_dz = dz_mid_to_top;
                    }
                    self.draw_line((x_straight_top as i32, y, z_straight_top), (x_two_part as i32, y, z_two_part), c);
                    x_straight_top += dx_straight_top;
                    x_two_part += curr_two_part_dx;

                    z_straight_top += dz_straight_top;
                    z_two_part += curr_two_part_dz;
                });
                // self.draw_line((p0.0 as i32, p0.1 as i32, p0.2), (p1.0 as i32, p1.1 as i32, p1.2), c); 
                // self.draw_line((p1.0 as i32, p1.1 as i32, p1.2), (p2.0 as i32, p2.1 as i32, p2.2), c); 
                // self.draw_line((p2.0 as i32, p2.1 as i32, p2.2), (p0.0 as i32, p0.1 as i32, p0.2), c); 
            });
    }

    pub fn draw_line(&mut self, mut p0: (i32, i32, f64), mut p1: (i32, i32, f64), c: Color) {
        // Ensure p0 is the left point
        if p0.0 > p1.0 {
            mem::swap(&mut p0, &mut p1);
        }

        let (x0, y0, z0) = p0;
        let (x1, y1, z1) = p1;

        let dz = z1 - z0;
        let dy = y1 - y0;
        let dx = x1 - x0;

        let steep = dy.abs() > dx.abs();
        let down = dy.signum() == -1;

        // Bresenham variables
        let mut error_accumulator = 2 * dy;
        let mut corrector = 2 * dx * if down{1} else {-1};
        
        let faster_coord_iter: CoordIter = match (steep, down) {
            (true, true) => CoordIter::YDown((y1..=y0).rev()),
            (true, false) => CoordIter::YUp(y0..=y1),
            (false, _) => CoordIter::XRight(x0..=x1)
        };

        let cmp_closure = |d: i32| -> bool {if steep == down {d >= 0} else {d <= 0}};

        let mut error: i32 = match (steep, down) { // D
            (true, _) => {dy + corrector},
            (false, true) => {error_accumulator + dx},
            (false, false) => {error_accumulator - dx}
        };

        let mut slower_coord: i32 = if steep {x0} else {y0};
        let iter_dir: i32 = if !steep && down {-1} else {1};

        if steep {mem::swap(&mut error_accumulator, &mut corrector);}

        let mut z = z0;
        let dzpp = dz / cmp::max(dy.abs(), dx.abs()) as f64;
        
        for faster_coord in faster_coord_iter {

            if faster_coord < 0 || faster_coord >= if steep {self.get_height() as i32} else {self.get_width() as i32} ||
               slower_coord < 0 || slower_coord >= if steep {self.get_width() as i32} else {self.get_height() as i32} {
                continue;
            }

            let zcmp = (z * 10000.0).round() / 10000.0;
            if steep {
                if zcmp > self.zbuffer[faster_coord as usize][slower_coord as usize] {
                    self.zbuffer[faster_coord as usize][slower_coord as usize] = zcmp;
                    self[faster_coord as usize][slower_coord as usize] = c;
                }
            } else {
                if zcmp > self.zbuffer[slower_coord as usize][faster_coord as usize] {
                    self.zbuffer[slower_coord as usize][faster_coord as usize] = zcmp;
                    self[slower_coord as usize][faster_coord as usize] = c;
                }
            }

            if cmp_closure(error) {
                slower_coord += iter_dir;
                error += corrector;
            }

            error += error_accumulator;
            z += dzpp;
        }
    }
}

impl<const WIDTH: usize, const HEIGHT: usize> Index<usize> for Image<WIDTH, HEIGHT> {
    type Output = [Color];
    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}

impl<const WIDTH: usize, const HEIGHT: usize> IndexMut<usize> for Image<WIDTH, HEIGHT> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.data[index]
    }
}

impl<const WIDTH: usize, const HEIGHT: usize> fmt::Display for Image<WIDTH, HEIGHT> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "P3\n")?;
        write!(f, "{} {}\n", self.get_width(), self.get_height())?;
        write!(f, "255\n")?;
        
        if self.y_invert {
            for r in (0..self.get_height()).rev() {
                for c in 0..self.get_width() {
                    write!(f, "{} ", self[r][c])?;
                }
            }
        } else {
            for r in 0..self.get_height() {
                for c in 0..self.get_width() {
                    write!(f, "{} ", self[r][c])?;
                }
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::color::color_constants;

    use super::Image;

    #[test]
    fn one_x_four_brgb() {
        let mut one_x_four: Image<4, 1> = Image::new("one_x_four".to_string());
        one_x_four[0][1] = color_constants::RED;
        one_x_four[0][2] = color_constants::GREEN;
        one_x_four[0][3] = color_constants::BLUE;
        assert_eq!(
            one_x_four.to_string(),
            "P3\n\
             4 1\n\
             255\n\
             0 0 0 255 0 0 0 255 0 0 0 255 "
        );
    }

    #[test]
    fn black_500x500() {
        let blank: Image<500, 500> = Image::new("blank".to_string());
        let mut comparison_str: String = String::new();
        comparison_str.push_str("P3\n");
        comparison_str.push_str("500 500\n");
        comparison_str.push_str("255\n");
        for _ in 0..500*500 {
            comparison_str.push_str("0 0 0 ");
        }
        assert_eq!(blank.to_string(), comparison_str);
    }

    #[test]
    fn octant1() {
        let mut blank: Image<500, 500> = Image::new("octant1".to_string());
        blank.draw_line((5, 10, 0.0), (450, 250, 0.0), color_constants::WHITE);
        blank.save_test().expect("Octant 1 line image file write failed");
    }

    #[test]
    fn all_octants() {
        let mut blank: Image<500, 500> = Image::new("octant_all".to_string());
        blank.draw_line((5, 10, 0.0), (450, 250, 0.0), color_constants::WHITE); // octant 1
        blank.draw_line((5, 10, 0.0), (250, 450, 0.0), color_constants::WHITE); // octant 2
        blank.draw_line((400, 250, 0.0), (5, 400, 0.0), color_constants::WHITE); // octant 7
        blank.draw_line((5, 400, 0.0), (400, 250, 0.0), color_constants::WHITE); // octant 7 duplicate backwards
        blank.draw_line((250, 5, 0.0), (5, 400, 0.0), color_constants::WHITE); // octant 8
        blank.save_test().expect("Octant 1 line image file write failed");
    }
}
