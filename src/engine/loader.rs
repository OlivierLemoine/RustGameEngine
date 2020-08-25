use super::prelude::*;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Object {
    pub transform: Option<Transform>,
    pub sprite: Option<Sprite>,
    pub rigidbody: Option<Rigidbody>,
    pub script: Option<Script>,
    #[serde(skip)]
    pub parent: Option<usize>,
    #[serde(skip)]
    pub children: Vec<usize>,
}

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
) -> Result<Vec<Object>, Box<dyn std::error::Error>> {
    let f = std::fs::read_to_string(path)?;
    let scene: Scene = toml::from_str(&f)?;

    let mut objects = vec![];

    for o in scene.objects {
        objects = load_object(&o.path, objects, None, frame)?;
    }

    Ok(objects)
}

pub fn load_object(
    path: &str,
    mut objects: Vec<Object>,
    parent: Option<usize>,
    frame: &mut crate::frame::Frame,
) -> Result<Vec<Object>, Box<dyn std::error::Error>> {
    let file = std::fs::read_to_string(path)?;

    let mut object: Object = toml::from_str(&file)?;
    object.parent = parent;

    let index = objects.len();

    objects.push(object);

    let builder: Builder = toml::from_str(&file)?;

    if let Some(cs) = builder
        .children
        .map(|c| c.scene.map(|s| s.objects).or(c.objects))
        .flatten()
    {
        for c in cs {
            let i = objects.len();
            objects[index].children.push(i);
            objects = load_object(&c.path, objects, Some(index), frame)?;
        }
    }

    if let Some(s) = builder.script {
        let lib = libloading::Library::new(s.path)?;
        unsafe {
            let f: libloading::Symbol<unsafe extern "C" fn()> = lib.get(b"on_click")?;
            f();
        }
    }

    // builder.sprite.map(|s| {
    //     s.animations.map(|a| {
    //         let mut image_name_to_index = std::collections::HashMap::<String, Vec<usize>>::new();
    //         for Image { name, path } in a {
    //             let reg = regex::Regex::new("\\{(\\d+)-(\\d+)\\}").ok()?;
    //             let caps = reg.captures(&path);
    //             let indices = match caps {
    //                 Some(caps) => {
    //                     //
    //                     let start = caps.get(1)?.as_str().parse::<u32>().ok()?;
    //                     let end = caps.get(2)?.as_str().parse::<u32>().ok()?;
    //                     // println!("{} - {}", start, end);

    //                     let paths = (start..=end)
    //                         .map(|v| {
    //                             path.clone()
    //                                 .replace(&format!("{{{}-{}}}", start, end), &v.to_string())
    //                         })
    //                         .collect();

    //                     frame.load_image(paths)
    //                 }
    //                 None => frame.load_image(vec![path]),
    //             };
    //             image_name_to_index.insert(name, indices);
    //         }
    //         objects[index]
    //             .sprite
    //             .map(|mut s| s.animations = image_name_to_index);
    //         Some(())
    //     })
    // });

    Ok(objects)
}
