pub fn generate_circle(radius: f64, center: (f64, f64, f64), steps: usize) -> Vec<(f64, f64, f64)> {
    let circle_steps: usize = steps / 2;
    (0..=circle_steps)
        .flat_map(|cs| {
            (0..=steps)
                .map(|s| -> (f64, f64, f64) {
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
