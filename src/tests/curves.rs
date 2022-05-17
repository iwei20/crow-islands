use std::fs::File;

use crate::MDLParser;
#[test]
fn main() {
    let mut p: MDLParser = Default::default();
    p.parse_file(File::open("src/tests/vase").expect("File read failed"));
}