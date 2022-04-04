use crate::matrix::Const2D;
use super::Parametric;

#[derive(Clone, Debug)]
pub struct Bezier {
    coeff_x: Const2D<f64, 1, 4>,
    coeff_y: Const2D<f64, 1, 4>
}

impl Bezier {
    pub fn new(
        p0: (f64, f64),
        p1: (f64, f64),
        p2: (f64, f64),
        p3: (f64, f64)
    ) -> Self {
        let bezier_mul = Const2D::from([
            [-1.0, 3.0, -3.0, 1.0],
            [3.0, -6.0, 3.0, 0.0],
            [-3.0, 3.0, 0.0, 0.0],
            [1.0, 0.0, 0.0, 0.0],
        ]);

        Self {
            coeff_x: &bezier_mul * &Const2D::from([
                [p0.0],
                [p1.0],
                [p2.0],
                [p3.0]
            ]),
            coeff_y: &bezier_mul * &Const2D::from([
                [p0.1],
                [p1.1],
                [p2.1],
                [p3.1]
            ]),
        }
    }
}

impl Parametric for Bezier {
    fn x(&self, t: f64) -> f64 {
        self.coeff_x[0][0] * t * t * t +
        self.coeff_x[1][0] * t * t +
        self.coeff_x[2][0] * t +
        self.coeff_x[3][0]
    }

    fn y(&self, t: f64) -> f64 {
        self.coeff_y[0][0] * t * t * t +
        self.coeff_y[1][0] * t * t +
        self.coeff_y[2][0] * t +
        self.coeff_y[3][0]
    }

    fn z(&self, _t: f64) -> f64 {
        0.0
    }
}
