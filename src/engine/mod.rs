mod components;
mod loader;
mod math;
mod systems;

pub mod prelude {
    use super::*;
    pub use components::{rigidbody::Rigidbody, sprite::Sprite, transform::Transform};
    pub use loader::{Image, Object};
    pub use math::Vector;
}

pub struct Engine<'a> {
    display: crate::frame::Frame<'a>,
    objects: Vec<loader::Object>,
}

impl<'a> Engine<'a> {
    pub fn new(mut frame: crate::frame::Frame) -> Result<Engine, Box<dyn std::error::Error>> {
        let objects = loader::load_scene("./resources/scenes/scene.toml".into(), &mut frame)?;
        println!("{:?}", objects);
        Ok(Engine {
            display: frame,
            objects,
        })
    }
    pub fn step(&mut self, dt: &std::time::Duration) -> Result<(), Box<dyn std::error::Error>> {
        for object in &self.objects {
            match (&object.transform, &object.sprite) {
                (Some(t), Some(s)) => {
                    self.display.draw_image(crate::frame::Image {
                        position: t.position.to_array(),
                        scale: t.scale.to_array(),
                        texture: *s
                            .animations
                            .get(s.animations.keys().next().unwrap())
                            .unwrap()
                            .first()
                            .unwrap(),
                    });
                }
                _ => {}
            }
        }
        self.display.new_frame()
    }
    pub fn kill(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.display.kill()
    }
}
