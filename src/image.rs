use std::{fmt, fs, io, ops::{Index, IndexMut, RangeInclusive}, mem, process::Command, result::Iter};
use crate::color::{Color, color_constants};

const TEMPDIR: &str = "temp/";
const TESTDIR: &str = "test_images/";
#[derive(Clone, Debug)]
pub struct Image {
    width: usize,
    height: usize,
    data: Vec<Color>
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
        if p0.0 > p1.0 {
            mem::swap(&mut p0, &mut p1);
        }

        let (x0, y0) = p0;
        let (x1, y1) = p1;

        let mut A = 2 * (y1 - y0);
        let mut B = 2 * (x0 - x1);

        let steep = A.abs() > B.abs();
        let down = A.signum() == -1;
        
        let iter: Box<dyn Iterator<Item = i32>>;
        let mut D: i32;
        let cmp_closure: Box<dyn Fn(i32) ->bool>;
        let iter_dir: i32;
        let mut start_val: i32;
        let which_index: bool;
        
        println!("{} {}", steep, down);
        match (steep, down) {
            (true, true) => {
                iter = Box::new((y1..=y0).rev());
                B *= -1;
                D = (y1 - y0) + B;
                iter_dir = 1;
                start_val = x0;
                which_index = false;
                cmp_closure = Box::new(|x: i32| -> bool {x >= 0});
                mem::swap(&mut A, &mut B);
            },
            (true, false) => {
                iter = Box::new(y0..=y1);
                D = (y1 - y0) + B;
                iter_dir = 1;
                start_val = x0;
                which_index = false;
                cmp_closure = Box::new(|x: i32| -> bool {x <= 0});
                mem::swap(&mut A, &mut B);
            },
            (false, true) => {
                iter = Box::new(x0..=x1);
                B *= -1;
                D = A + (x1 - x0); // -1 * 1/2B
                iter_dir = -1;
                start_val = y0;
                which_index = true;
                cmp_closure = Box::new(|x: i32| -> bool {x <= 0});
            },
            (false, false) => {
                iter = Box::new(x0..=x1);
                D = A + (x0 - x1);
                iter_dir = 1;
                start_val = y0;
                which_index = true;
                cmp_closure = Box::new(|x: i32| -> bool {x >= 0});
            }
        }
        for changer in iter {
            println!("{} {}", start_val, D);
            if which_index {
                self[start_val as usize][changer as usize] = c;
            } else {
                self[changer as usize][start_val as usize] = c;
            }
            if cmp_closure(D) {
                start_val += iter_dir;
                D += B;
                println!("Hi");
            }
            D += A;
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