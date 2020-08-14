use serde::Deserialize;

#[derive(Deserialize)]
pub struct Config {
    pub window: Window,
    pub font: Font,
    pub shaders: Shaders,
}
#[derive(Deserialize)]
pub struct Window {
    pub name: String,
    pub width: u32,
    pub height: u32,
    pub fps: f32,
}
#[derive(Deserialize)]
pub struct Font {
    pub path: String,
    pub height: u32,
}
#[derive(Deserialize)]
pub struct Shaders {
    pub vertex_path: String,
    pub fragment_path: String,
}
