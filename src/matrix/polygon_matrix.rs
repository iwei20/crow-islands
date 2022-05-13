use std::{fmt::Display, slice, ops::Mul, iter::Copied};

use itertools::{Zip, multizip, Tuples, Itertools};
use rayon::iter::{IntoParallelRefMutIterator, ParallelIterator, IndexedParallelIterator, IntoParallelIterator, Chunks, MultiZip, IntoParallelRefIterator};

use crate::Vector3D;

use super::{Dynamic2D, ParallelGrid, Const2D};
#[derive(Clone, Debug)]
pub struct PolygonMatrix {
    matrix: Dynamic2D<f64>,
    normals: Vec<Vector3D>
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

        let normals = 
            (copy[0].par_iter().copied(), copy[1].par_iter().copied(), copy[2].par_iter().copied())
                .into_par_iter()
                .chunks(3)
                .map(|points| -> Vector3D {
                    Vector3D::from_points(points[0], points[1]).cross(&Vector3D::from_points(points[0], points[2]))
                })
                .collect();

        Self {
            matrix: copy,
            normals
        }
    }

    pub fn from_fast(edgelist: Dynamic2D<f64>) -> Self {
        debug_assert_eq!(edgelist.get_height(), 4, "Given grid must have a height of 4 to be converted to an edge matrix.");

        let normals = 
            (edgelist[0].par_iter().copied(), edgelist[1].par_iter().copied(), edgelist[2].par_iter().copied())
                .into_par_iter()
                .chunks(3)
                .map(|points| -> Vector3D {
                    Vector3D::from_points(points[0], points[1]).cross(&Vector3D::from_points(points[0], points[2]))
                })
                .collect();

        Self {
            matrix: edgelist,
            normals
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

    pub fn get_poly_count(&self) -> usize {
        self.matrix.get_width() / 3
    }
}

impl Display for PolygonMatrix {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.matrix)
    }
}

impl Default for PolygonMatrix {
    fn default() -> Self {
        Self { matrix: Dynamic2D::new(0, 4), normals: vec![] }
    }
}

impl Mul<PolygonMatrix> for Const2D<f64, 4, 4> {
    type Output = PolygonMatrix;

    fn mul(self, rhs: PolygonMatrix) -> Self::Output {
        PolygonMatrix::from_fast(self * rhs.matrix)
    }
}

impl Mul<&PolygonMatrix> for &Const2D<f64, 4, 4> {
    type Output = PolygonMatrix;

    fn mul(self, rhs: &PolygonMatrix) -> Self::Output {
        PolygonMatrix::from_fast(self * &rhs.matrix)
    }
}

impl Mul for PolygonMatrix {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        PolygonMatrix::from(&(self.matrix * rhs.matrix))
    }
}

impl<'data> IntoIterator for &'data PolygonMatrix {
    type Item = ((f64, f64, f64), (f64, f64, f64), (f64, f64, f64));
    type IntoIter = Tuples<Zip<(Copied<slice::Iter<'data, f64>>, Copied<slice::Iter<'data, f64>>, Copied<slice::Iter<'data, f64>>)>, Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        multizip((self.matrix[0].iter().copied(), self.matrix[1].iter().copied(), self.matrix[2].iter().copied())).tuples()
    }
}

impl<'data> IntoParallelIterator for &'data PolygonMatrix {
    type Item = (Vec<(f64, f64, f64)>, Vector3D);
    type Iter = 
        rayon::iter::Zip<
            Chunks<
                MultiZip<(
                    rayon::iter::Copied<rayon::slice::Iter<'data, f64>>, 
                    rayon::iter::Copied<rayon::slice::Iter<'data, f64>>, 
                    rayon::iter::Copied<rayon::slice::Iter<'data, f64>>
                )>
            >,
            rayon::iter::Copied<rayon::slice::Iter<'data, Vector3D>>
        >;

    fn into_par_iter(self) -> Self::Iter {
        (
            self.matrix[0].par_iter().copied(), 
            self.matrix[1].par_iter().copied(), 
            self.matrix[2].par_iter().copied()
        )
            .into_par_iter()
            .chunks(3)
            .zip(self.normals.par_iter().copied())
    }
}
