use serde::Deserialize;

#[derive(Deserialize)]
pub struct Config {
    pub window: Window,
    pub font: Font,
    pub shaders: Shaders,
    pub scene: Scene,
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
    pub fragment_color_path: String,
}
#[derive(Deserialize)]
pub struct ViewSize {
    pub x_min: f32,
    pub y_min: f32,
    pub x_max: f32,
    pub y_max: f32,
}
#[derive(Deserialize)]
pub struct Scene {
    pub path: String,
    pub view_size: ViewSize,
}
