use std::fs::File;

use crate::MDLParser;
#[test]
fn main() {
    let mut p: MDLParser = Default::default();
    p.parse(File::open("src/tests/torusfractal").expect("File read failed"));
}