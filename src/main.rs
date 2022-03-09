use std::fs::File;

use graphics_year2::Parser;
fn main() {
    let mut p: Parser = Default::default();
    p.parse(File::open("script").expect("File read failed"));
}