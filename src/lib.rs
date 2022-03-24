pub mod color;
pub mod image;
pub mod matrix;
pub mod transform;
pub mod curves;
pub mod shapes3d;

mod parser;
pub use parser::Parser;
#[cfg(test)]
mod tests;
