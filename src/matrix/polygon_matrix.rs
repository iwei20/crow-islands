use std::{collections::HashMap, fmt::Display, iter::Copied, ops::Mul, slice};

use itertools::{multizip, Itertools, Tuples, Zip};
use ordered_float::OrderedFloat;
use rayon::iter::{
    Chunks, IndexedParallelIterator, IntoParallelIterator, IntoParallelRefIterator,
    IntoParallelRefMutIterator, MultiZip, ParallelIterator,
};

use crate::Vector3D;

use super::{Const2D, Dynamic2D, ParallelGrid};
#[derive(Clone, Debug)]
pub struct PolygonMatrix {
    matrix: Dynamic2D<f64>,
    normals: Vec<Vector3D>,
    vertex_normals: Vec<Vector3D>,
}

impl PolygonMatrix {
    pub fn from(edgelist: &impl ParallelGrid<Item = f64>) -> Self {
        debug_assert_eq!(
            edgelist.get_height(),
            4,
            "Given grid must have a height of 4 to be converted to an edge matrix."
        );
        let mut copy = Dynamic2D::new(edgelist.get_width(), 4);
        copy.iter_mut().enumerate().for_each(|(r, row)| {
            row.par_iter_mut().enumerate().for_each(|(c, ele)| {
                *ele = *edgelist.at(r, c);
            })
        });
        PolygonMatrix::from_fast(copy)
    }

    pub fn from_fast(edgelist: Dynamic2D<f64>) -> Self {
        debug_assert_eq!(
            edgelist.get_height(),
            4,
            "Given grid must have a height of 4 to be converted to an edge matrix."
        );

        let normals: Vec<Vector3D> = (
            edgelist[0].par_iter().copied(),
            edgelist[1].par_iter().copied(),
            edgelist[2].par_iter().copied(),
        )
            .into_par_iter()
            .chunks(3)
            .map(|points| -> Vector3D {
                Vector3D::from_points(points[0], points[1])
                    .cross(&Vector3D::from_points(points[0], points[2]))
            })
            .collect();

        type OrderedPoint = (OrderedFloat<f64>, OrderedFloat<f64>, OrderedFloat<f64>);
        let mut vertex_triangle_map: HashMap<OrderedPoint, Vec<usize>> = HashMap::new();
        multizip((
            edgelist[0].iter().copied(),
            edgelist[1].iter().copied(),
            edgelist[2].iter().copied(),
        ))
        .chunks(3)
        .into_iter()
        .enumerate()
        .for_each(|(i, points)| {
            points.for_each(|point| {
                let hashable_point = (
                    OrderedFloat(point.0),
                    OrderedFloat(point.1),
                    OrderedFloat(point.2),
                );

                if vertex_triangle_map.get(&hashable_point).is_none() {
                    vertex_triangle_map.insert(hashable_point, Vec::new());
                }

                vertex_triangle_map
                    .get_mut(&hashable_point)
                    .unwrap()
                    .push(i);
            });
        });

        let mut vertex_point_map: HashMap<OrderedPoint, Vec<usize>> = HashMap::new();
        multizip((
            edgelist[0].iter().copied(),
            edgelist[1].iter().copied(),
            edgelist[2].iter().copied(),
        ))
        .enumerate()
        .for_each(|(i, point)| {
            let hashable_point = (
                OrderedFloat(point.0),
                OrderedFloat(point.1),
                OrderedFloat(point.2),
            );

            if vertex_point_map.get(&hashable_point).is_none() {
                vertex_point_map.insert(hashable_point, Vec::new());
            }

            vertex_point_map.get_mut(&hashable_point).unwrap().push(i);
        });

        let mut vertex_normals = vec![Vector3D::new(0.0, 0.0, 0.0); edgelist.get_width()];
        vertex_triangle_map
            .into_iter()
            .for_each(|(vertex, triangle_index_list)| {
                let vertex_normal =
                    Vector3D::average(normals.iter().copied().enumerate().filter_map(
                        |(i, normal)| {
                            if triangle_index_list.contains(&i) {
                                Some(normal)
                            } else {
                                None
                            }
                        },
                    ));
                vertex_point_map
                    .get(&vertex)
                    .unwrap()
                    .iter()
                    .for_each(|point_index| {
                        vertex_normals[*point_index] = vertex_normal;
                    });
            });

        Self {
            matrix: edgelist,
            normals,
            vertex_normals,
        }
    }

    fn add_point(&mut self, (x, y, z): (f64, f64, f64)) {
        self.matrix.add_col([x, y, z, 1f64].into_iter());
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
        Self {
            matrix: Dynamic2D::new(0, 4),
            normals: vec![],
            vertex_normals: Vec::new(),
        }
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
    type Item = (
        (
            (f64, f64, f64, Vector3D),
            (f64, f64, f64, Vector3D),
            (f64, f64, f64, Vector3D),
        ),
        Vector3D,
    );
    type IntoIter = std::iter::Zip<
        Tuples<
            Zip<(
                Copied<slice::Iter<'data, f64>>,
                Copied<slice::Iter<'data, f64>>,
                Copied<slice::Iter<'data, f64>>,
                Copied<slice::Iter<'data, Vector3D>>,
            )>,
            (
                (f64, f64, f64, Vector3D),
                (f64, f64, f64, Vector3D),
                (f64, f64, f64, Vector3D),
            ),
        >,
        Copied<slice::Iter<'data, Vector3D>>,
    >;

    fn into_iter(self) -> Self::IntoIter {
        multizip((
            self.matrix[0].iter().copied(),
            self.matrix[1].iter().copied(),
            self.matrix[2].iter().copied(),
            self.vertex_normals.iter().copied(),
        ))
        .tuples()
        .zip(self.normals.iter().copied())
    }
}

impl<'data> IntoParallelIterator for &'data PolygonMatrix {
    type Item = (Vec<(f64, f64, f64, Vector3D)>, Vector3D);
    type Iter = rayon::iter::Zip<
        Chunks<
            MultiZip<(
                rayon::iter::Copied<rayon::slice::Iter<'data, f64>>,
                rayon::iter::Copied<rayon::slice::Iter<'data, f64>>,
                rayon::iter::Copied<rayon::slice::Iter<'data, f64>>,
                rayon::iter::Copied<rayon::slice::Iter<'data, Vector3D>>,
            )>,
        >,
        rayon::iter::Copied<rayon::slice::Iter<'data, Vector3D>>,
    >;

    fn into_par_iter(self) -> Self::Iter {
        (
            self.matrix[0].par_iter().copied(),
            self.matrix[1].par_iter().copied(),
            self.matrix[2].par_iter().copied(),
            self.vertex_normals.par_iter().copied(),
        )
            .into_par_iter()
            .chunks(3)
            .zip(self.normals.par_iter().copied())
    }
}
