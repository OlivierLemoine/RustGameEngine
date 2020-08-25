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

#[derive(Debug, Copy, Clone)]
pub enum Event {
    LeftClickOn(math::Vector),
    // LeftClickOff(math::Vector),
}

pub struct Engine<'a> {
    pub display: crate::frame::Frame<'a>,
    objects: Vec<loader::Object>,
    scene_path: String,
    pub event_pool: Vec<Event>,
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
            event_pool: vec![],
        })
    }
    pub fn reload(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let objects = loader::load_scene(&self.scene_path, &mut self.display)?;
        println!("{:?}", objects);
        self.objects = objects;
        Ok(())
    }
    pub fn step(&mut self, dt: &std::time::Duration) -> Result<(), Box<dyn std::error::Error>> {
        for event in self.event_pool.drain(0..) {
            match event {
                Event::LeftClickOn(position) => {
                    // self.display.
                    println!("{:?}", position);
                }
            }
        }
        for index in 0..self.objects.len() {
            if self.objects[index].transform.is_some() {
                if self.objects[index].rigidbody.is_some() {
                    let obj = &mut self.objects[index];
                    systems::physics::gravity(
                        obj.transform.as_mut().unwrap(),
                        obj.rigidbody.as_mut().unwrap(),
                        dt,
                    );
                }
                if self.objects[index].sprite.is_some() {
                    let obj = &self.objects[index];
                    let scale = obj.transform.as_ref().unwrap().scale;
                    let mut position = obj.transform.as_ref().unwrap().position;

                    let mut parent_index = obj.parent;
                    while let Some(parent) = (|| Some(&self.objects[parent_index?]))() {
                        parent.transform.as_ref().map(|t| position += t.position);
                        parent_index = parent.parent;
                    }

                    let sprite = obj.sprite.as_ref().unwrap();

                    if let Some(color) = &sprite.color {
                        self.display.draw_color(
                            position.to_array(),
                            scale.to_array(),
                            color.clone(),
                        );
                    } else {
                        self.display.draw_image(crate::frame::Image {
                            position: position.to_array(),
                            scale: scale.to_array(),
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
