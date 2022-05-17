use std::fs::File;

use crate::MDLParser;
#[test]
fn main() {
    let mut p: MDLParser = Default::default();
    p.parse(File::open("src/tests/vase").expect("File read failed"));
}