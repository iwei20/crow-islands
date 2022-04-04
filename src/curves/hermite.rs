use crate::matrix::Const2D;
use super::Parametric;

#[derive(Clone, Debug)]
pub struct Hermite {
    coeff_x: Const2D<f64, 1, 4>,
    coeff_y: Const2D<f64, 1, 4>
}

impl Hermite {
    pub fn new(
        p0: (f64, f64),
        p1: (f64, f64),
        r0: (f64, f64),
        r1: (f64, f64)
    ) -> Self {
        let hermite_inverse_solver = Const2D::from([
            [2.0, -2.0, 1.0, 1.0],
            [-3.0, 3.0, -2.0, -1.0],
            [0.0, 0.0, 1.0, 0.0],
            [1.0, 0.0, 0.0, 0.0],
        ]);

        Self {
            coeff_x: &hermite_inverse_solver * &Const2D::from([
                [p0.0],
                [p1.0],
                [r0.0],
                [r1.0],
            ]),
            coeff_y: &hermite_inverse_solver * &Const2D::from([
                [p0.1],
                [p1.1],
                [r0.1],
                [r1.1],
            ])
        } 
    }
}

impl Parametric for Hermite {
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