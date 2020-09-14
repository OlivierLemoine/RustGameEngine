use super::prelude::*;

pub const NULL_CAMERA: Camera = Camera {
    position: Vector::zero(),
    zoom: Vector::one(),
};

#[derive(Debug)]
pub struct Camera {
    pub position: Vector,
    pub zoom: Vector,
}
impl Default for Camera {
    fn default() -> Self {
        Camera {
            position: Vector::zero(),
            zoom: Vector::one(),
        }
    }
}
