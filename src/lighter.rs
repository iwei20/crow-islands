use crate::{Vector3D, Color, color::{self, color_constants}};

#[derive(Clone, Debug)]
pub struct Lighter {
    sources: Vec<(Vector3D, Color)>,
    k_a: (f64, f64, f64),
    k_d: (f64, f64, f64),
    k_s: (f64, f64, f64),
    spec_power: f64,
    ambient_color: Color,
    view_vector: Vector3D
}

impl Lighter {
    pub fn new(
        sources: Vec<(Vector3D, Color)>,
        k_a: (f64, f64, f64),
        k_d: (f64, f64, f64),
        k_s: (f64, f64, f64),
        spec_power: f64,
        ambient_color: Color,
    ) -> Self {
        Self {
            sources,
            k_a,
            k_d,
            k_s,
            spec_power,
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
            red: color_a.red.saturating_add(color_b.red), 
            green: color_a.green.saturating_add(color_b.green), 
            blue: color_a.blue.saturating_add(color_b.blue)
        }
    }

    fn calc_ambient(&self) -> Color {
        Lighter::scale_color(&self.ambient_color, self.k_a)
    }

    fn calc_diffuse(&self, normal: &Vector3D) -> Color {
        let normalized = normal.normalize();
        let mut result = color_constants::BLACK;
        for (source_vec, color) in &self.sources {
            let normalized_source = source_vec.normalize();
            let dotprod = normalized.dot(&normalized_source);
            result = Lighter::add_color(
                &result, 
                &Lighter::scale_color(&color, (dotprod * self.k_d.0, dotprod * self.k_d.1, dotprod * self.k_d.2))
            );
        }
        result
    }

    fn calc_specular(&self, normal: &Vector3D) -> Color {
        let normalized = normal.normalize();
        let mut result = color_constants::BLACK;
        for (source_vec, color) in &self.sources {
            let normalized_source = source_vec.normalize();
            let scale = (normalized.scale(2.0 * normalized.dot(&normalized_source)) - normalized_source).dot(&self.view_vector).powf(self.spec_power);
            result = Lighter::add_color(
                &result, 
                &Lighter::scale_color(&color, (scale * self.k_s.0, scale * self.k_s.1, scale * self.k_s.2))
            );
        }
        result
    }

    pub fn add_source(&mut self, direction: Vector3D, color: Color) {
        self.sources.push((direction, color));
    }

    pub fn calculate(&self, normal: &Vector3D) -> Color {
        let mut result = color_constants::BLACK;
        result = Lighter::add_color(&result,&self.calc_ambient());
        result = Lighter::add_color(&result,&self.calc_diffuse(normal));
        result = Lighter::add_color(&result,&self.calc_specular(normal));
        result
    }
}