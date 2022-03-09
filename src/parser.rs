use std::{fs, io::{BufReader, BufRead}};

use crate::{image::Image, matrix::EdgeMatrix, transform::{Transformer, Axis}, color::color_constants};

#[derive(Clone, Debug)]
pub struct Parser {
    image: Box<Image<500, 500>>,
    e: EdgeMatrix,
    t: Transformer
}

impl Parser {
    pub fn parse(&mut self, file: fs::File) {
        let reader = BufReader::new(file);

        let mut word_iter = reader
            .lines()
            .map(|line| 
                line.as_ref()
                    .expect("Bufread line failed")
                    .split_once('#')
                    .unwrap_or_else(|| (line.as_ref().expect("BufReader line failed").as_str(), ""))
                    .0 // Eliminate comments
                    .split_whitespace()
                    .map(|slice| slice.to_string())// Create whitespace
                    .collect::<Vec<_>>() 
            )
            .flatten();
        
        while let Some(word) = word_iter.next() {
            match word.as_str() {
                "line" => 
                    self.e.add_edge(
                        (word_iter.next().unwrap_or_else(|| panic!("Missing arguments")).parse().expect("Failed to parse float for line"), word_iter.next().unwrap_or_else(|| panic!("Missing arguments")).parse().expect("Failed to parse float for line"), word_iter.next().unwrap_or_else(|| panic!("Missing arguments")).parse().expect("Failed to parse float for line")),
                        (word_iter.next().unwrap_or_else(|| panic!("Missing arguments")).parse().expect("Failed to parse float for line"), word_iter.next().unwrap_or_else(|| panic!("Missing arguments")).parse().expect("Failed to parse float for line"), word_iter.next().unwrap_or_else(|| panic!("Missing arguments")).parse().expect("Failed to parse float for line"))
                    ),
                "ident" => self.t.reset(),
                "scale" => self.t.scale(word_iter.next().unwrap_or_else(|| panic!("Missing arguments")).parse().expect("Failed to parse float for line"), word_iter.next().unwrap_or_else(|| panic!("Missing arguments")).parse().expect("Failed to parse float for line"), word_iter.next().unwrap_or_else(|| panic!("Missing arguments")).parse().expect("Failed to parse float for line")),
                "move" => self.t.translate(word_iter.next().unwrap_or_else(|| panic!("Missing arguments")).parse().expect("Failed to parse float for line"), word_iter.next().unwrap_or_else(|| panic!("Missing arguments")).parse().expect("Failed to parse float for line"), word_iter.next().unwrap_or_else(|| panic!("Missing arguments")).parse().expect("Failed to parse float for line")),
                "rotate" => self.t.rotate(
                    match word_iter.next().unwrap_or_else(|| panic!("Missing arguments")).as_str() {
                        "x" => Axis::X,
                        "y" => Axis::Y,
                        "z" => Axis::Z,
                        _ => panic!("Unrecognized axis; use x/y/z.")
                    }, 
                    word_iter.next().unwrap_or_else(|| panic!("Missing arguments")).parse::<f64>().expect("Failed to parse float for line") * std::f64::consts::PI / 180.0
                ),
                "apply" => self.e = self.t.apply(&self.e),
                "display" => {
                    self.image.clear();
                    self.image.draw_matrix(&self.e, color_constants::WHITE);
                    self.image.display().expect("Image display failed");
                },
                "save" => {
                    match word_iter.next().unwrap_or_else(|| panic!("Missing arguments")).rsplit_once(".") {
                        Some((prefix, "png")) => self.image.save_name(prefix).expect("Failed image write"),
                        Some((_, _)) => panic!("File extension not png"),
                        None => panic!("No file extension"),
                    };
                }
                _ => panic!("{} not recognized as a command!", word)
            }
        }
    }
}

impl Default for Parser {
    fn default() -> Self {
        Self { 
            image: Box::new(Image::new_flip("result".to_string(), true)), 
            e: Default::default(), 
            t: Default::default() 
        }
    }
}
