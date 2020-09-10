mod prelude;

use macros::bind;
use prelude::*;

#[derive(Deserialize, Debug)]
pub struct __CHANGE_ME_ENTITY_NAME_CLASS_CASE__ {}

#[bind(ParseCustomObject)]
fn parse_custom_object(src: &str) -> Result<Box<dyn std::any::Any>, Box<dyn std::error::Error>> {
    let custom_obj: __CHANGE_ME_ENTITY_NAME_CLASS_CASE__ = toml::from_str(src)?;
    Ok(Box::new(custom_obj))
}

#[bind(OnClick)]
fn on_click(obj: &mut Object) {}

#[bind(Update)]
fn update(obj: &mut Object, camera: &mut Camera, time: &Time) {}
