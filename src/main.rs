use graphics_year2::color::color_constants;
use graphics_year2::image::Image;
use graphics_year2::matrix::{EdgeMatrix, Const2D};
fn main() {
    let mut img: Image<500, 500> = Image::new();
    let mut edges: EdgeMatrix = Default::default();
    
    let mut m2: EdgeMatrix = Default::default();
    println!("\nTesting add_edge. Adding (1, 2, 3), (4, 5, 6) m2 =");
    m2.add_edge((1.0, 2.0, 3.0), (4.0, 5.0, 6.0));
    println!("{}", m2);

    println!("\nTesting ident. m1 =");
    let mut m1 = EdgeMatrix::from(&Const2D::<f64, 4, 4>::ident());
    println!("{}", m1);

    println!("\nTesting matrix_mult. m1 * m2 =");
    m2 = m1 * m2;
    println!("{}", m2);

    m1 = Default::default();
    m1.add_edge((1.0, 2.0, 3.0), (4.0, 5.0, 6.0));
    m1.add_edge((7.0, 8.0, 9.0), (10.0, 11.0, 12.0));
    println!("\nTesting Matrix mult. m1 =");
    println!("{}", m1);
    println!("\nTesting Matrix mult. m1 * m2 =");
    m2 = m1 * m2;
    println!("{}", m2);

    edges.add_edge((50.0, 450.0, 0.0), (100.0, 450.0, 0.0));
    edges.add_edge((50.0, 450.0, 0.0), (50.0, 400.0, 0.0));
    edges.add_edge((100.0, 450.0, 0.0), (100.0, 400.0, 0.0));
    edges.add_edge((100.0, 400.0, 0.0), (50.0, 400.0, 0.0));

    edges.add_edge((200.0, 450.0, 0.0), (250.0, 450.0, 0.0));
    edges.add_edge((200.0, 450.0, 0.0), (200.0, 400.0, 0.0));
    edges.add_edge((250.0, 450.0, 0.0), (250.0, 400.0, 0.0));
    edges.add_edge((250.0, 400.0, 0.0), (200.0, 400.0, 0.0));

    edges.add_edge((150.0, 400.0, 0.0), (130.0, 360.0, 0.0));
    edges.add_edge((150.0, 400.0, 0.0), (170.0, 360.0, 0.0));
    edges.add_edge((130.0, 360.0, 0.0), (170.0, 360.0, 0.0));

    edges.add_edge((100.0, 340.0, 0.0), (200.0, 340.0, 0.0));
    edges.add_edge((100.0, 320.0, 0.0), (200.0, 320.0, 0.0));
    edges.add_edge((100.0, 340.0, 0.0), (100.0, 320.0, 0.0));
    edges.add_edge((200.0, 340.0, 0.0), (200.0, 320.0, 0.0));  

    img.draw_matrix(&edges, color_constants::CYAN);
    img.write_file("bob").expect("Image write failed");
}