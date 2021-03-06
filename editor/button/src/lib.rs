mod prelude;

use macros::bind;
use prelude::*;

#[derive(Deserialize, Debug)]
pub struct Button {
    speed: f32,
    curve: Curve,
}

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

    let custom = (&**obj.custom.as_ref().unwrap())
        .downcast_ref::<Button>()
        .unwrap();

    let x = custom
        .curve
        .sample(time.start.elapsed().as_millis() as f32 / 1000.0);

    camera.position = Vector::new(x, 0.0);
}
