use serde::Deserialize;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

#[derive(Debug, Default, Copy, Clone, Deserialize)]
pub struct Vector {
    pub x: f32,
    pub y: f32,
}
impl Vector {
    pub fn new(x: f32, y: f32) -> Self {
        Vector { x, y }
    }
    pub fn zero() -> Self {
        Vector::new(0.0, 0.0)
    }
    pub fn one() -> Self {
        Vector::new(1.0, 1.0)
    }
    pub fn up() -> Self {
        Vector::new(0.0, 1.0)
    }
    pub fn normalize(mut self) -> Self {
        let f = self.magnitude();
        self.x /= f;
        self.y /= f;
        self
    }
    pub fn abs(self) -> Self {
        Vector::new(self.x.abs(), self.y.abs())
    }
    pub fn dot_product(v1: &Vector, v2: &Vector) -> f32 {
        v1.x * v2.x + v1.y * v2.y
    }
    pub fn to_array(&self) -> [f32; 2] {
        [self.x, self.y]
    }
    pub fn magnitude_squared(&self) -> f32 {
        Vector::dot_product(self, self)
    }
    pub fn magnitude(&self) -> f32 {
        self.magnitude_squared().sqrt()
    }
    pub fn angle_radians(&self) -> f32 {
        self.y.atan2(self.x)
    }
    pub fn normal(&self) -> Self {
        let Vector { x, y } = *self;
        Vector::new(-y, x).normalize()
    }
}

impl From<[f32; 2]> for Vector {
    fn from(arr: [f32; 2]) -> Self {
        Vector::new(arr[0], arr[1])
    }
}
impl Sub for Vector {
    type Output = Vector;

    fn sub(mut self, other: Vector) -> Self::Output {
        self -= other;
        self
    }
}
impl SubAssign for Vector {
    fn sub_assign(&mut self, other: Vector) {
        self.x -= other.x;
        self.y -= other.y;
    }
}
impl Add for Vector {
    type Output = Vector;

    fn add(mut self, other: Vector) -> Self::Output {
        self += other;
        self
    }
}
impl AddAssign for Vector {
    fn add_assign(&mut self, other: Vector) {
        self.x += other.x;
        self.y += other.y;
    }
}
impl MulAssign for Vector {
    fn mul_assign(&mut self, other: Vector) {
        self.x *= other.x;
        self.y *= other.y;
    }
}
impl Mul for Vector {
    type Output = Vector;
    fn mul(mut self, other: Vector) -> Self::Output {
        self *= other;
        self
    }
}
impl MulAssign<f32> for Vector {
    fn mul_assign(&mut self, other: f32) {
        self.x *= other;
        self.y *= other;
    }
}
impl Mul<f32> for Vector {
    type Output = Vector;
    fn mul(mut self, other: f32) -> Self::Output {
        self *= other;
        self
    }
}
impl DivAssign<f32> for Vector {
    fn div_assign(&mut self, other: f32) {
        self.x /= other;
        self.y /= other;
    }
}
impl Div<f32> for Vector {
    type Output = Vector;
    fn div(mut self, other: f32) -> Self::Output {
        self *= other;
        self
    }
}
