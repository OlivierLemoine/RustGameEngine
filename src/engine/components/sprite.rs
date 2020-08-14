use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Sprite {
    pub depth: u32,
    #[serde(skip)]
    pub animations: std::collections::HashMap<String, Vec<usize>>,
}
