use std::{fmt, ops::{Add, Mul, AddAssign}, cmp};

use rand::{thread_rng, Rng};

#[derive(Clone, Copy, Debug, Hash, PartialEq)]
pub struct Color {
    pub red: u8,
    pub green: u8,
    pub blue: u8
}

impl Color {
    pub fn new(red: u8, green: u8, blue: u8) -> Self {
        Self {
            red,
            green,
            blue
        }
    }
    pub fn rand() -> Self {
        Self {
            red: thread_rng().gen::<u8>(),
            green: thread_rng().gen::<u8>(),
            blue: thread_rng().gen::<u8>()
        }
    }
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

#[macro_export]
macro_rules! impl_add {
    ($lhs:ty, $rhs:ty) => {
        impl Add<$rhs> for $lhs {
            type Output = Color;

            fn add(self, rhs: $rhs) -> Self::Output {
                Color { 
                    red: cmp::min(self.red.saturating_add(rhs.red), 255), 
                    green: cmp::min(self.green.saturating_add(rhs.green), 255), 
                    blue: cmp::min(self.blue.saturating_add(rhs.blue), 255)
                }
            }
        }
    };
}

impl_add!(Color, Color);
impl_add!(Color, &Color);
impl_add!(Color, &mut Color);
impl_add!(&Color, Color);
impl_add!(&Color, &Color);
impl_add!(&Color, &mut Color);
impl_add!(&mut Color, Color);
impl_add!(&mut Color, &Color);
impl_add!(&mut Color, &mut Color);

impl AddAssign for Color {
    fn add_assign(&mut self, rhs: Self) {
        let result = self + rhs;
        self = self + rhs;
    }
}

impl Mul<(f64, f64, f64)> for Color {
    type Output = Color;

    fn mul(self, rhs: (f64, f64, f64)) -> Self::Output {
        Self { 
            red: cmp::min((self.red as f64 * rhs.0) as u8, 255), 
            green: cmp::min((self.green as f64 * rhs.0) as u8, 255), 
            blue: cmp::min((self.blue as f64 * rhs.0) as u8, 255)
        }
    }
}

impl Mul<(f64, f64, f64)> for &Color {
    type Output = Color;

    fn mul(self, rhs: (f64, f64, f64)) -> Self::Output {
        Color { 
            red: cmp::min((self.red as f64 * rhs.0) as u8, 255), 
            green: cmp::min((self.green as f64 * rhs.0) as u8, 255), 
            blue: cmp::min((self.blue as f64 * rhs.0) as u8, 255)
        }
    }
}

impl Mul<Color> for (f64, f64, f64) {
    type Output = Color;

    fn mul(self, rhs: Color) -> Self::Output {
        rhs * self
    }
}

impl Mul<&Color> for (f64, f64, f64) {
    type Output = Color;

    fn mul(self, rhs: &Color) -> Self::Output {
        rhs * self
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