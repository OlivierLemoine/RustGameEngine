use crate::Vector;
use std::ops::Mul;

#[derive(Debug, Default, Copy, Clone)]
pub struct Matrix {
    pub a: f32,
    pub b: f32,
    pub c: f32,
    pub d: f32,
}
impl Matrix {
    pub fn new(a: f32, b: f32, c: f32, d: f32) -> Self {
        Matrix { a, b, c, d }
    }
    pub fn from_base(v1: &Vector, v2: &Vector) -> Self {
        Matrix::new(v1.x, v2.x, v1.y, v2.y)
    }
    pub fn inverse(self) -> Self {
        let Matrix { a, b, c, d } = self;

        let det = a * d - b * c;
        let factor = 1.0 / det;

        Matrix {
            a: d * factor,
            b: -b * factor,
            c: -c * factor,
            d: a * factor,
        }
    }
}

impl Mul<Vector> for Matrix {
    type Output = Vector;

    fn mul(self, other: Vector) -> Self::Output {
        Vector {
            x: self.a * other.x + self.b * other.y,
            y: self.c * other.x + self.d * other.y,
        }
    }
}
impl<'a> Mul<&'a Vector> for &'a Matrix {
    type Output = Vector;

    fn mul(self, other: &Vector) -> Self::Output {
        Vector {
            x: self.a * other.x + self.b * other.y,
            y: self.c * other.x + self.d * other.y,
        }
    }
}
