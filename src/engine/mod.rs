mod components;
mod loader;
mod math;
mod systems;

pub mod prelude;

use prelude::*;

#[derive(Debug, Copy, Clone)]
pub enum Event {
    LeftClickOn(Vector),
    // LeftClickOff(math::Vector),
}

pub struct Engine<'a> {
    pub display: crate::frame::Frame<'a>,
    objects: Vec<Rc<RefCell<Object>>>,
    scene_path: String,
    pub event_pool: Vec<Event>,
}

impl<'a> Engine<'a> {
    pub fn new(
        mut frame: crate::frame::Frame<'a>,
        scene_path: String,
    ) -> Result<Engine<'a>, Box<dyn std::error::Error>> {
        let objects = loader::load_scene(&scene_path, &mut frame)?;
        Ok(Engine {
            display: frame,
            objects,
            scene_path,
            event_pool: vec![],
        })
    }
    pub fn reload(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let objects = loader::load_scene(&self.scene_path, &mut self.display)?;
        self.objects = objects;
        Ok(())
    }
    pub fn step(&mut self, dt: &std::time::Duration) -> Result<(), Box<dyn std::error::Error>> {
        let mut events = vec![];
        std::mem::swap(&mut self.event_pool, &mut events);
        for event in events {
            match event {
                Event::LeftClickOn(position) => {
                    let scale = prelude::Vector::from(self.display.view_scale);
                    let offset = prelude::Vector::from(self.display.view_offset);
                    let position = position * scale - offset;

                    for index in (0..self.objects.len()).rev() {
                        if self.collide(self.objects[index].clone(), position)? {
                            break;
                        }
                    }
                }
            }
        }
        for index in 0..self.objects.len() {
            self.obj_step(self.objects[index].clone(), dt)?;
        }
        self.display.new_frame()
    }
    fn collide(
        &self,
        obj: Rc<RefCell<Object>>,
        point: Vector,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        if systems::physics::raycast_normal(obj.try_borrow()?.transform.as_ref().unwrap(), &point) {
            let new_point = {
                let obj = &*obj.try_borrow()?;
                let transform = obj.transform.as_ref().unwrap();
                (point - transform.position) / transform.scale
            };
            let mut has_child_collide = false;

            for child in obj.try_borrow()?.children.iter().map(|v| v.clone()) {
                has_child_collide = self.collide(child, new_point)?;
            }

            if !has_child_collide {
                if let Some(lib) = obj
                    .try_borrow()?
                    .script
                    .as_ref()
                    .map(|s| s.lib.as_ref())
                    .flatten()
                {
                    let f = unsafe { lib.get::<prelude::OnClick>(b"on_click") }?;
                    f();
                }
            }

            Ok(true)
        } else {
            Ok(false)
        }
    }
    fn obj_step(
        &mut self,
        obj: Rc<RefCell<Object>>,
        dt: &std::time::Duration,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let has_transform = obj.try_borrow()?.transform.is_some();
        let has_rigidbody = obj.try_borrow()?.rigidbody.is_some();
        let has_sprite = obj.try_borrow()?.sprite.is_some();

        if has_transform {
            if has_rigidbody {
                let obj = &mut *obj.try_borrow_mut()?;
                systems::physics::gravity(
                    obj.transform.as_mut().unwrap(),
                    obj.rigidbody.as_mut().unwrap(),
                    dt,
                );
            }
            if has_sprite {
                let obj = obj.try_borrow()?;
                let transform = obj.global_transform().unwrap();
                let sprite = obj.sprite.as_ref().unwrap();

                if let Some(color) = &sprite.color {
                    let _ = self.display.draw_color(
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

        let children = obj.try_borrow()?.children.clone();

        for c in children {
            self.obj_step(c.clone(), dt)?;
        }

        Ok(())
    }
}
