
use std::{fmt, ops::{Index, IndexMut}};

#[derive(Clone, Copy, Debug, Hash, PartialEq)]
pub struct Color {
    pub red: u8,
    pub green: u8,
    pub blue: u8
}

#[macro_export]
macro_rules! color {
    ($r:expr,$g:expr,$b:expr) => {
        Color {
            red: $r,
            green: $g,
            blue: $b
        }
    };
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {} {}", self.red, self.green, self.blue)
    }
}

pub mod color_constants {
    use crate::utils::Color;

    pub const BLACK: Color = color!(0, 0, 0);
    pub const RED: Color = color!(255, 0, 0);
    pub const GREEN: Color = color!(0, 255, 0);
    pub const BLUE: Color = color!(0, 0, 255);
    pub const WHITE: Color = color!(255, 255, 255);
}

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
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "P3\n")?;
        write!(f, "{} {}\n", self.width, self.height)?;
        write!(f, "255\n")?;
        
        self.data
            .iter()
            .try_for_each(|color| write!(f, "{} ", color))?;

        Ok(())
    }
}