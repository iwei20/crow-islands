use std::{fs, io::{BufReader, BufRead}};

use crate::{image::Image, matrix::{EdgeMatrix, PolygonMatrix}, transform::{Transformer, Axis}, color::color_constants, curves::{Circle, Parametric, Hermite, Bezier}, shapes3d::{add_box, add_points, generate_sphere, generate_torus, add_sphere}};

#[derive(Clone, Debug)]
pub struct Parser {
    image: Box<Image<500, 500>>,
    p: PolygonMatrix,
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
                "circle" => {
                    let center = (consume_float(&mut word_iter), consume_float(&mut word_iter), consume_float(&mut word_iter));
                    let radius = consume_float(&mut word_iter);
                    
                    const SIDE_LENGTH: f64 = 5.0;
                    let point_count = std::f64::consts::TAU * radius / SIDE_LENGTH;
                    let circle = Circle::new(radius, center);
                    circle
                        .points(point_count as usize)
                        .windows(2)
                        .for_each(|window| {
                            self.e.add_edge(window[0], window[1])
                        });
                },
                "hermite" => {
                    let p0 = (consume_float(&mut word_iter), consume_float(&mut word_iter));
                    let p1 = (consume_float(&mut word_iter), consume_float(&mut word_iter));
                    let r0 = (consume_float(&mut word_iter), consume_float(&mut word_iter));
                    let r1 = (consume_float(&mut word_iter), consume_float(&mut word_iter));
                    let hermite = Hermite::new(p0, p1, r0, r1);
                    hermite
                        .points(50)
                        .windows(2)
                        .for_each(|window| {
                            self.e.add_edge(window[0], window[1])
                        });
                },
                "bezier" => {
                    let p0 = (consume_float(&mut word_iter), consume_float(&mut word_iter));
                    let p1 = (consume_float(&mut word_iter), consume_float(&mut word_iter));
                    let p2 = (consume_float(&mut word_iter), consume_float(&mut word_iter));
                    let p3 = (consume_float(&mut word_iter), consume_float(&mut word_iter));
                    let bezier = Bezier::new(p0, p1, p2, p3);
                    bezier
                        .points(50)
                        .windows(2)
                        .for_each(|window| {
                            self.e.add_edge(window[0], window[1])
                        });
                },
                "box" => {
                    let ltf = (consume_float(&mut word_iter), consume_float(&mut word_iter), consume_float(&mut word_iter));
                    let width = consume_float(&mut word_iter);
                    let height = consume_float(&mut word_iter);
                    let depth = consume_float(&mut word_iter);
                    add_box(&mut self.p, ltf, width, height, depth);
                },
                "sphere" => {
                    let center = (consume_float(&mut word_iter), consume_float(&mut word_iter), consume_float(&mut word_iter));
                    let radius = consume_float(&mut word_iter);

                    const SIDE_LENGTH: f64 = 50.0;
                    let point_count = std::f64::consts::TAU * radius / SIDE_LENGTH;
                    add_sphere(&mut self.p, &generate_sphere(radius, center, point_count as usize), point_count as usize);
                },
                "torus" => {
                    let center = (consume_float(&mut word_iter), consume_float(&mut word_iter), consume_float(&mut word_iter));
                    let thickness = consume_float(&mut word_iter);
                    let radius = consume_float(&mut word_iter);

                    const SIDE_LENGTH: f64 = 5.0;
                    let ring_count = std::f64::consts::TAU * radius / SIDE_LENGTH;
                    let cir_count = std::f64::consts::TAU * thickness / SIDE_LENGTH;
                    add_points(&mut self.e, &generate_torus(thickness, radius, center, ring_count as usize, cir_count as usize));
                },
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
                "apply" => {
                    self.e = self.t.apply_edges(&self.e);
                    self.p = self.t.apply_poly(&self.p);
                },
                "display" => {
                    self.image.clear();
                    self.image.draw_matrix(&self.e, color_constants::WHITE);
                    self.image.draw_polygons(&self.p, color_constants::WHITE);
                    self.image.display().expect("Image display failed");
                },
                "clear" => {
                    self.e = Default::default();
                    self.p = Default::default();
                },
                "save" => {
                    match consume_word(&mut word_iter).rsplit_once(".") {
                        Some((prefix, "png")) => {
                            self.image.clear();
                            self.image.draw_matrix(&self.e, color_constants::WHITE);
                            self.image.draw_polygons(&self.p, color_constants::WHITE);
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
            p: Default::default(),
            e: Default::default(), 
            t: Default::default() 
        }
    }
}
