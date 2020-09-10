use super::prelude::*;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Scene {
    #[serde(default)]
    objects: Vec<SceneObject>,
}
#[derive(Deserialize, Debug)]
pub struct SceneObject {
    path: String,
    // position:
}
#[derive(Deserialize, Debug)]
pub struct Builder {
    sprite: Option<Sprites>,
    children: Option<Children>,
    script: Option<ScriptLoader>,
    custom: Option<toml::Value>,
}
#[derive(Deserialize, Debug)]
pub struct Sprites {
    animations: Option<Vec<Image>>,
}
#[derive(Deserialize, Debug)]
pub struct ScriptLoader {
    path: String,
    name: String,
}
#[derive(Deserialize, Debug)]
pub struct Children {
    objects: Option<Vec<SceneObject>>,
    scene: Option<Scene>,
}
#[derive(Deserialize, Debug)]
pub struct Image {
    name: String,
    path: String,
}

pub fn load_scene(
    path: &str,
    frame: &mut crate::frame::Frame,
    libs: &mut std::collections::HashMap<String, libloading::Library>,
) -> Result<Vec<Rc<RefCell<Object>>>, Box<dyn std::error::Error>> {
    let mut path = std::path::Path::new(path).to_owned();

    let f = std::fs::read_to_string(&path)?;
    let scene: Scene = toml::from_str(&f)?;

    let mut objects = vec![];

    path.pop();

    for o in scene.objects {
        let mut obj_path = path.clone();
        obj_path.push(o.path);
        objects.push(load_object(obj_path, None, frame, libs)?);
    }

    Ok(objects)
}

pub fn load_object(
    mut obj_path: std::path::PathBuf,
    parent: Option<Weak<RefCell<Object>>>,
    frame: &mut crate::frame::Frame,
    libs: &mut std::collections::HashMap<String, libloading::Library>,
) -> Result<Rc<RefCell<Object>>, Box<dyn std::error::Error>> {
    let file = std::fs::read_to_string(&obj_path)?;
    obj_path.pop();

    let mut object: Object = toml::from_str(&file)?;
    object.parent = parent;

    let object = Rc::new(RefCell::new(object));
    {
        let self_ref = Rc::downgrade(&object);
        let self_obj = &mut *object.try_borrow_mut()?;

        let builder: Builder = toml::from_str(&file)?;

        if let Some(cs) = builder
            .children
            .map(|c| c.scene.map(|s| s.objects).or(c.objects))
            .flatten()
        {
            for c in cs {
                let mut child_path = obj_path.clone();
                child_path.push(c.path);
                self_obj.children.push(load_object(
                    child_path,
                    Some(self_ref.clone()),
                    frame,
                    libs,
                )?)
            }
        }

        if let Some(s) = builder.script {
            let mut lib_path = obj_path.clone();

            lib_path.push(s.path);
            lib_path.push("target");

            if cfg!(debug_assertions) {
                lib_path.push("debug");
            } else {
                lib_path.push("release");
            }

            if cfg!(target_os = "windows") {
                lib_path.push(format!("{}.dll", s.name));
            } else {
                unimplemented!()
            }

            let lib_path_string = lib_path
                .clone()
                .into_os_string()
                .into_string()
                .map_err(|_| "Could not load lib")?;

            let lib = libloading::Library::new(lib_path)?;

            if let Some(c) = builder.custom {
                if let Some(f) =
                    unsafe { lib.get::<fn() -> ParseCustomObject>(b"parse_custom_object") }.ok()
                {
                    let f = f();
                    self_obj.custom = Some(f(&format!("{}", c))?);
                }
            }

            libs.insert(lib_path_string.clone(), lib);
            self_obj.script = Some(Script {
                lib: lib_path_string,
            })
        }

        builder.sprite.map(|s| {
            s.animations.map(|a| {
                let mut image_name_to_index =
                    std::collections::HashMap::<String, Vec<usize>>::new();
                for Image { name, path } in a {
                    let reg = regex::Regex::new("\\{(\\d+)-(\\d+)\\}").ok()?;
                    let caps = reg.captures(&path);
                    let indices = match caps {
                        Some(caps) => {
                            //
                            let start = caps.get(1)?.as_str().parse::<u32>().ok()?;
                            let end = caps.get(2)?.as_str().parse::<u32>().ok()?;
                            // println!("{} - {}", start, end);

                            let paths = (start..=end)
                                .map(|v| {
                                    let mut image_path = obj_path.clone();
                                    image_path.push(path.clone().replace(
                                        &format!("{{{}-{}}}", start, end),
                                        &v.to_string(),
                                    ));
                                    image_path
                                })
                                .collect();

                            frame.load_image(paths)
                        }
                        None => {
                            let mut image_path = obj_path.clone();
                            image_path.push(path);
                            frame.load_image(vec![image_path])
                        }
                    };
                    image_name_to_index.insert(name, indices);
                }
                self_obj
                    .sprite
                    .as_mut()
                    .map(|mut s| s.animations = image_name_to_index);
                Some(())
            })
        });
    }

    Ok(object)
}
