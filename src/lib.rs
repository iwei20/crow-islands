pub mod color;
pub mod image;
pub mod matrix;
pub mod transform;

mod parser;
pub use parser::Parser;
#[cfg(test)]
mod tests;