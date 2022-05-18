use std::{error::Error, fs, io::Read};
use pest::{Parser};
use pest_derive::Parser;

use crate::{Image, matrix::{EdgeMatrix, PolygonMatrix}, Transformer, Axis, color::color_constants, /*curves::{Circle, Parametric, Hermite, Bezier},*/ shapes3d::*, TStack};

#[derive(Clone, Debug, Parser)]
#[grammar = "parser/grammar.pest"]
pub struct MDLParser {
    image: Box<Image<500, 500>>,
    t: TStack
}

impl MDLParser {
    pub fn parse_file(&mut self, mut file: fs::File) -> Result<(), Box<dyn Error>> {
        let mut program = String::new();
        file.read_to_string(&mut program)?;
        self.parse_str(program.as_str())
    }

    pub fn parse_str(&mut self, program: &str) -> Result<(), Box<dyn Error>> {
        let mut pairs = MDLParser::parse(Rule::MDL, program)?;

        pairs.next().unwrap().into_inner().map(|command| -> Result<(), Box<dyn Error>> {
            match command.as_rule() {
                Rule::LINE_DDDDDD => {
                    let mut args = command.into_inner().skip(1);
                    let mut e: EdgeMatrix = Default::default();
                    e.add_edge(
                        (
                                args.next().unwrap().as_str().parse::<f64>()?,
                                args.next().unwrap().as_str().parse::<f64>()?,
                                args.next().unwrap().as_str().parse::<f64>()?
                            ),
                            (
                                args.next().unwrap().as_str().parse::<f64>()?,
                                args.next().unwrap().as_str().parse::<f64>()?,
                                args.next().unwrap().as_str().parse::<f64>()?
                            )
                    );
                    e = self.t.top().apply_edges(&e);
                    self.image.draw_matrix(&mut e, color_constants::WHITE);
                    Ok(())
                }
                /*
                Rule::CIRCLE_DDD => {
                    let mut args = command.into_inner().skip(1);
                    let mut e: EdgeMatrix = Default::default();

                    let center = 
                        (
                            args.next().unwrap().as_str().parse::<f64>()?,
                            args.next().unwrap().as_str().parse::<f64>()?,
                            args.next().unwrap().as_str().parse::<f64>()?
                        );
                    let radius = args.next().unwrap().as_str().parse::<f64>()?;
                    
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
                    Ok(())
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
                },*/
                Rule::BOX_DDDDDD => {
                    let mut args = command.into_inner().skip(1);
                    let mut p: PolygonMatrix = Default::default();

                    let ltf = 
                        (
                            args.next().unwrap().as_str().parse::<f64>()?, 
                            args.next().unwrap().as_str().parse::<f64>()?,
                            args.next().unwrap().as_str().parse::<f64>()?
                        );
                    let width = args.next().unwrap().as_str().parse::<f64>()?;
                    let height = args.next().unwrap().as_str().parse::<f64>()?;
                    let depth = args.next().unwrap().as_str().parse::<f64>()?;

                    let cube = Cube::new(ltf, width, height, depth);
                    cube.add_to_matrix(&mut p);

                    p = self.t.top().apply_poly(&p);
                    self.image.draw_polygons(&mut p);
                    Ok(())
                },
                Rule::SPHERE_DDDD => {
                    let mut args = command.into_inner().skip(1);
                    let mut p: PolygonMatrix = Default::default();

                    let center = 
                        (
                            args.next().unwrap().as_str().parse::<f64>()?, 
                            args.next().unwrap().as_str().parse::<f64>()?, 
                            args.next().unwrap().as_str().parse::<f64>()?
                        );
                    let radius = args.next().unwrap().as_str().parse::<f64>()?;

                    const SIDE_LENGTH: f64 = 1.0;
                    let point_count = std::f64::consts::TAU * radius / SIDE_LENGTH;

                    let sphere = Sphere::new(radius, center);
                    sphere.add_to_matrix(&mut p, point_count as usize);

                    p = self.t.top().apply_poly(&p);
                    self.image.draw_polygons(&mut p);
                    Ok(())
                },
                Rule::TORUS_DDDDD => {
                    let mut args = command.into_inner().skip(1);
                    let mut p: PolygonMatrix = Default::default();

                    let center = 
                        (
                            args.next().unwrap().as_str().parse::<f64>()?, 
                            args.next().unwrap().as_str().parse::<f64>()?, 
                            args.next().unwrap().as_str().parse::<f64>()?
                        );
                    let thickness = args.next().unwrap().as_str().parse::<f64>()?;
                    let radius = args.next().unwrap().as_str().parse::<f64>()?;

                    const SIDE_LENGTH: f64 = 1.0;
                    let ring_count = std::f64::consts::TAU * radius / SIDE_LENGTH;
                    let cir_count = std::f64::consts::TAU * thickness / SIDE_LENGTH;

                    let torus = Torus::new(thickness, radius, center);
                    torus.add_to_matrix(&mut p, ring_count as usize, cir_count as usize);

                    p = self.t.top().apply_poly(&p);
                    self.image.draw_polygons(&mut p);
                    Ok(())
                },
                Rule::SCALE_DDD => {
                    let mut args = command.into_inner().skip(1);
                    let mut scale_transform: Transformer = Default::default();
                    scale_transform.scale(
                        args.next().unwrap().as_str().parse::<f64>()?, 
                        args.next().unwrap().as_str().parse::<f64>()?, 
                        args.next().unwrap().as_str().parse::<f64>()?
                    );
                    self.t.top().compose(&scale_transform);
                    Ok(())
                },
                Rule::MOVE_DDD => {
                    let mut args = command.into_inner().skip(1);
                    let mut move_transform: Transformer = Default::default();
                    move_transform.translate(
                        args.next().unwrap().as_str().parse::<f64>()?, 
                        args.next().unwrap().as_str().parse::<f64>()?, 
                        args.next().unwrap().as_str().parse::<f64>()?
                    );
                    self.t.top().compose(&move_transform);
                    Ok(())
                },
                Rule::ROTATE_SD => {
                    let mut args = command.into_inner().skip(1);
                    let mut rotate_transform: Transformer = Default::default();
                    rotate_transform.rotate(
                        match args.next().unwrap().as_str() {
                            "x" => Axis::X,
                            "y" => Axis::Y,
                            "z" => Axis::Z,
                            _ => panic!("Unrecognized axis; use x/y/z.")
                        }, 
                        args.next().unwrap().as_str().parse::<f64>()? * std::f64::consts::PI / 180.0
                    );
                    self.t.top().compose(&rotate_transform);
                    Ok(())
                },
                Rule::TPUSH => {
                    self.t.push_copy();
                    Ok(())
                },
                Rule::TPOP => {
                    self.t.pop();
                    Ok(())
                },
                Rule::CLEAR => {
                    self.image = Box::new(Image::new_flip("result".to_string(), true));
                    // self.t = Default::default();
                    Ok(())
                },
                Rule::DISPLAY => {
                    if let None = self.image.display().ok() {
                        eprintln!("Could not display image.");
                    }
                    Ok(())
                },
                Rule::SAVE => {
                    let mut args = command.into_inner().skip(1);
                    let filename = args.next().unwrap().as_str();
                    match filename.rsplit_once(".") {
                        Some((prefix, "png")) => {
                            if let None = self.image.save_name(prefix).ok() {
                                eprintln!("Could not save {}.png", prefix);
                            }
                        },
                        Some((_, _)) => panic!("File extension not png"),
                        None => panic!("No file extension"),
                    };
                    Ok(())
                }
                _ => panic!("{} is unimplemented!", command.as_str())
            }
        })
        .collect()
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
