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
    scene_path: String,
}

impl<'a> Engine<'a> {
    pub fn new(
        mut frame: crate::frame::Frame<'a>,
        scene_path: String,
    ) -> Result<Engine<'a>, Box<dyn std::error::Error>> {
        let objects = loader::load_scene(&scene_path, &mut frame)?;
        println!("{:?}", objects);
        Ok(Engine {
            display: frame,
            objects,
            scene_path,
        })
    }
    pub fn reload(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let objects = loader::load_scene(&self.scene_path, &mut self.display)?;
        println!("{:?}", objects);
        self.objects = objects;
        Ok(())
    }
    pub fn step(&mut self, dt: &std::time::Duration) -> Result<(), Box<dyn std::error::Error>> {
        for object in &mut self.objects {
            if let Some(transform) = &mut object.transform {
                if let Some(rigidbody) = &mut object.rigidbody {
                    systems::physics::gravity(transform, rigidbody, dt);
                }

                if let Some(sprite) = &object.sprite {
                    if let Some(color) = &sprite.color {
                        self.display.draw_color(
                            transform.position.to_array(),
                            transform.scale.to_array(),
                            color.clone(),
                        );
                    } else {
                        self.display.draw_image(crate::frame::Image {
                            position: transform.position.to_array(),
                            scale: transform.scale.to_array(),
                            texture: *sprite
                                .animations
                                .get(sprite.animations.keys().next().unwrap())
                                .unwrap()
                                .first()
                                .unwrap(),
                        })?;
                    }
                }
            }
        }
        self.display.new_frame()
    }
}
