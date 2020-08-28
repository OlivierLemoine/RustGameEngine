use super::super::prelude::*;
use serde::Deserialize;

pub type OnClick = fn(&mut Object);
pub type Update = fn(&mut Object);

#[derive(Deserialize, Default, Debug)]
pub struct Script {
    #[serde(default)]
    pub lib: String,
}
