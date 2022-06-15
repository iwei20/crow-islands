pub trait Parametric {
    fn x(&self, t: f64) -> f64;
    fn y(&self, t: f64) -> f64;
    fn z(&self, t: f64) -> f64;

    fn f(&self, t: f64) -> (f64, f64, f64) {
        (self.x(t), self.y(t), self.z(t))
    }

    fn points(&self, n: usize) -> Vec<(f64, f64, f64)> {
        (0..n)
            .map(|i| -> (f64, f64, f64) { self.f(i as f64 / (n - 1) as f64) })
            .collect()
    }
}
