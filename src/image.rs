use std::{
    cmp, fmt, fs,
    io::{self, Write},
    mem,
    ops::{Index, IndexMut},
    process::{Command, ExitStatus, Stdio},
    sync::RwLock,
};

use rayon::iter::{IntoParallelIterator, ParallelIterator};

use crate::{
    lighter::LightingConfig,
    matrix::{Const2D, Dynamic2D, EdgeMatrix, ParallelGrid, PolygonMatrix},
    parser, Color, Lighter, Vector3D,
};

const TESTDIR: &str = "test_images/";
#[derive(Clone, Debug)]
pub struct Image<const WIDTH: usize, const HEIGHT: usize> {
    name: Option<String>,
    data: Box<Const2D<Color, WIDTH, HEIGHT>>,
    zbuffer: Dynamic2D<f64>,
    lighter: Lighter,
}

impl Image<{ parser::FINAL_SCREEN_SIZE }, { parser::FINAL_SCREEN_SIZE }> {
    pub fn downsample(&self) -> Image<{ parser::SCREEN_SIZE }, { parser::SCREEN_SIZE }> {
        let mut result: Image<{ parser::SCREEN_SIZE }, { parser::SCREEN_SIZE }> =
            Default::default();

        for r in 0..parser::SCREEN_SIZE {
            for c in 0..parser::SCREEN_SIZE {
                let mut ravg = 0.0;
                let mut gavg = 0.0;
                let mut bavg = 0.0;
                for i in
                    (r * parser::SAMPLE_SCALE as usize)..((r + 1) * parser::SAMPLE_SCALE as usize)
                {
                    for j in (c * parser::SAMPLE_SCALE as usize)
                        ..((c + 1) * parser::SAMPLE_SCALE as usize)
                    {
                        ravg += self[i][j].red as f64;
                        gavg += self[i][j].green as f64;
                        bavg += self[i][j].blue as f64;
                    }
                }
                ravg /= parser::SAMPLE_SCALE * parser::SAMPLE_SCALE;
                gavg /= parser::SAMPLE_SCALE * parser::SAMPLE_SCALE;
                bavg /= parser::SAMPLE_SCALE * parser::SAMPLE_SCALE;

                ravg = f64::min(ravg, 255.0);
                gavg = f64::min(gavg, 255.0);
                bavg = f64::min(bavg, 255.0);
                result[r][c] = Color {
                    red: ravg as u8,
                    green: gavg as u8,
                    blue: bavg as u8,
                };
            }
        }
        result
    }
}

impl<const WIDTH: usize, const HEIGHT: usize> Default for Image<WIDTH, HEIGHT> {
    fn default() -> Self {
        Self {
            name: None,
            data: Default::default(),
            zbuffer: Dynamic2D::fill(f64::NEG_INFINITY, WIDTH, HEIGHT),
            lighter: Default::default(),
        }
    }
}

fn dist(p0: (f64, f64, f64), p1: (f64, f64, f64)) -> f64 {
    ((p1.0 - p0.0) * (p1.0 - p0.0) + (p1.1 - p0.1) * (p1.1 - p0.1) + (p1.2 - p0.2) * (p1.2 - p0.2))
        .sqrt()
}

#[derive(Clone, Copy, Debug, Hash)]
pub enum ShadingMethod {
    Flat,
    Phong,
}

impl<const WIDTH: usize, const HEIGHT: usize> Image<WIDTH, HEIGHT> {
    pub fn new(name: String) -> Self {
        Image {
            name: Some(name),
            data: Default::default(),
            zbuffer: Dynamic2D::fill(f64::NEG_INFINITY, WIDTH, HEIGHT),
            lighter: Default::default(),
        }
    }

    pub fn get_width(&self) -> usize {
        self.data.get_width()
    }

    pub fn get_height(&self) -> usize {
        self.data.get_height()
    }

    pub fn get_lighter(&mut self) -> &mut Lighter {
        &mut self.lighter
    }

    pub fn clear(&mut self) {
        self.clear_shapes_only();
        self.clear_lighter();
    }

    pub fn clear_shapes_only(&mut self) {
        self.data = Default::default();
        self.zbuffer = Dynamic2D::fill(f64::NEG_INFINITY, WIDTH, HEIGHT);
    }

    pub fn clear_lighter(&mut self) {
        self.lighter = Default::default();
    }

    pub fn save(&self) -> io::Result<()> {
        let name = self
            .name
            .as_ref()
            .unwrap_or_else(|| panic!("No provided name field to write to"));
        let path = format!("{}.png", &name);
        self.save_name(&path)
    }

    pub fn save_test(&self) -> io::Result<()> {
        let name = self
            .name
            .as_ref()
            .unwrap_or_else(|| panic!("No provided name field to write to"));
        let path = format!("{}{}.png", TESTDIR, &name);
        self.save_name(&path)
    }

    pub fn save_name(&self, filename: &str) -> io::Result<()> {
        fs::create_dir_all(&filename.rsplit_once('/').unwrap_or((".", "")).0)?;

        let convert_syntax = format!("convert -resize 500x500 - {}", &filename);
        let mut convert_command = Command::new("sh")
            .args(["-c", &convert_syntax])
            .stdin(Stdio::piped())
            .spawn()?;

        convert_command
            .stdin
            .as_mut()
            .unwrap()
            .write_all(self.to_string().as_bytes())?;
        convert_command.wait()?;

        println!("Image can be found at {}.", &filename);
        Ok(())
    }

    pub fn display(&self) -> io::Result<ExitStatus> {
        let mut display_command = Command::new("sh")
            .env("DISPLAY", ":0")
            .args(["-c", "display -resize 500x500 -"])
            .stdin(Stdio::piped())
            .spawn()?;

        display_command
            .stdin
            .as_mut()
            .unwrap()
            .write_all(self.to_string().as_bytes())?;
        display_command.wait()
    }

    pub fn draw_matrix(&mut self, matrix: &EdgeMatrix, c: Color) {
        matrix.into_iter().for_each(|(p0, p1)| {
            self.draw_line(
                (p0.0 as i32, p0.1 as i32, 0.0),
                (p1.0 as i32, p1.1 as i32, 0.0),
                c,
            );
        });
    }

    pub fn draw_polygons(
        &mut self,
        matrix: &PolygonMatrix,
        light_conf: &LightingConfig,
        shading: ShadingMethod,
    ) {
        let lighter = self.lighter.clone();
        let image_rwlock = RwLock::new(self);
        matrix
            .into_par_iter()
            .filter(|(_points, normal)| -> bool {
                normal.dot(&Vector3D::new(0.0, 0.0, 1.0)) >= 0.0
            })
            .for_each(|(points, normal)| {
                let c = lighter.calculate(&normal, light_conf);

                let mut v = points;
                for mut point in &mut v {
                    point.0 *= parser::SAMPLE_SCALE;
                    point.1 *= parser::SAMPLE_SCALE;
                    point.2 *= parser::SAMPLE_SCALE;
                }
                // Sort by y value
                v.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());

                let mut x_straight_top = v[0].0;
                let mut x_two_part = v[0].0;

                let mut z_straight_top = v[0].2;
                let mut z_two_part = v[0].2;

                let dx_straight_top =
                    (v[2].0 - v[0].0) / (v[2].1 as i32 - v[0].1 as i32 + 1) as f64;
                let dx_bot_to_mid = (v[1].0 - v[0].0) / (v[1].1 as i32 - v[0].1 as i32 + 1) as f64;
                let dx_mid_to_top = (v[2].0 - v[1].0) / (v[2].1 as i32 - v[1].1 as i32 + 1) as f64;

                let dz_straight_top =
                    (v[2].2 - v[0].2) / (v[2].1 as i32 - v[0].1 as i32 + 1) as f64;
                let dz_bot_to_mid = (v[1].2 - v[0].2) / (v[1].1 as i32 - v[0].1 as i32 + 1) as f64;
                let dz_mid_to_top = (v[2].2 - v[1].2) / (v[2].1 as i32 - v[1].1 as i32 + 1) as f64;

                let mut curr_two_part_dx = dx_bot_to_mid;
                let mut curr_two_part_dz = dz_bot_to_mid;

                let mut swapped = false;
                (v[0].1 as i32..=v[2].1 as i32).for_each(|y| {
                    if y == v[1].1 as i32 {
                        x_two_part = v[1].0;
                        curr_two_part_dx = dx_mid_to_top;

                        z_two_part = v[1].2;
                        curr_two_part_dz = dz_mid_to_top;

                        swapped = true;
                    }

                    match shading {
                        ShadingMethod::Flat => {
                            image_rwlock.write().unwrap().draw_line(
                                (x_straight_top as i32, y, z_straight_top),
                                (x_two_part as i32, y, z_two_part),
                                c,
                            );
                        }
                        ShadingMethod::Phong => {
                            let straight_top_normal = Vector3D::interpolate(
                                [
                                    (
                                        v[0].3,
                                        dist(
                                            (x_straight_top, y as f64, z_straight_top),
                                            (v[2].0, v[2].1, v[2].2),
                                        ),
                                    ),
                                    (
                                        v[2].3,
                                        dist(
                                            (x_straight_top, y as f64, z_straight_top),
                                            (v[0].0, v[0].1, v[0].2),
                                        ),
                                    ),
                                ]
                                .into_iter(),
                            );

                            let two_part_normal = if swapped {
                                Vector3D::interpolate(
                                    [
                                        (
                                            v[1].3,
                                            dist(
                                                (x_two_part, y as f64, z_two_part),
                                                (v[2].0, v[2].1, v[2].2),
                                            ),
                                        ),
                                        (
                                            v[2].3,
                                            dist(
                                                (x_two_part, y as f64, z_two_part),
                                                (v[1].0, v[1].1, v[1].2),
                                            ),
                                        ),
                                    ]
                                    .into_iter(),
                                )
                            } else {
                                Vector3D::interpolate(
                                    [
                                        (
                                            v[0].3,
                                            dist(
                                                (x_two_part, y as f64, z_two_part),
                                                (v[1].0, v[1].1, v[1].2),
                                            ),
                                        ),
                                        (
                                            v[1].3,
                                            dist(
                                                (x_two_part, y as f64, z_two_part),
                                                (v[0].0, v[0].1, v[0].2),
                                            ),
                                        ),
                                    ]
                                    .into_iter(),
                                )
                            };

                            image_rwlock.write().unwrap().scan_line_phong(
                                y,
                                (x_straight_top as i32, z_straight_top, straight_top_normal),
                                (x_two_part as i32, z_two_part, two_part_normal),
                                &lighter,
                                light_conf,
                            );
                        }
                    }

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

    fn scan_line_phong(
        &mut self,
        y: i32,
        mut leftdata: (i32, f64, Vector3D),
        mut rightdata: (i32, f64, Vector3D),
        lighter: &Lighter,
        light_conf: &LightingConfig,
    ) {
        if y < 0 || y >= self.get_height() as i32 {
            return;
        }

        if leftdata.0 > rightdata.0 {
            mem::swap(&mut leftdata, &mut rightdata);
        }

        let (leftx, leftz, leftnormal) = leftdata;
        let (rightx, rightz, rightnormal) = rightdata;

        let mut z = leftz;
        let dz = rightz - leftz;
        let dx = rightx - leftx;
        let dzpp = dz / (dx as f64 + 1.0);

        let casty = y as usize;
        (leftx..=rightx).for_each(|x| {
            if x >= 0 && x < self.get_width() as i32 {
                let castx = x as usize;

                if z > self.zbuffer[casty][castx] {
                    self[casty][castx] = lighter.calculate(
                        &Vector3D::interpolate(
                            [
                                (
                                    leftnormal,
                                    dist(
                                        (x as f64, y as f64, z),
                                        (rightx as f64, y as f64, rightz),
                                    ),
                                ),
                                (
                                    rightnormal,
                                    dist((x as f64, y as f64, z), (leftx as f64, y as f64, leftz)),
                                ),
                            ]
                            .into_iter(),
                        ),
                        light_conf,
                    );
                    self.zbuffer[casty][castx] = z;
                }
            }
            z += dzpp;
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
        let mut corrector = 2 * dx * if down { 1 } else { -1 };

        let (mut yup, mut ydown, mut xright);
        let faster_coord_iter: &mut dyn Iterator<Item = i32> = match (steep, down) {
            (true, true) => {
                ydown = (y1..=y0).rev();
                &mut ydown
            }
            (true, false) => {
                yup = y0..=y1;
                &mut yup
            }
            (false, _) => {
                xright = x0..=x1;
                &mut xright
            }
        };

        let cmp_closure = |d: i32| -> bool {
            if steep == down {
                d >= 0
            } else {
                d <= 0
            }
        };

        let mut error: i32 = match (steep, down) {
            // D
            (true, _) => dy + corrector,
            (false, true) => error_accumulator + dx,
            (false, false) => error_accumulator - dx,
        };

        let mut slower_coord: i32 = if steep { x0 } else { y0 };
        let iter_dir: i32 = if !steep && down { -1 } else { 1 };

        if steep {
            mem::swap(&mut error_accumulator, &mut corrector);
        }

        let mut z = z0;
        let dzpp = dz / (cmp::max(dy.abs(), dx.abs()) as f64 + 1.0);

        faster_coord_iter.for_each(|faster_coord| {
            if faster_coord < 0
                || faster_coord
                    >= if steep {
                        self.get_height() as i32
                    } else {
                        self.get_width() as i32
                    }
                || slower_coord < 0
                || slower_coord
                    >= if steep {
                        self.get_width() as i32
                    } else {
                        self.get_height() as i32
                    }
            {
                return;
            }

            //let zcmp = (z * 10000.0).round() / 10000.0;
            if steep {
                if z > self.zbuffer[faster_coord as usize][slower_coord as usize] {
                    self.zbuffer[faster_coord as usize][slower_coord as usize] = z;
                    self[faster_coord as usize][slower_coord as usize] = c;
                }
            } else if z > self.zbuffer[slower_coord as usize][faster_coord as usize] {
                self.zbuffer[slower_coord as usize][faster_coord as usize] = z;
                self[slower_coord as usize][faster_coord as usize] = c;
            }

            if cmp_closure(error) {
                slower_coord += iter_dir;
                error += corrector;
            }

            error += error_accumulator;
            z += dzpp;
        });
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
        writeln!(f, "P3")?;
        writeln!(f, "{} {}", self.get_width(), self.get_height())?;
        writeln!(f, "255")?;

        for r in (0..self.get_height()).rev() {
            for c in 0..self.get_width() {
                write!(f, "{} ", self[r][c])?;
            }
        }
        writeln!(f)?;

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
        for _ in 0..500 * 500 {
            comparison_str.push_str("0 0 0 ");
        }
        assert_eq!(blank.to_string(), comparison_str);
    }

    #[test]
    fn octant1() {
        let mut blank: Image<500, 500> = Image::new("octant1".to_string());
        blank.draw_line((5, 10, 0.0), (450, 250, 0.0), color_constants::WHITE);
        blank
            .save_test()
            .expect("Octant 1 line image file write failed");
    }

    #[test]
    fn all_octants() {
        let mut blank: Image<500, 500> = Image::new("octant_all".to_string());
        blank.draw_line((5, 10, 0.0), (450, 250, 0.0), color_constants::WHITE); // octant 1
        blank.draw_line((5, 10, 0.0), (250, 450, 0.0), color_constants::WHITE); // octant 2
        blank.draw_line((400, 250, 0.0), (5, 400, 0.0), color_constants::WHITE); // octant 7
        blank.draw_line((5, 400, 0.0), (400, 250, 0.0), color_constants::WHITE); // octant 7 duplicate backwards
        blank.draw_line((250, 5, 0.0), (5, 400, 0.0), color_constants::WHITE); // octant 8
        blank
            .save_test()
            .expect("Octant 1 line image file write failed");
    }
}
