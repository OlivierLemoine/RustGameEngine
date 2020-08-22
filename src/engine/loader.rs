use super::prelude::*;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Object {
    pub transform: Option<Transform>,
    pub sprite: Option<Sprite>,
    pub rigidbody: Option<Rigidbody>,
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
    path: &str,
    frame: &mut crate::frame::Frame,
) -> Result<Vec<Object>, Box<dyn std::error::Error>> {
    let f = std::fs::read_to_string(path)?;
    let scene: Scene = toml::from_str(&f)?;

    let objects = scene
        .objects
        .into_iter()
        .map(|v: SceneObject| {
            let mut image_name_to_index = std::collections::HashMap::<String, Vec<usize>>::new();
            let f = std::fs::read_to_string(v.path)?;
            let mut o: Object = toml::from_str(&f)?;
            if let Some(sprite) = &mut o.sprite {
                if sprite.color.is_none() {
                    let animations = toml::from_str::<Resources>(&f)?.sprite.animations;

                    for Image { name, path } in animations {
                        let reg = regex::Regex::new("\\{(\\d+)-(\\d+)\\}")?;
                        let caps = reg.captures(&path);
                        let indices = match caps {
                            Some(caps) => {
                                //
                                let start = caps.get(1).unwrap().as_str().parse::<u32>()?;
                                let end = caps.get(2).unwrap().as_str().parse::<u32>()?;
                                // println!("{} - {}", start, end);

                                let paths = (start..=end)
                                    .map(|v| {
                                        path.clone().replace(
                                            &format!("{{{}-{}}}", start, end),
                                            &v.to_string(),
                                        )
                                    })
                                    .collect();

                                frame.load_image(paths)
                            }
                            None => frame.load_image(vec![path]),
                        };
                        image_name_to_index.insert(name, indices);
                    }

                    sprite.animations = image_name_to_index;
                }
            }

            Ok(o)
        })
        .collect::<Result<Vec<_>, Box<dyn std::error::Error>>>()?;

    Ok(objects)
}
