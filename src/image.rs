use std::{fmt, fs, io, ops::{Index, IndexMut, RangeInclusive}, mem, process::Command, iter::Rev};
use crate::{color::{Color}, grid::{Const2D, ParallelGrid}};

const TEMPDIR: &str = "temp/";
const TESTDIR: &str = "test_images/";
#[derive(Clone, Debug, Default)]
pub struct Image<const WIDTH: usize, const HEIGHT: usize> {
    data: Box<Const2D<Color, WIDTH, HEIGHT>>,
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

impl<const WIDTH: usize, const HEIGHT: usize> Image<WIDTH, HEIGHT> {
    pub fn new() -> Self {
        let data: Box<Const2D<Color, WIDTH, HEIGHT>> = Default::default();
        let y_invert = false;
        Image {
            data,
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

        println!("Test image can be found at {}{}.", TESTDIR, &pngname);
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
        let mut one_x_four: Image<4, 1> = Default::default();
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
        let blank: Image<500, 500> = Default::default();
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
        let mut blank: Image<500, 500> = Default::default();
        blank.draw_line((5, 10), (450, 250), color_constants::WHITE);
        blank.write_file_test("octant1").expect("Octant 1 line image file write failed");
    }

    #[test]
    fn all_octants() {
        let mut blank: Image<500, 500> = Default::default();
        blank.draw_line((5, 10), (450, 250), color_constants::WHITE); // octant 1
        blank.draw_line((5, 10), (250, 450), color_constants::WHITE); // octant 2
        blank.draw_line((400, 250), (5, 400), color_constants::WHITE); // octant 7
        blank.draw_line((5, 400), (400, 250), color_constants::WHITE); // octant 7 duplicate backwards
        blank.draw_line((250, 5), (5, 400), color_constants::WHITE); // octant 8
        blank.write_file_test("octant_all").expect("Octant 1 line image file write failed");
    }
}