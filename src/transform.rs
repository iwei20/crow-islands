use crate::matrix::{Const2D, EdgeMatrix, PolygonMatrix};

pub enum Axis {
    X, Y, Z
}

impl Axis {
    pub fn get_matrix(&self, angle: f64) -> Const2D<f64, 4, 4> {
        match self {
            Axis::X => Const2D::from([
                [1.0, 0.0, 0.0, 0.0],
                [0.0, angle.cos(), -angle.sin(), 0.0],
                [0.0, angle.sin(), angle.cos(), 0.0],
                [0.0, 0.0, 0.0, 1.0]
            ]),
            Axis::Y => Const2D::from([
                [angle.cos(), 0.0, angle.sin(), 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [-angle.sin(), 0.0, angle.cos(), 0.0],
                [0.0, 0.0, 0.0, 1.0]
            ]),
            Axis::Z => Const2D::from([
                [angle.cos(), -angle.sin(), 0.0, 0.0],
                [angle.sin(), angle.cos(), 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [0.0, 0.0, 0.0, 1.0]
            ])
        }
    }
}

#[derive(Clone, Debug)]
pub struct Transformer {
    transform_matrix: Const2D<f64, 4, 4>
}

impl Transformer {
    pub fn reset(&mut self) {
        self.transform_matrix = Const2D::ident();
    }

    pub fn scale(&mut self, sx: f64, sy: f64, sz: f64) {
        self.transform_matrix = 
            &Const2D::from([
                [sx, 0.0, 0.0, 0.0],
                [0.0, sy, 0.0, 0.0],
                [0.0, 0.0, sz, 0.0],
                [0.0, 0.0, 0.0, 1.0]
            ]) * &self.transform_matrix;
    }

    pub fn translate(&mut self, tx: f64, ty: f64, tz: f64) {
        self.transform_matrix = 
            &Const2D::from([
                [1.0, 0.0, 0.0, tx],
                [0.0, 1.0, 0.0, ty],
                [0.0, 0.0, 1.0, tz],
                [0.0, 0.0, 0.0, 1.0]
            ]) * &self.transform_matrix;
    }

    pub fn rotate(&mut self, axis: Axis, angle: f64) {
        self.transform_matrix = &axis.get_matrix(angle) * &self.transform_matrix;
    }

    pub fn apply_edges(&self, edge_matrix: &EdgeMatrix) -> EdgeMatrix {
        &self.transform_matrix * edge_matrix
    }

    pub fn apply_poly(&self, poly_matrix: &PolygonMatrix) -> PolygonMatrix {
        &self.transform_matrix * poly_matrix
    }
}

impl Default for Transformer {
    fn default() -> Self {
        Self { transform_matrix: Const2D::ident() }
    }
}
