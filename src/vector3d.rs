use std::{
    iter::Sum,
    ops::{Add, Sub},
};

#[derive(Clone, Copy, Debug)]
pub struct Vector3D {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Add for Vector3D {
    type Output = Vector3D;

    fn add(self, rhs: Self) -> Self::Output {
        Self::Output {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl Sub for Vector3D {
    type Output = Vector3D;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::Output {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl Sum for Vector3D {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Vector3D::new(0.0, 0.0, 0.0), |sum, x| sum + x)
    }
}

impl Vector3D {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    pub fn from_point(p: (f64, f64, f64)) -> Self {
        Self {
            x: p.0,
            y: p.1,
            z: p.2,
        }
    }

    pub fn from_points(p0: (f64, f64, f64), p1: (f64, f64, f64)) -> Self {
        Self {
            x: p1.0 - p0.0,
            y: p1.1 - p0.1,
            z: p1.2 - p0.2,
        }
    }

    pub fn dot(&self, other: &Self) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn cross(&self, other: &Self) -> Self {
        Self {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }

    pub fn scale(&self, factor: f64) -> Self {
        Self {
            x: self.x * factor,
            y: self.y * factor,
            z: self.z * factor,
        }
    }

    pub fn normalize(&self) -> Self {
        let magnitude = (self.x * self.x + self.y * self.y + self.z * self.z).sqrt();
        Self {
            x: self.x / magnitude,
            y: self.y / magnitude,
            z: self.z / magnitude,
        }
    }

    pub fn average(vectors: impl Iterator<Item = Self>) -> Self {
        vectors.sum::<Self>().normalize()
    }

    pub fn interpolate(vectors_weights: impl Iterator<Item = (Self, f64)>) -> Vector3D {
        vectors_weights
            .map(|(vector, weight)| -> Self {
                vector.scale(weight)
            })
            .sum::<Self>()
            .normalize()
    }
}
