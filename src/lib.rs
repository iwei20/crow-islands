pub mod color;
pub use color::Color;

mod image;
pub use image::Image;

pub mod matrix;

mod transform;
pub use transform::Transformer;
pub use transform::Axis;

pub mod curves;
pub mod shapes3d;

mod vector3d;
pub use vector3d::Vector3D;

mod parser;
pub use parser::Parser;
#[cfg(test)]
mod tests;
