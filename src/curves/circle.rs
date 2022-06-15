use super::Parametric;

#[derive(Clone, Copy, Debug)]
pub struct Circle {
    radius: f64,
    center: (f64, f64, f64),
}

impl Circle {
    pub fn new(radius: f64, center: (f64, f64, f64)) -> Self {
        Self { radius, center }
    }
}

impl Parametric for Circle {
    fn x(&self, t: f64) -> f64 {
        self.center.0 + self.radius * (std::f64::consts::TAU * t).cos()
    }

    fn y(&self, t: f64) -> f64 {
        self.center.1 + self.radius * (std::f64::consts::TAU * t).sin()
    }

    fn z(&self, _t: f64) -> f64 {
        self.center.2
    }
}
