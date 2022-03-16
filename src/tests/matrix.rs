use rand::prelude::SliceRandom;

use crate::{image::Image, matrix::{EdgeMatrix, Const2D}, color::color_constants};

/**
  You are in a series of n = 2^100 rooms, each numbered 1-n, which each have an infinite expanse of whiteboards. They are connected to each other by identical looking hallways. Your ultimate goal is to create an algorithm to be able to form an adjacency matrix of all the rooms at any point. You're allowed to carry the necessary stationary and a piece of paper that holds k bits. Each step, you move through a random hallway, but you are guaranteed to have traversed every hallway in both directions in n^n steps (this number probably doesn't matter). When you step outside of a room, your memory is wiped of all except the algorithm. You start in room 1, and you know the label of your current room. Minimize k, and describe the algorithm.

    - Adjacency matrix: a map represented by a 2d array where a[i][j] = true indicates the existence of a hallway between rooms i and j

    (CMIMC 2022)
    Proof for 1 bit was 100
    Proof for <20 bits was 80
    Proof for <150 bits is 60
 */
#[test]
fn btree() {
    let mut img: Image<500, 500> = Image::new_flip("btree".to_string(), true);
    let mut edges: EdgeMatrix = Default::default();

    let mut points: Vec<(f64, f64)> = Vec::new();
    (0..30).into_iter().for_each(|_| {points.push((
        rand::random::<f64>() * 500.0,
        rand::random::<f64>() * 500.0
    ))});
    
    for point in &points {
        for other in (&points).choose_multiple(&mut rand::thread_rng(), rand::random::<usize>() % points.len()) {
            edges.add_edge((point.0, point.1, 0.0), (other.0, other.1, 0.0));
        }
    }

    edges.add_edge((10.0, 10.0, 0.0), (10.0, 40.0, 0.0));
    edges.add_edge((10.0, 20.0, 0.0), (20.0, 30.0, 0.0));
    edges.add_edge((10.0, 20.0, 0.0), (20.0, 10.0, 0.0));

    edges.add_edge((0.0, 45.0, 0.0), (500.0, 45.0, 0.0));
    for lerp in 1..10 {
        let x = lerp as f64 * 50.0;
        edges.add_edge((x, 45.0, 0.0), (x, 0.0, 0.0));
    }
    img.draw_matrix(&edges, color_constants::RED);
    img.save_test().expect("btree file write failed");
}

#[test]
fn dwmatrix() {
    let mut img: Image<500, 500> = Image::new_flip("bob".to_string(), true);
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
    img.save_test().expect("Image write failed");
}