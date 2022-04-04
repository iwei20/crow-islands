use crate::matrix::PolygonMatrix;

#[derive(Clone, Copy, Debug)]
pub struct Torus {
    thickness: f64,
    radius: f64,
    center: (f64, f64, f64)
}

impl Torus {
    pub fn new(thickness: f64, radius: f64, center: (f64, f64, f64)) -> Self {
        Self {
            thickness,
            radius,
            center
        }
    }

    fn generate_torus(&self, ring_steps: usize, cir_steps: usize) -> Vec<(f64, f64, f64)> {
        (0..=ring_steps)
            .flat_map(|s0| {
                (0..=cir_steps)
                    .map(move |s1| -> (f64, f64, f64) {
                        let p = s0 as f64 / ring_steps as f64;
                        let t = s1 as f64 / cir_steps as f64;
                        (
                            (p * std::f64::consts::TAU).cos() * (self.thickness * (std::f64::consts::TAU * t).cos() + self.radius) + self.center.0,
                            self.thickness * (std::f64::consts::TAU * t).sin() + self.center.1,
                            -(p * std::f64::consts::TAU).sin() * (self.thickness * (std::f64::consts::TAU * t).cos() + self.radius) + self.center.2,
                        )
                    })
            })
            .collect()
    }

    pub fn add_to_matrix(&self, p: &mut PolygonMatrix, ring_steps: usize, cir_steps: usize) {
        let points = self.generate_torus(ring_steps, cir_steps);
        (0..ring_steps)
            .for_each(|s0| {
                (0..cir_steps)
                    .for_each(|s1| {
                        p.add_triangle(points[s0 * (cir_steps + 1) + s1], points[s0 * (cir_steps + 1) + s1 + 1], points[(s0 + 1) * (cir_steps + 1) + s1 + 1]);
                        p.add_triangle(points[s0 * (cir_steps + 1) + s1], points[(s0 + 1) * (cir_steps + 1) + s1 + 1], points[(s0 + 1) * (cir_steps + 1) + s1]);
                    });
            });
    }
}
