
use std::{fmt, iter, ops::IndexMut};

#[derive(Copy, Debug, Hash, PartialEq)]
pub struct Color {
    pub red: u8,
    pub green: u8,
    pub blue: u8
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {} {}", self.red, self.green, self.blue)
    }
}

#[derive(Clone, Debug)]
pub struct Image {
    pub width: usize,
    pub height: usize,
    data: Vec<Color>
}

impl IndexMut<usize> for Image {
    fn index(&self, index: usize) -> &mut [Color] {
        &self.data[index * self.width .. (index+1) * self.width]
    }
}

impl fmt::Display for Image {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "P3\n")?;
        write!(f, "{} {}\n", self.width, self.height)?;
        write!(f, "255\n")?;
        
        &self.data
            .iter()
            .for_each(|color| write!(f, "{} ", color)?);

        Ok(())
    }
}