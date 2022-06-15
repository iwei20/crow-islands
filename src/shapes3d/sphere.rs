use rayon::iter::{IntoParallelIterator, ParallelIterator};

use crate::matrix::PolygonMatrix;

#[derive(Clone, Copy, Debug)]
pub struct Sphere {
    radius: f64,
    center: (f64, f64, f64),
}

impl Sphere {
    pub fn new(radius: f64, center: (f64, f64, f64)) -> Self {
        Self { radius, center }
    }

    fn generate(&self, steps: usize) -> Vec<(f64, f64, f64)> {
        let circle_steps: usize = steps / 2;
        (0..steps)
            .into_par_iter()
            .flat_map(|s| {
                (0..=circle_steps)
                    .into_par_iter()
                    .map(move |cs| -> (f64, f64, f64) {
                        let cir = cs as f64 / circle_steps as f64;
                        let rot = s as f64 / steps as f64;
                        (
                            self.radius * (std::f64::consts::PI * cir).cos() + self.center.0,
                            self.radius
                                * (std::f64::consts::PI * cir).sin()
                                * (std::f64::consts::TAU * rot).cos()
                                + self.center.1,
                            self.radius
                                * (std::f64::consts::PI * cir).sin()
                                * (std::f64::consts::TAU * rot).sin()
                                + self.center.2,
                        )
                    })
            })
            .collect()
    }

    pub fn add_to_matrix(&self, p: &mut PolygonMatrix, steps: usize) {
        let points = self.generate(steps);
        let n: usize = steps / 2 + 1;
        (0..steps - 1).for_each(|turn| {
            p.add_triangle(
                points[turn * n],
                points[turn * n + 1],
                points[(turn + 1) * n + 1],
            );
            p.add_triangle(
                points[(turn + 1) * n - 2],
                points[(turn + 1) * n - 1],
                points[(turn + 2) * n - 2],
            );
            (turn * n + 1..(turn + 1) * n - 2).for_each(|pi| {
                p.add_triangle(points[pi], points[pi + 1], points[pi + n + 1]);
                p.add_triangle(points[pi], points[pi + n + 1], points[pi + n]);
            });
        });

        p.add_triangle(
            points[(steps - 1) * n],
            points[(steps - 1) * n + 1],
            points[1],
        );
        p.add_triangle(points[steps * n - 2], points[steps * n - 1], points[n - 2]);

        (1..n - 2).for_each(|pi| {
            p.add_triangle(
                points[(steps - 1) * n + pi],
                points[(steps - 1) * n + pi + 1],
                points[pi + 1],
            );
            p.add_triangle(points[(steps - 1) * n + pi], points[pi + 1], points[pi]);
        });
    }
}
