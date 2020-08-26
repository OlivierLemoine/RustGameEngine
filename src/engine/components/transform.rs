use super::super::prelude::*;
use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct Transform {
    #[serde(default)]
    pub position: Vector,
    #[serde(default = "Vector::one")]
    pub scale: Vector,
}
impl Default for Transform {
    fn default() -> Self {
        Transform {
            position: Vector::default(),
            scale: Vector::one(),
        }
    }
}
