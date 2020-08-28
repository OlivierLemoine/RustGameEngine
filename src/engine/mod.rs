mod components;
mod loader;
mod math;
mod object;
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
    libs: std::collections::HashMap<String, libloading::Library>,
}

impl<'a> Engine<'a> {
    pub fn new(
        mut frame: crate::frame::Frame<'a>,
        scene_path: String,
    ) -> Result<Engine<'a>, Box<dyn std::error::Error>> {
        let mut libs = std::collections::HashMap::new();
        let objects = loader::load_scene(&scene_path, &mut frame, &mut libs)?;
        Ok(Engine {
            display: frame,
            objects,
            scene_path,
            event_pool: vec![],
            libs,
        })
    }
    pub fn reload(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let mut libs = std::collections::HashMap::new();
        let objects = loader::load_scene(&self.scene_path, &mut self.display, &mut libs)?;
        self.objects = objects;
        self.libs = libs;
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
        for child in obj.try_borrow()?.children.iter().map(|v| v.clone()) {
            if self.collide(child, point)? {
                return Ok(true);
            }
        }

        if systems::physics::raycast_normal(&obj.try_borrow()?.global_transform().unwrap(), &point)
        {
            if let Some(Some(f)) = {
                let obj = &*obj.try_borrow()?;
                obj.script.as_ref().map(|s| {
                    let lib = self.libs.get(&s.lib)?;
                    unsafe { lib.get::<prelude::OnClick>(b"on_click") }.ok()
                })
            } {
                f(&mut *obj.try_borrow_mut()?);
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
        let has_script = obj.try_borrow()?.script.is_some();

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
                    let _ = self.display.draw_image(crate::frame::Image {
                        position: transform.position.to_array(),
                        scale: transform.scale.to_array(),
                        texture: *sprite
                            .animations
                            .get(sprite.animations.keys().next().unwrap())
                            .unwrap()
                            .first()
                            .unwrap(),
                    });
                }
            }
        }

        if has_script {
            let obj = &mut *obj.try_borrow_mut()?;
            if let Some(f) = {
                let lib = self
                    .libs
                    .get(&obj.script.as_ref().unwrap().lib)
                    .ok_or(format!("Unknown lib {}", obj.script.as_ref().unwrap().lib))?;
                unsafe { lib.get::<prelude::Update>(b"update") }.ok()
            } {
                f(obj)
            }
        }

        let children = obj.try_borrow()?.children.clone();

        for c in children {
            self.obj_step(c.clone(), dt)?;
        }

        Ok(())
    }
}
