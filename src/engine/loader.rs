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
}
#[derive(Deserialize, Debug)]
pub struct Sprites {
    animations: Option<Vec<Image>>,
}
#[derive(Deserialize, Debug)]
pub struct ScriptLoader {
    path: String,
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
) -> Result<Vec<Rc<RefCell<Object>>>, Box<dyn std::error::Error>> {
    let f = std::fs::read_to_string(path)?;
    let scene: Scene = toml::from_str(&f)?;

    let mut objects = vec![];

    for o in scene.objects {
        objects.push(load_object(&o.path, None, frame)?);
    }

    Ok(objects)
}

pub fn load_object(
    path: &str,
    parent: Option<Weak<RefCell<Object>>>,
    frame: &mut crate::frame::Frame,
) -> Result<Rc<RefCell<Object>>, Box<dyn std::error::Error>> {
    let file = std::fs::read_to_string(path)?;

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
                self_obj
                    .children
                    .push(load_object(&c.path, Some(self_ref.clone()), frame)?)
            }
        }

        if let Some(s) = builder.script {
            let lib = libloading::Library::new(s.path)?;
            self_obj.script = Some(Script { lib: Some(lib) })
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
                                    path.clone()
                                        .replace(&format!("{{{}-{}}}", start, end), &v.to_string())
                                })
                                .collect();

                            frame.load_image(paths)
                        }
                        None => frame.load_image(vec![path]),
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
