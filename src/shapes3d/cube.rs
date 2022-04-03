use crate::matrix::PolygonMatrix;

#[derive(Clone, Copy, Debug)]
pub struct Cube {
    left: f64,
    top: f64,
    front: f64,
    width: f64,
    height: f64,
    depth: f64
}

impl Cube {
    pub fn new(ltf: (f64, f64, f64), width: f64, height: f64, depth: f64) -> Self {
        Self {
            left: ltf.0,
            top: ltf.1,
            front: ltf.2,
            width,
            height,
            depth
        }
    }

    pub fn add_to_matrix(&self, p: &mut PolygonMatrix) {
        let ltf = (self.left, self.top, self.front);
        let lbf = (self.left, self.top - self.height, self.front);
        let lbb = (self.left, self.top - self.height, self.front - self.depth);
        let ltb = (self.left, self.top, self.front - self.depth);

        let rtf = (self.left + self.width, self.top, self.front);
        let rbf = (self.left + self.width, self.top - self.height, self.front);
        let rbb = (self.left + self.width, self.top - self.height, self.front - self.depth);
        let rtb = (self.left + self.width, self.top, self.front - self.depth);

        // Left face
        p.add_triangle(ltf, ltb, lbb);
        p.add_triangle(ltf, lbb, lbf);

        // Front face
        p.add_triangle(rtf, ltf, lbf);
        p.add_triangle(rtf, lbf, rbf);

        // Right face
        p.add_triangle(rtb, rtf, rbf);
        p.add_triangle(rtb, rbf, rbb);

        // Back face
        p.add_triangle(ltb, rtb, rbb);
        p.add_triangle(ltb, rbb, lbb);

        // Top face
        p.add_triangle(rtb, ltb, ltf);
        p.add_triangle(rtb, ltf, rtf);

        // Bottom face
        p.add_triangle(rbf, lbf, lbb);
        p.add_triangle(rbf, lbb, rbb);
    }
}
