use crate::matrix::{PolygonMatrix, EdgeMatrix};

pub fn generate_sphere(radius: f64, center: (f64, f64, f64), steps: usize) -> Vec<(f64, f64, f64)> {
    let circle_steps: usize = steps / 2;
    (0..steps)
        .flat_map(|s| {
            (0..=circle_steps)
                .map(move |cs| -> (f64, f64, f64) {
                    let cir = cs as f64 / circle_steps as f64;
                    let rot = s as f64 / steps as f64;
                    (
                        radius * (std::f64::consts::PI * cir).cos() + center.0,
                        radius * (std::f64::consts::PI * cir).sin() * (std::f64::consts::TAU * rot).cos() + center.1,
                        radius * (std::f64::consts::PI * cir).sin() * (std::f64::consts::TAU * rot).sin() + center.2,
                    )
                })
        })
        .collect()
}

pub fn add_sphere(p: &mut PolygonMatrix, points: &Vec<(f64, f64, f64)>, steps: usize) {
    let n: usize = steps / 2 + 1;
    (0..steps - 1)
        .for_each(|turn| {
            p.add_triangle(points[turn * n], points[(turn + 1) * n + 1], points[turn * n + 1]);
            p.add_triangle(points[(turn + 1) * n - 2], points[(turn + 2) * n - 2], points[(turn + 1) * n - 1]);
            (turn * n + 1..(turn + 1) * n - 1)
                .for_each(|pi| {
                    p.add_triangle(points[pi], points[pi + n + 1], points[pi + 1]);
                    p.add_triangle(points[pi], points[pi + n], points[pi + n + 1]);
                });
        });
}

pub fn generate_torus(thickness: f64, radius: f64, center: (f64, f64, f64), ring_steps: usize, cir_steps: usize) -> Vec<(f64, f64, f64)> {
    (0..=ring_steps)
        .flat_map(|s0| {
            (0..=cir_steps)
                .map(move |s1| -> (f64, f64, f64) {
                    let p = s0 as f64 / ring_steps as f64;
                    let t = s1 as f64 / cir_steps as f64;
                    (
                        (p * std::f64::consts::TAU).cos() * (thickness * (std::f64::consts::TAU * t).cos() + radius) + center.0,
                        thickness * (std::f64::consts::TAU * t).sin() + center.1,
                        -(p * std::f64::consts::TAU).sin() * (thickness * (std::f64::consts::TAU * t).cos() + radius) + center.2,
                    )
                })
        })
        .collect()
}

pub fn add_points(e: &mut EdgeMatrix, points: &Vec<(f64, f64, f64)>) {
    points.iter().for_each(|point| {
        e.add_edge(point.clone(), point.clone());
    });
}

pub fn add_box(p: &mut PolygonMatrix, ltf: (f64, f64, f64), width: f64, height: f64, depth: f64) {
    let (l, t, f) = ltf;
    let lbf = (l, t - height, f);
    let lbb = (l, t - height, f - depth);
    let ltb = (l, t, f - depth);

    let rtf = (l + width, t, f);
    let rbf = (l + width, t - height, f);
    let rbb = (l + width, t - height, f - depth);
    let rtb = (l + width, t, f - depth);

    // Left face
    p.add_triangle(ltf, ltb, lbb);
    p.add_triangle(ltf, lbb, lbf);

    // Front face
    p.add_triangle(rtf, ltf, lbf);
    p.add_triangle(rtf, lbf, rbf);

    // Right face
    p.add_triangle(rtb, rtf, rbf);
    p.add_triangle(rtb, rbf, rbb);

    // Back face
    p.add_triangle(ltb, rtb, rbb);
    p.add_triangle(ltb, rbb, lbb);

    // Top face
    p.add_triangle(rtb, ltb, ltf);
    p.add_triangle(rtb, ltf, rtf);

    // Bottom face
    p.add_triangle(rbf, lbf, lbb);
    p.add_triangle(rbf, lbb, rbb);
}
