use core::panic;
use std::{error::Error, fs, io::{Read, Write}, collections::HashMap, num::{ParseIntError, ParseFloatError}, process::{Command, Stdio}, time::Instant};
use itertools::Itertools;
use pest::{Parser, iterators::{Pair, Pairs}};
use pest_derive::Parser;
use rayon::iter::{IntoParallelRefMutIterator, IndexedParallelIterator, ParallelIterator};

use crate::{Image, matrix::{EdgeMatrix, PolygonMatrix}, Transformer, Axis, color::color_constants, curves::{Circle, Parametric, Hermite, Bezier}, shapes3d::*, TStack, lighter::LightingConfig};

#[derive(Clone, Debug)]
pub enum OutputType {
    Image(Frame),
    Animation(Vec<Frame>)
}

#[derive(Clone, Debug, Parser)]
#[grammar = "parser/grammar.pest"]
pub struct MDLParser {
    basename: Option<String>,
    frames: Option<OutputType>
}

#[derive(Clone, Debug)]
pub struct Frame {
    image: Box<Image<500, 500>>,
    t: TStack,
    constants: HashMap<String, LightingConfig>,
    knob_map: Option<HashMap<String, f64>>
}

const DEFAULT_LIGHTING_CONFIG: LightingConfig = LightingConfig {
    ka: (0.1, 0.1, 0.1),
    ks: (0.5, 0.5, 0.5),
    kd: (0.5, 0.5, 0.5)
};
const SIDE_LENGTH: f64 = 1.0;

impl MDLParser {
    fn next<'i>(args: &mut impl Iterator<Item = Pair<'i, Rule>>) -> &'i str {
        args.next().unwrap().as_str()
    }

    fn next_f64<'i>(args: &mut impl Iterator<Item = Pair<'i, Rule>>) -> Result<f64, ParseFloatError> {
        MDLParser::next(args).parse::<f64>()
    }

    fn next_usize<'i>(args: &mut impl Iterator<Item = Pair<'i, Rule>>) -> Result<usize, ParseIntError> {
        MDLParser::next(args).parse::<usize>()
    }

    pub fn parse_file(&mut self, mut file: fs::File) -> Result<(), Box<dyn Error>> {
        let mut program = String::new();
        file.read_to_string(&mut program)?;
        self.parse_str(program.as_str())
    }

    pub fn parse_str(&mut self, program: &str) -> Result<(), Box<dyn Error>> {
        let mut pairs = MDLParser::parse(Rule::MDL, program)?;
        
        let parse_result = pairs.next()
                                    .unwrap()
                                    .into_inner();

        let frames_opt = parse_result.clone().find(|pair| pair.as_rule() == Rule::FRAMES_ARG);
        match frames_opt {
            Some(frames_cmd) => {
                let size = frames_cmd.into_inner().nth(1).unwrap().as_str().parse()?;

                let mut frame_vec = vec![Frame::default(); size];

                if let Some(basename_cmd) = parse_result.clone().find(|pair| pair.as_rule() == Rule::BASENAME_ARG) {
                    self.basename = Some(basename_cmd.into_inner().nth(1).unwrap().as_str().to_string())
                }

                parse_result.clone()
                    .filter(|pair| pair.as_rule() == Rule::VARY_ARGS)
                    .try_for_each(|vary_cmd| -> Result<(), Box<dyn Error>> {
                        let mut args = vary_cmd.into_inner().skip(1);

                        let knob = MDLParser::next(&mut args);
                        let frame_start = MDLParser::next_usize(&mut args)?;
                        let frame_stop = MDLParser::next_usize(&mut args)?;
                        let length = frame_stop - frame_start + 1;

                        let lerp_start = MDLParser::next_f64(&mut args)?;
                        let lerp_stop = MDLParser::next_f64(&mut args)?;
                        
                        let lerp_mul = (lerp_stop - lerp_start) / ((length - 1) as f64);

                        frame_vec
                            .iter_mut()
                            .take(frame_start)
                            .for_each(|frame| {
                                frame.knob_map.as_mut().unwrap().entry(knob.to_string()).or_insert(lerp_start);
                            });

                        frame_vec
                            .iter_mut()
                            .skip(frame_start)
                            .take(length)
                            .enumerate()
                            .for_each(|(i, frame)| {
                                frame.knob_map.as_mut().unwrap().insert(knob.to_string(), lerp_start + lerp_mul * i as f64);
                            });

                        frame_vec
                            .iter_mut()
                            .skip(frame_start + length)
                            .for_each(|frame| {
                                frame.knob_map.as_mut().unwrap().entry(knob.to_string()).or_insert(lerp_stop);
                            });
                        Ok(())
                    })?;
                self.frames = Some(OutputType::Animation(frame_vec));
                    
            },
            None => {
                self.frames = Some(OutputType::Image(Default::default()));
                let vary_exists = parse_result.clone().map(|command| command.as_rule()).contains(&Rule::VARY_ARGS);
                if vary_exists {panic!("Vary exists without frames.")}
            }
        }

        match self.frames.as_mut().unwrap() {
            OutputType::Image(frame) => {
                let time = Instant::now();
                frame.parse_command(parse_result)?;
                println!("Drew image in {:?}.", time.elapsed());
                Ok(())
            },
            OutputType::Animation(frames) => {
                let drawn_frames = frames.par_iter_mut()
                    .enumerate()
                    .map(|(i, frame)| -> &mut Frame {
                        let local_parse_result = MDLParser::parse(Rule::MDL, program).expect("Program parse fail").next().unwrap().into_inner();
                        let time = Instant::now();
                        frame.parse_command(local_parse_result).expect("Command parse failed");
                        println!("Drew frame {} in {:?}.", i, time.elapsed());
                        frame
                    })
                    .collect::<Vec<_>>();
                
                if let None = self.basename {
                    self.basename = Some("result".to_string());
                }
                println!("Beginning file write to {}.gif...", self.basename.as_ref().unwrap());
                let time = Instant::now();
                fs::create_dir_all(self.basename.as_ref().unwrap().rsplit_once("/").unwrap_or((".", "")).0)?;

                let convert_syntax = format!("convert -delay 1.7 -loop 0 - {}.gif", self.basename.as_ref().unwrap());
                let mut convert_command = Command::new("sh")
                        .args(["-c", &convert_syntax])
                        .stdin(Stdio::piped())
                        .spawn()?;

                
                drawn_frames.iter().map(|frame| -> Result<(), Box<dyn Error>> {
                    convert_command.stdin.as_mut().unwrap().write_all(frame.image.to_string().as_bytes())?;
                    Ok(())
                }).collect::<Result<(),Box<dyn Error>>>()?;
                convert_command.wait()?;

                println!("Wrote frames to {}.gif in {:?}.", self.basename.as_ref().unwrap(), time.elapsed());
                Ok(())
            },
        }

    }
}

impl Frame {
    fn parse_command<'i>(&mut self, parse_result: Pairs<'i, Rule>) -> Result<(), Box<dyn Error>> {
        parse_result.map(|command| -> Result<(), Box<dyn Error>> {
            let mut args = command.clone().into_inner().skip(1);

            match command.as_rule() {
                Rule::CONSTANTS_SHORT_ARGS => self.process_constants(&mut args),
                Rule::LINE_DDDDDD => self.line(&mut args),
                Rule::CIRCLE_DDDD => self.circle(&mut args),
                Rule::HERMITE_DDDDDDDD => self.hermite(&mut args),
                Rule::BEZIER_DDDDDDDD => self.bezier(&mut args),
                Rule::BOX_DDDDDD => self.cube(&mut args, false),
                Rule::BOX_SDDDDDD => self.cube(&mut args, true),
                Rule::SPHERE_DDDD => self.sphere(&mut args, false),
                Rule::SPHERE_SDDDD => self.sphere(&mut args, true),
                Rule::TORUS_DDDDD => self.torus(&mut args, false),
                Rule::TORUS_SDDDDD => self.torus(&mut args, true),
                Rule::SCALE_DDD => self.scale(&mut args),
                Rule::SCALE_DDDS => self.scale(&mut args),
                Rule::MOVE_DDD => self.translate(&mut args),
                Rule::MOVE_DDDS => self.translate(&mut args),
                Rule::ROTATE_SD => self.rotate(&mut args),
                Rule::ROTATE_SDS => self.rotate(&mut args),
                Rule::TPUSH => Ok(self.t.push_copy()),
                Rule::TPOP => Ok(self.t.pop()),
                Rule::CLEAR => Ok(self.image = Box::new(Image::new("result".to_string()))), // self.t = Default::default();
                Rule::DISPLAY => Ok({self.image.display().ok();}),
                Rule::SAVE_S => self.save(&mut args),
                Rule::FRAMES_ARG => Ok(()),
                Rule::BASENAME_ARG => Ok(()),
                Rule::VARY_ARGS => Ok(()),
                Rule::EOI => Ok(()),
                _ => panic!("{} is unimplemented!", command.as_str())
            }
        })
        .collect()
    }

    pub fn process_constants<'i>(&mut self, args: &mut impl Iterator<Item = Pair<'i, Rule>>) -> Result<(), Box<dyn Error>> {
        let name = MDLParser::next(args).to_string();
        let reds = (
            MDLParser::next_f64(args)?,
            MDLParser::next_f64(args)?,
            MDLParser::next_f64(args)?
        );
        let greens = (
            MDLParser::next_f64(args)?,
            MDLParser::next_f64(args)?,
            MDLParser::next_f64(args)?
        );
        let blues = (
            MDLParser::next_f64(args)?,
            MDLParser::next_f64(args)?,
            MDLParser::next_f64(args)?
        );
        self.constants.insert(name, LightingConfig {
            ka: (reds.0, greens.0, blues.0),
            ks: (reds.1, greens.1, blues.1),
            kd: (reds.2, greens.2, blues.2)
        });
        Ok(())
    }

    pub fn line<'i>(&mut self, args: &mut impl Iterator<Item = Pair<'i, Rule>>) -> Result<(), Box<dyn Error>> {
        let mut e: EdgeMatrix = Default::default();
        e.add_edge(
            (
                    MDLParser::next_f64(args)?,
                    MDLParser::next_f64(args)?,
                    MDLParser::next_f64(args)?
                ),
                (
                    MDLParser::next_f64(args)?,
                    MDLParser::next_f64(args)?,
                    MDLParser::next_f64(args)?
                )
        );
        e = self.t.top().apply_edges(&e);
        self.image.draw_matrix(&mut e, color_constants::WHITE);
        Ok(())
    }

    pub fn circle<'i>(&mut self, args: &mut impl Iterator<Item = Pair<'i, Rule>>) -> Result<(), Box<dyn Error>> {
        let mut e: EdgeMatrix = Default::default();

        let center = 
            (
                MDLParser::next_f64(args)?,
                MDLParser::next_f64(args)?,
                MDLParser::next_f64(args)?
            );
        let radius = MDLParser::next_f64(args)?;
        
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
    }

    pub fn hermite<'i>(&mut self, args: &mut impl Iterator<Item = Pair<'i, Rule>>) -> Result<(), Box<dyn Error>> {
        let mut e: EdgeMatrix = Default::default();

        let p0 = (MDLParser::next_f64(args)?, MDLParser::next_f64(args)?);
        let p1 = (MDLParser::next_f64(args)?, MDLParser::next_f64(args)?);
        let r0 = (MDLParser::next_f64(args)?, MDLParser::next_f64(args)?);
        let r1 = (MDLParser::next_f64(args)?, MDLParser::next_f64(args)?);
        let hermite = Hermite::new(p0, p1, r0, r1);
        hermite
            .points(50)
            .windows(2)
            .for_each(|window| {
                e.add_edge(window[0], window[1])
            });
        e = self.t.top().apply_edges(&e);
        self.image.draw_matrix(&mut e, color_constants::WHITE);
        Ok(())
    }

    pub fn bezier<'i>(&mut self, args: &mut impl Iterator<Item = Pair<'i, Rule>>) -> Result<(), Box<dyn Error>> {
        let mut e: EdgeMatrix = Default::default();

        let p0 = (MDLParser::next_f64(args)?, MDLParser::next_f64(args)?);
        let p1 = (MDLParser::next_f64(args)?, MDLParser::next_f64(args)?);
        let p2 = (MDLParser::next_f64(args)?, MDLParser::next_f64(args)?);
        let p3 = (MDLParser::next_f64(args)?, MDLParser::next_f64(args)?);
        let bezier = Bezier::new(p0, p1, p2, p3);
        bezier
            .points(50)
            .windows(2)
            .for_each(|window| {
                e.add_edge(window[0], window[1])
            });
        e = self.t.top().apply_edges(&e);
        self.image.draw_matrix(&mut e, color_constants::WHITE);
        Ok(())
    }

    pub fn cube<'i>(&mut self, args: &mut impl Iterator<Item = Pair<'i, Rule>>, use_constant: bool) -> Result<(), Box<dyn Error>> {
        let mut p: PolygonMatrix = Default::default();

        let mut light_conf = None;
        if use_constant {
            let constant = MDLParser::next(args);
            light_conf = Some(&self.constants[constant]);
        }
        let ltf = 
            (
                MDLParser::next_f64(args)?, 
                MDLParser::next_f64(args)?,
                MDLParser::next_f64(args)?
            );
        let width = MDLParser::next_f64(args)?;
        let height = MDLParser::next_f64(args)?;
        let depth = MDLParser::next_f64(args)?;

        let cube = Cube::new(ltf, width, height, depth);
        cube.add_to_matrix(&mut p);

        p = self.t.top().apply_poly(&p);
        self.image.draw_polygons(&mut p, light_conf.unwrap_or(&DEFAULT_LIGHTING_CONFIG));
        Ok(())
    }

    pub fn sphere<'i>(&mut self, args: &mut impl Iterator<Item = Pair<'i, Rule>>, use_constant: bool) -> Result<(), Box<dyn Error>> {
        let mut p: PolygonMatrix = Default::default();

        let mut light_conf = None;
        if use_constant {
            let constant = MDLParser::next(args);
            light_conf = Some(&self.constants[constant]);
        }
        let center = 
            (
                MDLParser::next_f64(args)?, 
                MDLParser::next_f64(args)?, 
                MDLParser::next_f64(args)?
            );
        let radius = MDLParser::next_f64(args)?;

        let point_count = std::f64::consts::TAU * radius / SIDE_LENGTH;

        let sphere = Sphere::new(radius, center);
        sphere.add_to_matrix(&mut p, point_count as usize);

        p = self.t.top().apply_poly(&p);
        self.image.draw_polygons(&mut p, light_conf.unwrap_or(&DEFAULT_LIGHTING_CONFIG));
        Ok(())
    }

    pub fn torus<'i>(&mut self, args: &mut impl Iterator<Item = Pair<'i, Rule>>, use_constant: bool) -> Result<(), Box<dyn Error>> {
        let mut p: PolygonMatrix = Default::default();

        let mut light_conf = None;
        if use_constant {
            let constant = MDLParser::next(args);
            light_conf = Some(&self.constants[constant]);
        }
        let center = 
            (
                MDLParser::next_f64(args)?, 
                MDLParser::next_f64(args)?, 
                MDLParser::next_f64(args)?
            );
        let thickness = MDLParser::next_f64(args)?;
        let radius = MDLParser::next_f64(args)?;

        let ring_count = std::f64::consts::TAU * radius / SIDE_LENGTH;
        let cir_count = std::f64::consts::TAU * thickness / SIDE_LENGTH;

        let torus = Torus::new(thickness, radius, center);
        torus.add_to_matrix(&mut p, ring_count as usize, cir_count as usize);

        p = self.t.top().apply_poly(&p);
        self.image.draw_polygons(&mut p, light_conf.unwrap_or(&DEFAULT_LIGHTING_CONFIG));
        Ok(())
    }

    pub fn scale<'i>(&mut self, args: &mut impl Iterator<Item = Pair<'i, Rule>>) -> Result<(), Box<dyn Error>> {
        let mut scale_transform: Transformer = Default::default();
        let mut knob_mul = 1.0;
        let (sx, sy, sz) = (MDLParser::next_f64(args)?, MDLParser::next_f64(args)?, MDLParser::next_f64(args)?);
        if let Some(knob_map_r) = &self.knob_map {
            if let Some(knob) = args.next() {
                knob_mul = knob_map_r[knob.as_str()];
            }
        }
        scale_transform.scale(
            sx * knob_mul, 
            sy * knob_mul, 
            sz * knob_mul
        );
        self.t.top().compose(&scale_transform);
        Ok(())
    }

    pub fn translate<'i>(&mut self, args: &mut impl Iterator<Item = Pair<'i, Rule>>) -> Result<(), Box<dyn Error>> {
        let mut move_transform: Transformer = Default::default();
        let mut knob_mul = 1.0;
        let (tx, ty, tz) = (MDLParser::next_f64(args)?, MDLParser::next_f64(args)?, MDLParser::next_f64(args)?);
        if let Some(knob_map_r) = &self.knob_map {
            if let Some(knob) = args.next() {
                knob_mul = knob_map_r[knob.as_str()];
            }
        }
        move_transform.translate(
            tx * knob_mul, 
            ty * knob_mul, 
            tz * knob_mul
        );
        self.t.top().compose(&move_transform);
        Ok(())
    }

    pub fn rotate<'i>(&mut self, args: &mut impl Iterator<Item = Pair<'i, Rule>>) -> Result<(), Box<dyn Error>> {
        let mut rotate_transform: Transformer = Default::default();
        let mut knob_mul = 1.0;
        let axis = match MDLParser::next(args) {
            "x" => Axis::X,
            "y" => Axis::Y,
            "z" => Axis::Z,
            _ => panic!("Unrecognized axis; use x/y/z.")
        };
        let angle = MDLParser::next_f64(args)? * std::f64::consts::PI / 180.0;

        if let Some(knob_map_r) = &self.knob_map {
            if let Some(knob) = args.next() {
                knob_mul = knob_map_r[knob.as_str()];
            }
        }
        rotate_transform.rotate(axis, angle * knob_mul);
        self.t.top().compose(&rotate_transform);
        Ok(())
    }

    pub fn save<'i>(&mut self, args: &mut impl Iterator<Item = Pair<'i, Rule>>) -> Result<(), Box<dyn Error>> {
        let filename = MDLParser::next(args);
        if filename.contains(".") {
            self.image.save_name(filename).expect(format!("Could not save {}", filename).as_str());
        } else {
            panic!("No file extension found!");
        }
        Ok(())
    }
}

impl Default for MDLParser {
    fn default() -> Self {
        Self { 
            basename: Some("result".to_string()),
            frames: None
        }
    }
}

impl Default for Frame {
    fn default() -> Self {
        Self {
            image: Box::new(Image::new("result".to_string())), 
            t: Default::default(),
            constants: HashMap::new(),
            knob_map: Some(HashMap::new()),
        }
    }
}
