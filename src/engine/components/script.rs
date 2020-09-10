use super::super::prelude::*;
use serde::Deserialize;

pub type ParseCustomObject = fn(&str) -> Result<Box<dyn std::any::Any>, Box<dyn std::error::Error>>;
pub type OnClick = fn(&mut Object);
pub type Update = fn(&mut Object, &mut Camera, &Time);

#[derive(Deserialize, Default, Debug)]
pub struct Script {
    #[serde(default)]
    pub lib: String,
}
