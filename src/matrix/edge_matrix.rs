use std::{fmt::Display, slice, ops::Mul, iter::Copied};

use itertools::{Zip, multizip, Tuples, Itertools};
use rayon::iter::{IntoParallelRefMutIterator, ParallelIterator, IndexedParallelIterator, IntoParallelIterator};

use super::{Dynamic2D, ParallelGrid, Const2D};
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
        self.matrix.add_col([x, y, z, 1f64].into_iter());
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

impl Mul<EdgeMatrix> for Const2D<f64, 4, 4> {
    type Output = EdgeMatrix;

    fn mul(self, rhs: EdgeMatrix) -> Self::Output {
        EdgeMatrix::from(&(self * rhs.matrix))
    }
}

impl Mul<&EdgeMatrix> for &Const2D<f64, 4, 4> {
    type Output = EdgeMatrix;

    fn mul(self, rhs: &EdgeMatrix) -> Self::Output {
        EdgeMatrix::from(&(self * &rhs.matrix))
    }
}

impl Mul for EdgeMatrix {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        EdgeMatrix::from(&(self.matrix * rhs.matrix))
    }
}

impl<'data> IntoIterator for &'data EdgeMatrix {
    type Item = ((f64, f64, f64), (f64, f64, f64));
    type IntoIter = Tuples<Zip<(Copied<slice::Iter<'data, f64>>, Copied<slice::Iter<'data, f64>>, Copied<slice::Iter<'data, f64>>)>, Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        multizip((self.matrix[0].iter().copied(), self.matrix[1].iter().copied(), self.matrix[2].iter().copied())).tuples()
    }
}