use std::{fs, io::{BufReader, BufRead}};

use crate::{image::Image, matrix::EdgeMatrix, transform::{Transformer, Axis}, color::color_constants, curves::Circle};

#[derive(Clone, Debug)]
pub struct Parser {
    image: Box<Image<500, 500>>,
    e: EdgeMatrix,
    t: Transformer
}

fn consume_word(word_iter: &mut impl Iterator<Item = String>) -> String {
    word_iter.next().unwrap_or_else(|| panic!("Missing arguments"))
}
fn consume_float(word_iter: &mut impl Iterator<Item = String>) -> f64 {
    consume_word(word_iter).parse().expect("Failed to parse float")
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
                        (consume_float(&mut word_iter), consume_float(&mut word_iter), consume_float(&mut word_iter)),
                        (consume_float(&mut word_iter), consume_float(&mut word_iter), consume_float(&mut word_iter))
                    ),
                "ident" => self.t.reset(),
                "scale" => self.t.scale(consume_float(&mut word_iter), consume_float(&mut word_iter), consume_float(&mut word_iter)),
                "move" => self.t.translate(consume_float(&mut word_iter), consume_float(&mut word_iter), consume_float(&mut word_iter)),
                "rotate" => self.t.rotate(
                    match consume_word(&mut word_iter).as_str() {
                        "x" => Axis::X,
                        "y" => Axis::Y,
                        "z" => Axis::Z,
                        _ => panic!("Unrecognized axis; use x/y/z.")
                    }, 
                    consume_float(&mut word_iter) * std::f64::consts::PI / 180.0
                ),
                "apply" => self.e = self.t.apply(&self.e),
                "display" => {
                    self.image.clear();
                    self.image.draw_matrix(&self.e, color_constants::WHITE);
                    self.image.display().expect("Image display failed");
                },
                "save" => {
                    match consume_word(&mut word_iter).rsplit_once(".") {
                        Some((prefix, "png")) => {
                            self.image.clear();
                            self.image.draw_matrix(&self.e, color_constants::WHITE);
                            self.image.save_name(prefix).expect("Failed image write")
                        },
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
