use std::{error::Error, fs, io::Read, collections::HashMap, num::ParseFloatError};
use pest::{Parser, iterators::Pair};
use pest_derive::Parser;

use crate::{Image, matrix::{EdgeMatrix, PolygonMatrix}, Transformer, Axis, color::color_constants, /*curves::{Circle, Parametric, Hermite, Bezier},*/ shapes3d::*, TStack, lighter::LightingConfig};

#[derive(Clone, Debug, Parser)]
#[grammar = "parser/grammar.pest"]
pub struct MDLParser {
    image: Box<Image<500, 500>>,
    t: TStack,
    constants: HashMap<String, LightingConfig>
}

const DEFAULT_LIGHTING_CONFIG: LightingConfig = LightingConfig {
    ka: (0.1, 0.1, 0.1),
    ks: (0.5, 0.5, 0.5),
    kd: (0.5, 0.5, 0.5)
};
const SIDE_LENGTH: f64 = 2.0;

impl MDLParser {
    fn next<'i>(args: &mut impl Iterator<Item = Pair<'i, Rule>>) -> &'i str {
        args.next().unwrap().as_str()
    }

    fn next_f64<'i>(args: &mut impl Iterator<Item = Pair<'i, Rule>>) -> Result<f64, ParseFloatError> {
        MDLParser::next(args).parse::<f64>()
    }

    pub fn parse_file(&mut self, mut file: fs::File) -> Result<(), Box<dyn Error>> {
        let mut program = String::new();
        file.read_to_string(&mut program)?;
        self.parse_str(program.as_str())
    }

    pub fn parse_str(&mut self, program: &str) -> Result<(), Box<dyn Error>> {
        let mut pairs = MDLParser::parse(Rule::MDL, program)?;
        //println!("{:?}", pairs);
        pairs.next().unwrap().into_inner().map(|command| -> Result<(), Box<dyn Error>> {
            match command.as_rule() {
                Rule::CONSTANTS_SHORT_ARGS => {
                    let mut args = command.into_inner().skip(1);
                    let name = MDLParser::next(&mut args).to_string();
                    let reds = (
                        MDLParser::next_f64(&mut args)?,
                        MDLParser::next_f64(&mut args)?,
                        MDLParser::next_f64(&mut args)?
                    );
                    let greens = (
                        MDLParser::next_f64(&mut args)?,
                        MDLParser::next_f64(&mut args)?,
                        MDLParser::next_f64(&mut args)?
                    );
                    let blues = (
                        MDLParser::next_f64(&mut args)?,
                        MDLParser::next_f64(&mut args)?,
                        MDLParser::next_f64(&mut args)?
                    );
                    self.constants.insert(name, LightingConfig {
                        ka: (reds.0, greens.0, blues.0),
                        ks: (reds.1, greens.1, blues.1),
                        kd: (reds.2, greens.2, blues.2)
                    });
                    Ok(())
                },
                Rule::LINE_DDDDDD => {
                    let mut args = command.into_inner().skip(1);
                    let mut e: EdgeMatrix = Default::default();
                    e.add_edge(
                        (
                                MDLParser::next_f64(&mut args)?,
                                MDLParser::next_f64(&mut args)?,
                                MDLParser::next_f64(&mut args)?
                            ),
                            (
                                MDLParser::next_f64(&mut args)?,
                                MDLParser::next_f64(&mut args)?,
                                MDLParser::next_f64(&mut args)?
                            )
                    );
                    e = self.t.top().apply_edges(&e);
                    self.image.draw_matrix(&mut e, color_constants::WHITE);
                    Ok(())
                },
                /*
                Rule::CIRCLE_DDD => {
                    let mut args = command.into_inner().skip(1);
                    let mut e: EdgeMatrix = Default::default();

                    let center = 
                        (
                            MDLParser::next_f64(&mut args)?,
                            MDLParser::next_f64(&mut args)?,
                            MDLParser::next_f64(&mut args)?
                        );
                    let radius = MDLParser::next_f64(&mut args)?;
                    
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
                            MDLParser::next_f64(&mut args)?, 
                            MDLParser::next_f64(&mut args)?,
                            MDLParser::next_f64(&mut args)?
                        );
                    let width = MDLParser::next_f64(&mut args)?;
                    let height = MDLParser::next_f64(&mut args)?;
                    let depth = MDLParser::next_f64(&mut args)?;

                    let cube = Cube::new(ltf, width, height, depth);
                    cube.add_to_matrix(&mut p);

                    p = self.t.top().apply_poly(&p);
                    self.image.draw_polygons(&mut p, &DEFAULT_LIGHTING_CONFIG);
                    Ok(())
                },
                Rule::BOX_SDDDDDD => {
                    let mut args = command.into_inner().skip(1);
                    let mut p: PolygonMatrix = Default::default();

                    let constant = MDLParser::next(&mut args);
                    let ltf = 
                        (
                            MDLParser::next_f64(&mut args)?, 
                            MDLParser::next_f64(&mut args)?,
                            MDLParser::next_f64(&mut args)?
                        );
                    let width = MDLParser::next_f64(&mut args)?;
                    let height = MDLParser::next_f64(&mut args)?;
                    let depth = MDLParser::next_f64(&mut args)?;

                    let cube = Cube::new(ltf, width, height, depth);
                    cube.add_to_matrix(&mut p);

                    p = self.t.top().apply_poly(&p);
                    self.image.draw_polygons(&mut p, &self.constants[constant]);
                    Ok(())
                },
                Rule::SPHERE_DDDD => {
                    let mut args = command.into_inner().skip(1);
                    let mut p: PolygonMatrix = Default::default();
                    
                    let center = 
                        (
                            MDLParser::next_f64(&mut args)?, 
                            MDLParser::next_f64(&mut args)?, 
                            MDLParser::next_f64(&mut args)?
                        );
                    let radius = MDLParser::next_f64(&mut args)?;

                    let point_count = std::f64::consts::TAU * radius / SIDE_LENGTH;

                    let sphere = Sphere::new(radius, center);
                    sphere.add_to_matrix(&mut p, point_count as usize);

                    p = self.t.top().apply_poly(&p);
                    self.image.draw_polygons(&mut p, &DEFAULT_LIGHTING_CONFIG);
                    Ok(())
                },
                Rule::SPHERE_SDDDD => {
                    let mut args = command.into_inner().skip(1);
                    let mut p: PolygonMatrix = Default::default();

                    let constant = MDLParser::next(&mut args);
                    let center = 
                        (
                            MDLParser::next_f64(&mut args)?, 
                            MDLParser::next_f64(&mut args)?, 
                            MDLParser::next_f64(&mut args)?
                        );
                    let radius = MDLParser::next_f64(&mut args)?;

                    let point_count = std::f64::consts::TAU * radius / SIDE_LENGTH;

                    let sphere = Sphere::new(radius, center);
                    sphere.add_to_matrix(&mut p, point_count as usize);

                    p = self.t.top().apply_poly(&p);
                    self.image.draw_polygons(&mut p, &self.constants[constant]);
                    Ok(())
                },
                Rule::TORUS_DDDDD => {
                    let mut args = command.into_inner().skip(1);
                    let mut p: PolygonMatrix = Default::default();

                    let center = 
                        (
                            MDLParser::next_f64(&mut args)?, 
                            MDLParser::next_f64(&mut args)?, 
                            MDLParser::next_f64(&mut args)?
                        );
                    let thickness = MDLParser::next_f64(&mut args)?;
                    let radius = MDLParser::next_f64(&mut args)?;

                    let ring_count = std::f64::consts::TAU * radius / SIDE_LENGTH;
                    let cir_count = std::f64::consts::TAU * thickness / SIDE_LENGTH;

                    let torus = Torus::new(thickness, radius, center);
                    torus.add_to_matrix(&mut p, ring_count as usize, cir_count as usize);

                    p = self.t.top().apply_poly(&p);
                    self.image.draw_polygons(&mut p, &DEFAULT_LIGHTING_CONFIG);
                    Ok(())
                },
                Rule::TORUS_SDDDDD => {
                    let mut args = command.into_inner().skip(1);
                    let mut p: PolygonMatrix = Default::default();

                    let constant = MDLParser::next(&mut args);
                    let center = 
                        (
                            MDLParser::next_f64(&mut args)?, 
                            MDLParser::next_f64(&mut args)?, 
                            MDLParser::next_f64(&mut args)?
                        );
                    let thickness = MDLParser::next_f64(&mut args)?;
                    let radius = MDLParser::next_f64(&mut args)?;

                    let ring_count = std::f64::consts::TAU * radius / SIDE_LENGTH;
                    let cir_count = std::f64::consts::TAU * thickness / SIDE_LENGTH;

                    let torus = Torus::new(thickness, radius, center);
                    torus.add_to_matrix(&mut p, ring_count as usize, cir_count as usize);

                    p = self.t.top().apply_poly(&p);
                    self.image.draw_polygons(&mut p, &self.constants[constant]);
                    Ok(())
                },
                Rule::SCALE_DDD => {
                    let mut args = command.into_inner().skip(1);
                    let mut scale_transform: Transformer = Default::default();
                    scale_transform.scale(
                        MDLParser::next_f64(&mut args)?, 
                        MDLParser::next_f64(&mut args)?, 
                        MDLParser::next_f64(&mut args)?
                    );
                    self.t.top().compose(&scale_transform);
                    Ok(())
                },
                Rule::MOVE_DDD => {
                    let mut args = command.into_inner().skip(1);
                    let mut move_transform: Transformer = Default::default();
                    move_transform.translate(
                        MDLParser::next_f64(&mut args)?, 
                        MDLParser::next_f64(&mut args)?, 
                        MDLParser::next_f64(&mut args)?
                    );
                    self.t.top().compose(&move_transform);
                    Ok(())
                },
                Rule::ROTATE_SD => {
                    let mut args = command.into_inner().skip(1);
                    let mut rotate_transform: Transformer = Default::default();
                    rotate_transform.rotate(
                        match MDLParser::next(&mut args) {
                            "x" => Axis::X,
                            "y" => Axis::Y,
                            "z" => Axis::Z,
                            _ => panic!("Unrecognized axis; use x/y/z.")
                        }, 
                        MDLParser::next_f64(&mut args)? * std::f64::consts::PI / 180.0
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
                    self.image = Box::new(Image::new("result".to_string()));
                    // self.t = Default::default();
                    Ok(())
                },
                Rule::DISPLAY => {
                    if let None = self.image.display().ok() {
                        eprintln!("Could not display image.");
                    }
                    Ok(())
                },
                Rule::SAVE_S => {
                    let mut args = command.into_inner().skip(1);
                    let filename = MDLParser::next(&mut args);
                    if filename.contains(".") {
                        self.image.save_name(filename).expect(format!("Could not save {}", filename).as_str());
                    } else {
                        panic!("No file extension found!");
                    }
                    Ok(())
                },
                Rule::EOI => Ok(()),
                _ => panic!("{} is unimplemented!", command.as_str())
            }
        })
        .collect()
    }
}

impl Default for MDLParser {
    fn default() -> Self {
        Self { 
            image: Box::new(Image::new("result".to_string())), 
            t: Default::default(),
            constants: HashMap::new()
        }
    }
}
