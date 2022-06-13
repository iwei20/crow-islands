use rand::{thread_rng, Rng};

use crate::{
    lighter::LightingConfig, matrix::PolygonMatrix, shapes3d::Sphere, Color, Image, Vector3D,
};
#[test]
fn generate() {
    let mut img: Image<500, 500> = Image::new("lightanimation".to_string());
    let mut p: PolygonMatrix = Default::default();

    let center = (250.0, 250.0, 250.0);
    let radius = 200.0;

    const SIDE_LENGTH: f64 = 3.0;
    let point_count = std::f64::consts::TAU * radius / SIDE_LENGTH;

    let sphere = Sphere::new(radius, center);
    sphere.add_to_matrix(&mut p, point_count as usize);

    for i in 0..60 {
        img.clear_shapes_only();
        img.get_lighter().add_source(
            Vector3D::new(
                thread_rng().gen::<f64>(),
                thread_rng().gen::<f64>(),
                thread_rng().gen::<f64>(),
            ),
            Color::new(
                thread_rng().gen_range(0..20),
                thread_rng().gen_range(0..20),
                thread_rng().gen_range(0..20),
            ),
        );

        img.draw_polygons(
            &mut p,
            &LightingConfig {
                ka: (0.1, 0.1, 0.1),
                ks: (0.5, 0.5, 0.5),
                kd: (0.5, 0.5, 0.5),
            },
        );
        img.save_name(format!("lightanimation{}", i).as_str())
            .expect("Image write failed");
    }
}
