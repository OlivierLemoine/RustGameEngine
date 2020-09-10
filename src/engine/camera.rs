use super::prelude::*;

pub const NULL_CAMERA: Camera = Camera {
    position: Vector::zero(),
};

#[derive(Debug, Default)]
pub struct Camera {
    pub position: Vector,
}
