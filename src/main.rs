use std::{fs::File, env};

use graphics_year2::MDLParser;
fn main() {
    let args: Vec<String> = env::args().collect();
    let mut p: MDLParser = Default::default();
    if args.len() <= 1 {
        p.parse_file(File::open("script").expect("File read failed"));
    } else {
        p.parse_file(File::open(&args[1]).expect("File read failed"));
    }
}