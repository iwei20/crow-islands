use std::{fmt, fs, io, ops::{Index, IndexMut}, mem, process::Command};
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
        let A = 2 * (y1 - y0);
        let B = 2 * (x0 - x1);

        let mut D = A + (x0 - x1);
        let mut y = y0;
        for x in x0..x1 + 1 {
            self[y as usize][x as usize] = c;
            if D > 0 {
                y += 1;
                D += B;
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
}