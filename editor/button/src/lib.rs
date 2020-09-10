mod prelude;

use macros::bind;
use prelude::*;

#[derive(Deserialize, Debug)]
pub struct Button {}

#[bind(ParseCustomObject)]
fn parse_custom_object(src: &str) -> Result<Box<dyn std::any::Any>, Box<dyn std::error::Error>> {
    let custom_obj: Button = toml::from_str(src)?;
    Ok(Box::new(custom_obj))
}

#[bind(OnClick)]
fn on_click(obj: &mut Object) {
    if let Some(t) = &mut obj.transform {
        t.position += Vector::new(0.1, 0.0);
    }
}

#[bind(Update)]
fn update(obj: &mut Object, camera: &mut Camera, time: &Time) {
    if let Some(t) = &mut obj.transform {
        // t.position += Vector::new(-0.001, 0.0);
    }

    camera.position += Vector::new(0.0001, 0.0) * (time.delta.as_millis() as f32);
}
