#[cfg(test)]
pub mod tests {
    use crate::image::Image;
    use std::{fs::File, io::Write};
    use crate::color::color_constants;

    const STEPS: usize = 1000;
    const DELTA: [(i32, i32); 8] = [(-1, 0), (-1, -1), (0, -1), (1, -1), (1, 0), (1, 1), (0, 1), (-1, 1)];

    fn conway_step(prev_state: &[[bool; 100]; 100]) -> [[bool; 100]; 100] {
        let mut result = [[false; 100]; 100];
        for r in 1..99 {
            for c in 1..99 {
                let count_neighbors: i32 = 
                    DELTA
                        .iter()
                        .map(|(dr, dc)| if prev_state[(r + dr) as usize][(c + dc) as usize] {1} else {0})
                        .sum();
                
                result[r as usize][c as usize] = match count_neighbors {
                    2 => (prev_state[r as usize][c as usize]),
                    3 => true,
                    _other => false
                }
            }
        }
        result
    }

    fn draw_gosper(state: & mut[[bool; 100]; 100], r: usize, c: usize) {
        let gosper_deltas = [
            // Left square
            (4, 0),
            (5, 0),
            (4, 1),
            (5, 1),

            // Left arc
            (4, 10),
            (5, 10),
            (6, 10),
            (3, 11),
            (7, 11),
            (2, 12),
            (8, 12),
            (2, 13),
            (8, 13),

            // Center point
            (5, 14),
            
            // Rightwards triangle
            (3, 15),
            (7, 15),
            (4, 16),
            (5, 16),
            (6, 16),
            (5, 17),

            // Leftwards blunt tirangle
            (2, 20),
            (3, 20),
            (4, 20),
            (2, 21),
            (3, 21),
            (4, 21),
            (1, 22),
            (5, 22),

            // Two short rectangles
            (0, 24),
            (1, 24),
            (5, 24),
            (6, 24),

            // Rightmost square
            (2, 34),
            (3, 34),
            (2, 35),
            (3, 35)
        ];

        gosper_deltas.iter().for_each(|(dr, dc)| state[r + dr][c + dc] = true);
    }

    #[test]
    fn threegliders() {
        let mut img = Image::new(500, 500);
        let mut plot = [[false; 100]; 100];

        draw_gosper(&mut plot, 1, 1);
        draw_gosper(&mut plot, 1, 51);
        draw_gosper(&mut plot, 51, 1);
        for _ in 0..STEPS {
            plot = conway_step(&plot);
        }

        for (r, row) in plot.iter().enumerate() {
            for (c, ele) in row.iter().enumerate() {
                if *ele {
                    for ri in 0..5 {
                        for ci in 0..5 {
                            img[r * 5 + ri][c * 5 + ci] = color_constants::GREEN;
                        }
                    }
                }
            }
        }

        let mut fout = File::create("result.ppm").expect("Failed to create image file");
        write!(fout, "{}", img).expect("Failed to write to image file");
        println!("Image can be found at result.ppm or result.png");
    }
}
