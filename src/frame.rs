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

pub struct Frame<'a> {
    display: glium::Display,
    parameters: glium::DrawParameters<'a>,
    program: glium::Program,
    frame: glium::Frame,
    vertex_buffer: glium::VertexBuffer<Vertex>,
    indices_buffer: glium::index::IndexBuffer<u16>,
    images_to_load: Arc<Mutex<Vec<String>>>,
    images_to_add: Arc<Mutex<Vec<image::ImageBuffer<image::Rgba<u8>, Vec<u8>>>>>,
    done_loading: Arc<Mutex<bool>>,
    images: Vec<glium::texture::Texture2d>,
    next_index: usize,
}

impl<'a> Frame<'a> {
    pub fn new(
        display: glium::Display,
        parameters: glium::DrawParameters<'a>,
        program: glium::Program,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let vertex_buffer = glium::VertexBuffer::<Vertex>::new(&display, &SQUARE)?;
        let indices_buffer = glium::index::IndexBuffer::new(
            &display,
            glium::index::PrimitiveType::TrianglesList,
            &INDICES,
        )?;
        let mut frame = display.draw();
        frame.clear_color(1.0, 1.0, 1.0, 1.0);

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
        })
    }
    pub fn load_image(&mut self, mut paths: Vec<String>) -> Vec<usize> {
        let nb_new_images = paths.len();
        {
            self.images_to_load.lock().unwrap().append(&mut paths);
        }
        if { *self.done_loading.lock().unwrap() } {
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
                    println!("Loading image {}", path);
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
    pub fn draw_image(&mut self, image: Image) -> Result<(), Box<dyn std::error::Error>> {
        let img = self
            .images
            .get(image.texture)
            .ok_or(format!("Image index {} does not exists", image.texture))?;
        self.frame.draw(
            &self.vertex_buffer,
            &self.indices_buffer,
            &self.program,
            &uniform! {
                obj_position: image.position,
                obj_scale: image.scale,
                tex: img,
                // window_size: window_size,
            },
            &self.parameters,
        )?;
        Ok(())
    }
    pub fn new_frame(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.frame.set_finish()?;

        while let Some(img) = { self.images_to_add.lock().unwrap().pop() } {
            let dims = img.dimensions();
            let image = glium::texture::RawImage2d::from_raw_rgba_reversed(&img.into_raw(), dims);
            self.images
                .push(glium::texture::Texture2d::new(&self.display, image).unwrap());
        }

        self.frame = self.display.draw();
        self.frame.clear_color(1.0, 1.0, 1.0, 1.0);
        Ok(())
    }
    pub fn kill(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.frame.set_finish()?;
        Ok(())
    }
}
