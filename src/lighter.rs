use std::cmp;

use crate::{Vector3D, Color, color::color_constants};

#[derive(Clone, Debug)]
pub struct Lighter {
    sources: Vec<(Vector3D, Color)>,
    spec_power: f64,
    ambient_color: Color,
    view_vector: Vector3D
}

#[derive(Clone, Copy, Debug)]
pub struct LightingConfig {
    pub ka: (f64, f64, f64),
    pub kd: (f64, f64, f64),
    pub ks: (f64, f64, f64)
}

impl Lighter {
    const SPEC_POWER: f64 = 3.0;

    pub fn from_sources(sources: Vec<(Vector3D, Color)>) -> Self {
        Self {
            sources,
            spec_power: Lighter::SPEC_POWER,
            ambient_color: color_constants::WHITE,
            view_vector: Vector3D::new(0.0, 0.0, 1.0)
        }
    }

    pub fn from_sources_ambient(sources: Vec<(Vector3D, Color)>, ambient_color: Color) -> Self {
        Self {
            sources,
            spec_power: Lighter::SPEC_POWER,
            ambient_color,
            view_vector: Vector3D::new(0.0, 0.0, 1.0)
        }
    }

    fn scale_color(color: &Color, k: (f64, f64, f64)) -> Color {
        Color { 
            red: (color.red as f64 * k.0) as u8, 
            green: (color.green as f64 * k.1) as u8, 
            blue: (color.blue as f64 * k.2) as u8
        }
    }

    fn add_color(color_a: &Color, color_b: &Color) -> Color {
        Color { 
            red: cmp::min(color_a.red.saturating_add(color_b.red), 255), 
            green: cmp::min(color_a.green.saturating_add(color_b.green), 255), 
            blue: cmp::min(color_a.blue.saturating_add(color_b.blue), 255)
        }
    }

    fn calc_ambient(&self, conf: &LightingConfig) -> Color {
        Lighter::scale_color(&self.ambient_color, conf.ka)
    }

    fn calc_diffuse(&self, normal: &Vector3D, conf: &LightingConfig) -> Color {
        let normalized = normal.normalize();
        let mut result = color_constants::BLACK;
        for (source_vec, color) in &self.sources {
            let normalized_source = source_vec.normalize();
            let dotprod = normalized.dot(&normalized_source);
            result = Lighter::add_color(
                &result, 
                &Lighter::scale_color(&color, (dotprod * conf.kd.0, dotprod * conf.kd.1, dotprod * conf.kd.2))
            );
        }
        result
    }

    fn calc_specular(&self, normal: &Vector3D, conf: &LightingConfig) -> Color {
        let normalized = normal.normalize();
        let mut result = color_constants::BLACK;
        for (source_vec, color) in &self.sources {
            let normalized_source = source_vec.normalize();
            let scale = (normalized.scale(2.0 * normalized.dot(&normalized_source)) - normalized_source).dot(&self.view_vector).powf(self.spec_power);
            result = Lighter::add_color(
                &result, 
                &Lighter::scale_color(&color, (scale * conf.ks.0, scale * conf.ks.1, scale * conf.ks.2))
            );
        }
        result
    }

    pub fn add_source(&mut self, direction: Vector3D, color: Color) {
        self.sources.push((direction, color));
    }

    pub fn set_ambient(&mut self, color: Color) {
        self.ambient_color = color;
    }

    pub fn calculate(&self, normal: &Vector3D, conf: &LightingConfig) -> Color {
        let mut result = color_constants::BLACK;
        result = Lighter::add_color(&result,&self.calc_ambient(conf));
        result = Lighter::add_color(&result,&self.calc_diffuse(normal, conf));
        result = Lighter::add_color(&result,&self.calc_specular(normal, conf));
        result
    }
}

impl Default for Lighter {
    fn default() -> Self {
        Self { 
            sources: vec![(Vector3D::new(1.0, 1.0, 1.0), color_constants::WHITE)], 
            spec_power: Lighter::SPEC_POWER, 
            ambient_color: color_constants::WHITE, 
            view_vector: Vector3D::new(0.0, 0.0, 1.0)
        }
    }
}