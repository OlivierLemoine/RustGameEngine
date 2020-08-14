use super::prelude::*;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Object {
    pub transform: Option<Transform>,
    pub sprite: Option<Sprite>,
}

#[derive(Deserialize, Debug)]
pub struct Scene {
    objects: Vec<SceneObject>,
}
#[derive(Deserialize, Debug)]
pub struct SceneObject {
    path: String,
    // position:
}
#[derive(Deserialize, Debug)]
pub struct Resources {
    sprite: Sprites,
}
#[derive(Deserialize, Debug)]
pub struct Sprites {
    animations: Vec<Image>,
}
#[derive(Deserialize, Debug)]
pub struct Image {
    name: String,
    path: String,
}

pub fn load_scene(
    path: String,
    frame: &mut crate::frame::Frame,
) -> Result<Vec<Object>, Box<dyn std::error::Error>> {
    let f = std::fs::read_to_string(path)?;
    let scene: Scene = toml::from_str(&f)?;

    let objects = scene
        .objects
        .into_iter()
        .map(|v| {
            let mut image_name_to_index = std::collections::HashMap::<String, Vec<usize>>::new();
            let f = std::fs::read_to_string(v.path).unwrap();
            let mut o: Object = toml::from_str(&f).unwrap();
            let animations = toml::from_str::<Resources>(&f).unwrap().sprite.animations;

            for Image { name, path } in animations {
                let reg = regex::Regex::new("\\{(\\d+)-(\\d+)\\}").unwrap();
                let caps = reg.captures(&path);
                let indices = match caps {
                    Some(caps) => {
                        //
                        let start = caps.get(1).unwrap().as_str().parse::<u32>().unwrap();
                        let end = caps.get(2).unwrap().as_str().parse::<u32>().unwrap();
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

            o.sprite
                .as_mut()
                .map(|mut v| v.animations = image_name_to_index);

            o
        })
        .collect::<Vec<_>>();

    Ok(objects)
}
