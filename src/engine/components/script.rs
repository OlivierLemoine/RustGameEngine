use super::super::prelude::*;
use serde::Deserialize;

pub type OnClick = fn();

#[derive(Deserialize, Default, Debug)]
pub struct Script {
    #[serde(skip)]
    pub lib: Option<libloading::Library>,
    #[serde(skip)]
    pub on_click: Option<libloading::Symbol<OnClick>>,
}
