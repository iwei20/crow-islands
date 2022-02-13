use std::{fmt, fs, io, ops::{Index, IndexMut, RangeInclusive}, mem, process::Command, iter::Rev, array::IntoIter};
use crate::color::{Color, color_constants};

const TEMPDIR: &str = "temp/";
const TESTDIR: &str = "test_images/";
#[derive(Clone, Debug)]
pub struct Image {
    width: usize,
    height: usize,
    data: Vec<Color>
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

impl Image {
    pub fn new(width: usize, height: usize) -> Image {
        let data = vec![color_constants::BLACK; width * height];
        Image {
            width,
            height,
            data
        }
    }

    pub fn get_width(&self) -> usize {
        self.width
    }

    pub fn get_height(&self) -> usize {
        self.height
    } 

    pub fn write_file(&self, imagename: &str) -> io::Result<()> {
        let ppmname = format!("{}{}.ppm", TEMPDIR, imagename);
        let pngname = format!("{}.png", imagename);

        let convert_syntax = format!("convert {} {}", &ppmname, &pngname);
        let remove_syntax = format!("rm {}", &ppmname);
        let display_syntax = format!("display {}", &pngname);

        fs::create_dir_all(TEMPDIR)?;
        fs::write(&ppmname, self.to_string())?;
        
        for command in [&convert_syntax, &remove_syntax, &display_syntax] {
            Command::new("sh")
                .args(&["-c", command])
                .spawn()?
                .wait()?;
        }

        println!("Image can be found at {}.", &pngname);
        Ok(())
    }

    pub fn write_file_test(&self, imagename: &str) -> io::Result<()> {
        let ppmname = format!("{}{}.ppm", TEMPDIR, imagename);
        let pngname = format!("{}{}.png", TESTDIR, imagename);

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

        println!("Test image can be found at {}.", &pngname);
        Ok(())
    }

    pub fn draw_line(&mut self, mut p0: (i32, i32), mut p1: (i32, i32), c: Color) {
        let mut oob_warn_triggered = false;
        // Ensure p0 is the left point
        if p0.0 > p1.0 {
            mem::swap(&mut p0, &mut p1);
        }

        let (x0, y0) = p0;
        let (x1, y1) = p1;

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

        for faster_coord in faster_coord_iter {

            if faster_coord < 0 || faster_coord >= if steep {self.get_height() as i32} else {self.get_width() as i32} ||
               slower_coord < 0 || slower_coord >= if steep {self.get_width() as i32} else {self.get_height() as i32} {
                if !oob_warn_triggered {
                    eprintln!("({}, {}) to ({}, {}) drawn out of bounds!", x0, y0, x1, y1);
                    oob_warn_triggered = true;
                }
                continue;
            }
            if steep {
                self[faster_coord as usize][slower_coord as usize] = c;
            } else {
                self[slower_coord as usize][faster_coord as usize] = c;
            }

            if cmp_closure(error) {
                slower_coord += iter_dir;
                error += corrector;
            }

            error += error_accumulator;
        }
    }
}

impl Index<usize> for Image {
    type Output = [Color];
    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index * self.width .. (index+1) * self.width]
    }
}

impl IndexMut<usize> for Image {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.data[index * self.width .. (index+1) * self.width]
    }
}

impl fmt::Display for Image {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "P3\n")?;
        write!(f, "{} {}\n", self.width, self.height)?;
        write!(f, "255\n")?;
        
        self.data
            .iter()
            .try_for_each(|color| write!(f, "{} ", color))?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::color::color_constants;

    use super::Image;

    #[test]
    fn one_x_four_brgb() {
        let mut one_x_four: Image = Image::new(4, 1);
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
        let blank: Image = Image::new(500, 500);
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
        let mut blank: Image = Image::new(500, 500);
        blank.draw_line((5, 10), (450, 250), color_constants::WHITE);
        blank.write_file_test("octant1").expect("Octant 1 line image file write failed");
    }

    #[test]
    fn all_octants() {
        let mut blank: Image = Image::new(500, 500);
        blank.draw_line((5, 10), (450, 250), color_constants::WHITE); // octant 1
        blank.draw_line((5, 10), (250, 450), color_constants::WHITE); // octant 2
        blank.draw_line((400, 250), (5, 400), color_constants::WHITE); // octant 7
        blank.draw_line((5, 400), (400, 250), color_constants::WHITE); // octant 7 duplicate backwards
        blank.draw_line((250, 5), (5, 400), color_constants::WHITE); // octant 8
        blank.write_file_test("octant_all").expect("Octant 1 line image file write failed");
    }
}