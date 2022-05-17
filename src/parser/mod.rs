use std::{fs, io::{BufReader, BufRead, Read}};

use pest::Parser;
use pest_derive::Parser;

use crate::{Image, matrix::{EdgeMatrix, PolygonMatrix}, Transformer, Axis, color::color_constants, curves::{Circle, Parametric, Hermite, Bezier}, shapes3d::*, TStack};

#[derive(Clone, Debug, Parser)]
#[grammar = "parser/grammar.pest"]
pub struct MDLParser {
    image: Box<Image<500, 500>>,
    t: TStack
}

fn consume_word(word_iter: &mut impl Iterator<Item = String>) -> String {
    word_iter.next().unwrap_or_else(|| panic!("Missing arguments"))
}
fn consume_float(word_iter: &mut impl Iterator<Item = String>) -> f64 {
    consume_word(word_iter).parse().expect("Failed to parse float")
}

impl MDLParser {
    pub fn parse_str(&mut self, program: &str) {
        let pairs = MDLParser::parse(Rule::MDL, program);

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
                "line" => {
                        let mut e: EdgeMatrix = Default::default();
                        e.add_edge(
                            (consume_float(&mut word_iter), consume_float(&mut word_iter), consume_float(&mut word_iter)),
                            (consume_float(&mut word_iter), consume_float(&mut word_iter), consume_float(&mut word_iter))
                        );
                        e = self.t.top().apply_edges(&e);
                        self.image.draw_matrix(&mut e, color_constants::WHITE);
                    }
                "circle" => {
                    let mut e: EdgeMatrix = Default::default();

                    let center = (consume_float(&mut word_iter), consume_float(&mut word_iter), consume_float(&mut word_iter));
                    let radius = consume_float(&mut word_iter);
                    
                    const SIDE_LENGTH: f64 = 5.0;
                    let point_count = std::f64::consts::TAU * radius / SIDE_LENGTH;
                    let circle = Circle::new(radius, center);
                    circle
                        .points(point_count as usize)
                        .windows(2)
                        .for_each(|window| {
                            e.add_edge(window[0], window[1])
                        });

                    e = self.t.top().apply_edges(&e);
                    self.image.draw_matrix(&mut e, color_constants::WHITE);
                },
                "hermite" => {
                    let mut e: EdgeMatrix = Default::default();

                    let p0 = (consume_float(&mut word_iter), consume_float(&mut word_iter));
                    let p1 = (consume_float(&mut word_iter), consume_float(&mut word_iter));
                    let r0 = (consume_float(&mut word_iter), consume_float(&mut word_iter));
                    let r1 = (consume_float(&mut word_iter), consume_float(&mut word_iter));
                    let hermite = Hermite::new(p0, p1, r0, r1);
                    hermite
                        .points(50)
                        .windows(2)
                        .for_each(|window| {
                            e.add_edge(window[0], window[1])
                        });
                    e = self.t.top().apply_edges(&e);
                    self.image.draw_matrix(&mut e, color_constants::WHITE);
                },
                "bezier" => {
                    let mut e: EdgeMatrix = Default::default();

                    let p0 = (consume_float(&mut word_iter), consume_float(&mut word_iter));
                    let p1 = (consume_float(&mut word_iter), consume_float(&mut word_iter));
                    let p2 = (consume_float(&mut word_iter), consume_float(&mut word_iter));
                    let p3 = (consume_float(&mut word_iter), consume_float(&mut word_iter));
                    let bezier = Bezier::new(p0, p1, p2, p3);
                    bezier
                        .points(50)
                        .windows(2)
                        .for_each(|window| {
                            e.add_edge(window[0], window[1])
                        });
                    e = self.t.top().apply_edges(&e);
                    self.image.draw_matrix(&mut e, color_constants::WHITE);
                },
                "box" => {
                    let mut p: PolygonMatrix = Default::default();

                    let ltf = (consume_float(&mut word_iter), consume_float(&mut word_iter), consume_float(&mut word_iter));
                    let width = consume_float(&mut word_iter);
                    let height = consume_float(&mut word_iter);
                    let depth = consume_float(&mut word_iter);

                    let cube = Cube::new(ltf, width, height, depth);
                    cube.add_to_matrix(&mut p);

                    p = self.t.top().apply_poly(&p);
                    self.image.draw_polygons(&mut p);
                },
                "sphere" => {
                    let mut p: PolygonMatrix = Default::default();

                    let center = (consume_float(&mut word_iter), consume_float(&mut word_iter), consume_float(&mut word_iter));
                    let radius = consume_float(&mut word_iter);

                    const SIDE_LENGTH: f64 = 1.0;
                    let point_count = std::f64::consts::TAU * radius / SIDE_LENGTH;

                    let sphere = Sphere::new(radius, center);
                    sphere.add_to_matrix(&mut p, point_count as usize);

                    p = self.t.top().apply_poly(&p);
                    self.image.draw_polygons(&mut p);
                },
                "torus" => {
                    let mut p: PolygonMatrix = Default::default();

                    let center = (consume_float(&mut word_iter), consume_float(&mut word_iter), consume_float(&mut word_iter));
                    let thickness = consume_float(&mut word_iter);
                    let radius = consume_float(&mut word_iter);

                    const SIDE_LENGTH: f64 = 1.0;
                    let ring_count = std::f64::consts::TAU * radius / SIDE_LENGTH;
                    let cir_count = std::f64::consts::TAU * thickness / SIDE_LENGTH;

                    let torus = Torus::new(thickness, radius, center);
                    torus.add_to_matrix(&mut p, ring_count as usize, cir_count as usize);

                    p = self.t.top().apply_poly(&p);
                    self.image.draw_polygons(&mut p);
                },
                "scale" => {
                    let mut scale_transform: Transformer = Default::default();
                    scale_transform.scale(consume_float(&mut word_iter), consume_float(&mut word_iter), consume_float(&mut word_iter));
                    self.t.top().compose(&scale_transform);
                },
                "move" => {
                    let mut move_transform: Transformer = Default::default();
                    move_transform.translate(consume_float(&mut word_iter), consume_float(&mut word_iter), consume_float(&mut word_iter));
                    self.t.top().compose(&move_transform);
                },
                "rotate" => {
                    let mut rotate_transform: Transformer = Default::default();
                    rotate_transform.rotate(
                        match consume_word(&mut word_iter).as_str() {
                            "x" => Axis::X,
                            "y" => Axis::Y,
                            "z" => Axis::Z,
                            _ => panic!("Unrecognized axis; use x/y/z.")
                        }, 
                        consume_float(&mut word_iter) * std::f64::consts::PI / 180.0
                    );
                    self.t.top().compose(&rotate_transform);
                },
                "push" => self.t.push_copy(),
                "pop" => self.t.pop(),
                "clear" => {
                    self.image = Box::new(Image::new_flip("result".to_string(), true));
                    // self.t = Default::default();
                },
                "display" => {
                    if let None = self.image.display().ok() {
                        eprintln!("Could not display image.");
                    }
                },
                "save" => {
                    match consume_word(&mut word_iter).rsplit_once(".") {
                        Some((prefix, "png")) => {
                            if let None = self.image.save_name(prefix).ok() {
                                eprintln!("Could not save {}.png", prefix);
                            }
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

impl Default for MDLParser {
    fn default() -> Self {
        Self { 
            image: Box::new(Image::new_flip("result".to_string(), true)), 
            t: Default::default() 
        }
    }
}
