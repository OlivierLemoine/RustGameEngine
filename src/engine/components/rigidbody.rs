use super::super::prelude::*;
use serde::Deserialize;

fn one() -> f32 {
    1f32
}

#[derive(Deserialize, Debug)]
pub struct Rigidbody {
    #[serde(default = "one")]
    pub mass: f32,
    #[serde(default)]
    pub acceleration: Vector,
    #[serde(default)]
    pub speed: Vector,
    #[serde(default)]
    pub force: Vector,

    #[serde(skip)]
    pub is_on_ground: bool,
}

impl Default for Rigidbody {
    fn default() -> Self {
        Rigidbody {
            mass: 1f32,
            acceleration: Vector::zero(),
            speed: Vector::zero(),
            force: Vector::zero(),

            is_on_ground: false,
        }
    }
}
