use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Sprite {
    #[serde(flatten)]
    pub ty: SpriteType,
}

#[derive(Deserialize, Debug)]
#[serde(tag = "type")]
pub enum SpriteType {
    Animation {
        #[serde(skip)]
        animations: std::collections::HashMap<String, Vec<usize>>,
    },
    Circle {
        #[serde(default = "default_color")]
        color: [f32; 4],
    },
    Rect {
        #[serde(default = "default_color")]
        color: [f32; 4],
    },
    Text {
        text: String,
        #[serde(default = "default_color")]
        color: [f32; 4],
    },
}

fn default_color() -> [f32; 4] {
    [0.0, 0.0, 0.0, 1.0]
}
