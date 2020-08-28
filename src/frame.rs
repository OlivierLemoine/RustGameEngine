use glium::{implement_vertex, uniform, Surface};
use std::sync::{Arc, Mutex};

#[derive(Copy, Clone)]
pub struct Vertex {
    position: [f32; 2],
    tex_coords: [f32; 2],
}

implement_vertex!(Vertex, position, tex_coords);

pub const INDICES: [u16; 6] = [0, 1, 2, 1, 3, 2];
pub const SQUARE: [Vertex; 4] = [
    Vertex {
        position: [-1.0, -1.0],
        tex_coords: [0.0, 0.0],
    },
    Vertex {
        position: [-1.0, 1.0],
        tex_coords: [0.0, 1.0],
    },
    Vertex {
        position: [1.0, -1.0],
        tex_coords: [1.0, 0.0],
    },
    Vertex {
        position: [1.0, 1.0],
        tex_coords: [1.0, 1.0],
    },
];
pub struct Image {
    pub position: [f32; 2],
    pub scale: [f32; 2],
    pub texture: usize,
}

pub struct Program {
    pub texture: glium::Program,
    pub color: glium::Program,
}

pub struct Frame<'a> {
    pub display: glium::Display,
    parameters: glium::DrawParameters<'a>,
    program: Program,
    frame: glium::Frame,
    vertex_buffer: glium::VertexBuffer<Vertex>,
    indices_buffer: glium::index::IndexBuffer<u16>,
    images_to_load: Arc<Mutex<Vec<std::path::PathBuf>>>,
    images_to_add: Arc<Mutex<Vec<image::ImageBuffer<image::Rgba<u8>, Vec<u8>>>>>,
    done_loading: Arc<Mutex<bool>>,
    images: Vec<glium::texture::Texture2d>,
    next_index: usize,
    current_frame_dim: (u32, u32),
    pub view_offset: [f32; 2],
    pub view_scale: [f32; 2],
}

impl<'a> Frame<'a> {
    pub fn new(
        display: glium::Display,
        parameters: glium::DrawParameters<'a>,
        program: Program,
        view_size: super::config::ViewSize,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let vertex_buffer = glium::VertexBuffer::<Vertex>::new(&display, &SQUARE)?;
        let indices_buffer = glium::index::IndexBuffer::new(
            &display,
            glium::index::PrimitiveType::TrianglesList,
            &INDICES,
        )?;
        let mut frame = display.draw();
        frame.clear_color(1.0, 1.0, 1.0, 1.0);

        let current_frame_dim = display.get_framebuffer_dimensions();

        Ok(Frame {
            parameters,
            display,
            program,
            frame,
            vertex_buffer,
            indices_buffer,
            images_to_load: Arc::new(Mutex::new(vec![])),
            images_to_add: Arc::new(Mutex::new(vec![])),
            done_loading: Arc::new(Mutex::new(true)),
            images: vec![],
            next_index: 0,
            current_frame_dim,
            view_offset: [
                (view_size.x_max + view_size.x_min) / 2.0,
                (view_size.y_max + view_size.y_min) / 2.0,
            ],
            view_scale: [
                2.0 / (view_size.x_max - view_size.x_min),
                2.0 / (view_size.y_max - view_size.y_min),
            ],
        })
    }
    pub fn load_image(&mut self, mut paths: Vec<std::path::PathBuf>) -> Vec<usize> {
        let nb_new_images = paths.len();
        {
            self.images_to_load.lock().unwrap().append(&mut paths);
        }
        if *self.done_loading.lock().unwrap() {
            {
                *self.done_loading.lock().unwrap() = false;
            }
            let images_to_load_ref = Arc::clone(&self.images_to_load);
            let images_to_add_ref = Arc::clone(&self.images_to_add);
            let done_loading_ref = Arc::clone(&self.done_loading);
            std::thread::spawn(move || {
                while { images_to_load_ref.lock().unwrap().len() } > 0 {
                    let path = { images_to_load_ref.lock().unwrap().drain(0..1) }
                        .next()
                        .unwrap();
                    println!("Loading image {:?}", path);
                    let img = image::load(
                        std::io::Cursor::new(std::fs::read(path).unwrap()),
                        image::ImageFormat::Png,
                    )
                    .unwrap()
                    .to_rgba();
                    {
                        images_to_add_ref.lock().unwrap().push(img);
                    }
                }
                {
                    *done_loading_ref.lock().unwrap() = true;
                }
            });
        }

        let start_index = self.next_index;
        self.next_index += nb_new_images;
        (start_index..(start_index + nb_new_images)).collect()
    }
    pub fn draw_color(
        &mut self,
        position: [f32; 2],
        scale: [f32; 2],
        color: [f32; 4],
    ) -> Result<(), Box<dyn std::error::Error>> {
        self.frame.draw(
            &self.vertex_buffer,
            &self.indices_buffer,
            &self.program.color,
            &uniform! {
                obj_position: position,
                obj_scale: scale,
                c: color,
                window_ratio: if self.current_frame_dim.1 != 0 {self.current_frame_dim.0 / self.current_frame_dim.1} else {1},
                view_offset: self.view_offset,
                view_scale: self.view_scale,
            },
            &self.parameters,
        )?;
        Ok(())
    }
    pub fn draw_image(&mut self, image: Image) -> Result<(), Box<dyn std::error::Error>> {
        println!("{:?}", self.display.get_framebuffer_dimensions());
        let img = self
            .images
            .get(image.texture)
            .ok_or(format!("Image index {} does not exists", image.texture))?;
        self.frame.draw(
            &self.vertex_buffer,
            &self.indices_buffer,
            &self.program.texture,
            &uniform! {
                obj_position: image.position,
                obj_scale: image.scale,
                tex: img,
                window_ratio: if self.current_frame_dim.1 != 0 {self.current_frame_dim.0 / self.current_frame_dim.1} else {1},
                view_offset: self.view_offset,
                view_scale: self.view_scale,
            },
            &self.parameters,
        )?;
        Ok(())
    }
    pub fn new_frame(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.frame.set_finish()?;

        self.current_frame_dim = self.display.get_framebuffer_dimensions();

        while let Some(img) = self.images_to_add.lock().unwrap().pop() {
            let dims = img.dimensions();
            let image = glium::texture::RawImage2d::from_raw_rgba_reversed(&img.into_raw(), dims);
            self.images
                .push(glium::texture::Texture2d::new(&self.display, image).unwrap());
        }

        self.frame = self.display.draw();
        self.frame.clear_color(1.0, 1.0, 1.0, 1.0);
        Ok(())
    }
}
impl<'a> Drop for Frame<'a> {
    fn drop(&mut self) {
        let _ = self.frame.set_finish();
    }
}
