use std::fmt;

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
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {} {}", self.red, self.green, self.blue)
    }
}

impl Default for Color {
    fn default() -> Self {
        color_constants::BLACK
    }
}

pub mod color_constants {
    use super::Color;
    
    pub const BLACK: Color = color!(0, 0, 0);
    pub const RED: Color = color!(255, 0, 0);
    pub const GREEN: Color = color!(0, 255, 0);
    pub const BLUE: Color = color!(0, 0, 255);
    pub const PURPLE: Color = color!(255, 0, 255);
    pub const CYAN: Color = color!(0, 255, 255);
    pub const YELLOW: Color = color!(255, 255, 0);
    pub const WHITE: Color = color!(255, 255, 255);
}