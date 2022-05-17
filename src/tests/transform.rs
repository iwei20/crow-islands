use std::fs::{self, File};

use crate::MDLParser;

#[test]
fn clock() {
    fs::create_dir_all("clockframes").expect("Clock directory creation failed");
    for i in 0..60 {
        let mut p: MDLParser = Default::default();
        p.parse_file(File::open(format!("src/tests/clockscripts/clock{:0>2}", i).as_str()).expect("File read failed"))
    }
}