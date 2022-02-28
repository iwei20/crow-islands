use std::fmt::Display;

use rayon::iter::{IntoParallelRefMutIterator, ParallelIterator, IndexedParallelIterator, IntoParallelIterator};

use crate::grid::{Dynamic2D, ParallelGrid};
#[derive(Clone, Debug)]
pub struct EdgeMatrix {
    matrix: Dynamic2D<f64>
}

impl EdgeMatrix {

    pub fn from(edgelist: &impl ParallelGrid<Item = f64>) -> Self {
        debug_assert_eq!(edgelist.get_height(), 4, "Given grid must have a height of 4 to be converted to an edge matrix.");
        let mut copy = Dynamic2D::new(edgelist.get_width(), 4);
        copy.par_iter_mut().enumerate().for_each(|(r, row)| {
            row.par_iter_mut().enumerate().for_each(|(c, ele)| {
                *ele = *edgelist.at(r, c);
            })
        });

        Self {
            matrix: copy
        }
    }

    fn add_point(&mut self, (x, y, z): (f64, f64, f64)) {
        self.matrix.add_col([x, y, z, 1f64].into_par_iter());
    }

    pub fn add_edge(&mut self, p0: (f64, f64, f64), p1: (f64, f64, f64)) {
        self.add_point(p0);
        self.add_point(p1);
    }

}

impl Display for EdgeMatrix {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.matrix)
    }
}

impl Default for EdgeMatrix {
    fn default() -> Self {
        Self { matrix: Dynamic2D::new(0, 4) }
    }
}