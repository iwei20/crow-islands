use crate::matrix::EdgeMatrix;

pub fn generate_circle(radius: f64, center: (f64, f64, f64), steps: usize) -> Vec<(f64, f64, f64)> {
    let circle_steps: usize = steps / 2;
    (0..=circle_steps)
        .flat_map(|cs| {
            (0..=steps)
                .map(move |s| -> (f64, f64, f64) {
                    let cir = s as f64 / steps as f64;
                    let rot = cs as f64 / circle_steps as f64;
                    (
                        radius * (std::f64::consts::PI * cir).cos() + center.0,
                        radius * (std::f64::consts::PI * cir).sin() * (std::f64::consts::TAU * rot).cos() + center.1,
                        radius * (std::f64::consts::PI * cir).sin() * (std::f64::consts::TAU * rot).sin() + center.2,
                    )
                })
        })
        .collect()
}

pub fn generate_torus(thickness: f64, radius: f64, center: (f64, f64, f64), steps: usize) -> Vec<(f64, f64, f64)> {
    (0..=steps)
        .flat_map(|s0| {
            (0..=steps)
                .map(move |s1| -> (f64, f64, f64) {
                    let p = s0 as f64 / steps as f64;
                    let t = s1 as f64 / steps as f64;
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

pub fn add_box(e: &mut EdgeMatrix, ltf: (f64, f64, f64), width: f64, height: f64, depth: f64) {
    let (l, t, f) = ltf;
    let lbf = (l, t - height, f);
    let lbb = (l, t - height, f - depth);
    let ltb = (l, t, f - depth);

    let rtf = (l + width, t, f);
    let rbf = (l + width, t - height, f);
    let rbb = (l + width, t - height, f - depth);
    let rtb = (l + width, t, f - depth);

    // Left face
    e.add_edge(ltf, lbf);
    e.add_edge(ltf, ltb);
    e.add_edge(lbb, lbf);
    e.add_edge(lbb, ltb);

    // Right face
    e.add_edge(rtf, rbf);
    e.add_edge(rtf, rtb);
    e.add_edge(rbb, rbf);
    e.add_edge(rbb, rtb);

    // Connectors
    e.add_edge(ltf, rtf);
    e.add_edge(lbf, rbf);
    e.add_edge(lbb, rbb);
    e.add_edge(ltb, rtb);
}