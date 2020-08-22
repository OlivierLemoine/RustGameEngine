use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Sprite {
    pub depth: u32,
    pub color: Option<[f32; 4]>,
    #[serde(skip)]
    pub animations: std::collections::HashMap<String, Vec<usize>>,
}
