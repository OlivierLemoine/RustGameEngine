#[path = "../../prelude.rs"]
mod prelude;
use prelude::*;

pub struct Button {}

#[no_mangle]
unsafe extern "C" fn parse_custom_object(
    src: &str,
) -> Result<Box<dyn std::any::Any>, Box<dyn std::error::Error>> {
    unimplemented!()
}

#[no_mangle]
unsafe extern "C" fn on_click(obj: &mut Object) {
    if let Some(t) = &mut obj.transform {
        t.position += Vector::new(0.1, 0.0);
    }
}

#[no_mangle]
unsafe extern "C" fn update(obj: &mut Object, camera: &mut Camera, time: &mut Time) {
    if let Some(t) = &mut obj.transform {
        // t.position += Vector::new(-0.001, 0.0);
    }

    camera.position += Vector::new(0.01, 0.0) * (time.delta.as_millis() as f32);
}
