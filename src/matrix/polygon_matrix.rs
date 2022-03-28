use std::{fmt::Display, slice, ops::Mul};

use itertools::{Zip, multizip, Tuples, Itertools};
use rayon::iter::{IntoParallelRefMutIterator, ParallelIterator, IndexedParallelIterator, IntoParallelIterator};

use super::{Dynamic2D, ParallelGrid, Const2D};
#[derive(Clone, Debug)]
pub struct PolygonMatrix {
    matrix: Dynamic2D<f64>
}

impl PolygonMatrix {

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

    pub fn add_triangle(&mut self, p0: (f64, f64, f64), p1: (f64, f64, f64), p2: (f64, f64, f64)) {
        self.add_point(p0);
        self.add_point(p1);
        self.add_point(p2);
    }

}

impl Display for PolygonMatrix {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.matrix)
    }
}

impl Default for PolygonMatrix {
    fn default() -> Self {
        Self { matrix: Dynamic2D::new(0, 4) }
    }
}

impl Mul<PolygonMatrix> for Const2D<f64, 4, 4> {
    type Output = PolygonMatrix;

    fn mul(self, rhs: PolygonMatrix) -> Self::Output {
        PolygonMatrix::from(&(self * rhs.matrix))
    }
}

impl Mul<&PolygonMatrix> for &Const2D<f64, 4, 4> {
    type Output = PolygonMatrix;

    fn mul(self, rhs: &PolygonMatrix) -> Self::Output {
        PolygonMatrix::from(&(self * &rhs.matrix))
    }
}

impl Mul for PolygonMatrix {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        PolygonMatrix::from(&(self.matrix * rhs.matrix))
    }
}

impl<'data> IntoIterator for &'data PolygonMatrix {
    type Item = ((&'data f64, &'data f64, &'data f64), (&'data f64, &'data f64, &'data f64));
    type IntoIter = Tuples<Zip<(slice::Iter<'data, f64>, slice::Iter<'data, f64>, slice::Iter<'data, f64>)>, Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        multizip((self.matrix[0].iter(), self.matrix[1].iter(), self.matrix[2].iter())).tuples()
    }
}
