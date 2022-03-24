use std::fs::File;

use crate::Parser;
#[test]
fn main() {
    let mut p: Parser = Default::default();
    p.parse(File::open("src/tests/hanoi").expect("File read failed"));
}