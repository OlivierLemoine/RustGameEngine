mod camera;
mod components;
mod loader;
mod math;
mod object;
mod systems;
mod time;

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
    scene_path: std::path::PathBuf,
    pub event_pool: Vec<Event>,
    libs: std::collections::HashMap<String, libloading::Library>,
    camera: Camera,
    start: std::time::Instant,
}

impl<'a> Engine<'a> {
    pub fn new(
        mut frame: crate::frame::Frame<'a>,
        scene_path: std::path::PathBuf,
    ) -> Result<Engine<'a>, Box<dyn std::error::Error>> {
        let mut libs = std::collections::HashMap::new();
        let objects = loader::load_scene(scene_path.clone(), &mut frame, &mut libs)?;
        Ok(Engine {
            display: frame,
            objects,
            scene_path,
            event_pool: vec![],
            libs,
            camera: Camera::default(),
            start: std::time::Instant::now(),
        })
    }
    pub fn reload(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let mut libs = std::collections::HashMap::new();
        let objects = loader::load_scene(self.scene_path.clone(), &mut self.display, &mut libs)?;
        self.objects = objects;
        self.libs = libs;
        Ok(())
    }
    pub fn step(&mut self, dt: &std::time::Duration) -> Result<(), Box<dyn std::error::Error>> {
        let time = Time {
            delta: dt.clone(),
            start: self.start.clone(),
        };

        let mut events = vec![];
        std::mem::swap(&mut self.event_pool, &mut events);
        for event in events {
            match event {
                Event::LeftClickOn(mut position) => {
                    position -= self.camera.position;

                    for index in (0..self.objects.len()).rev() {
                        if self.collide(self.objects[index].clone(), position)? {
                            break;
                        }
                    }
                }
            }
        }
        for index in 0..self.objects.len() {
            self.obj_step(self.objects[index].clone(), &time)?;
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

        let has_transform = obj.try_borrow()?.transform.is_some();

        if !has_transform {
            return Ok(false);
        }

        let has_collide = {
            let obj = &obj.try_borrow()?;
            let mut p = point.clone();
            if obj.ui {
                p += self.camera.position;
            }
            systems::physics::raycast_normal(&obj.global_transform().unwrap(), &p)
        };

        if has_collide {
            if let Some(Some(f)) = {
                let obj = &*obj.try_borrow()?;
                obj.script.as_ref().map(|s| {
                    let lib = self.libs.get(&s.lib)?;
                    unsafe { lib.get::<fn() -> prelude::OnClick>(b"on_click") }.ok()
                })
            } {
                let f = f();
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
        time: &Time,
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
                    &time.delta,
                );
            }
            if has_sprite {
                let obj = obj.try_borrow()?;
                let transform = obj.global_transform().unwrap();
                let sprite = obj.sprite.as_ref().unwrap();

                if let Some(color) = &sprite.color {
                    let _ = self.display.draw_color(
                        if obj.ui { &NULL_CAMERA } else { &self.camera },
                        transform.position.to_array(),
                        transform.scale.to_array(),
                        color.clone(),
                    );
                } else {
                    let _ = self.display.draw_image(
                        if obj.ui { &NULL_CAMERA } else { &self.camera },
                        crate::frame::Image {
                            position: transform.position.to_array(),
                            scale: transform.scale.to_array(),
                            texture: *sprite
                                .animations
                                .get(sprite.animations.keys().next().unwrap())
                                .unwrap()
                                .first()
                                .unwrap(),
                        },
                    );
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
                unsafe { lib.get::<fn() -> prelude::Update>(b"update") }.ok()
            } {
                let f = f();
                f(obj, &mut self.camera, time)
            }
        }

        let children = obj.try_borrow()?.children.clone();

        for c in children {
            self.obj_step(c.clone(), time)?;
        }

        Ok(())
    }
}
