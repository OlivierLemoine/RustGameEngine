mod cli;
mod config;
mod engine;
mod frame;

use config::Config;
use glium::glutin;
use std::time;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let run_path = if std::env::args().len() > 1 {
        match cli::run()? {
            cli::CLIRes::Stop => {
                return Ok(());
            }
            cli::CLIRes::Run(path) => path,
        }
    } else {
        std::path::PathBuf::from(".")
    };

    let config_path = {
        let mut tmp = run_path.clone();
        tmp.push("./config.toml");
        tmp
    };
    let config_str = std::fs::read_to_string(config_path)?;
    let config: Config = toml::from_str(&config_str)?;

    let events_loop = glium::glutin::event_loop::EventLoop::new();
    let window_builder = glium::glutin::window::WindowBuilder::new()
        .with_decorations(true)
        .with_maximized(true)
        .with_title(config.window.name);
    let context_builder = glium::glutin::ContextBuilder::new();
    let display = glium::Display::new(window_builder, context_builder, &events_loop)?;

    let drawing_params = glium::DrawParameters {
        multisampling: false,
        blend: glium::Blend::alpha_blending(),
        ..Default::default()
    };

    let vertex_shader_src = std::fs::read_to_string("./shaders/vertex_shader.glsl")?;

    let fragment_shader_src = std::fs::read_to_string("./shaders/fragment_shader.glsl")?;
    let program_texture =
        glium::Program::from_source(&display, &vertex_shader_src, &fragment_shader_src, None)?;

    let fragment_color_shader_src =
        std::fs::read_to_string("./shaders/fragment_color_shader.glsl")?;
    let program_color = glium::Program::from_source(
        &display,
        &vertex_shader_src,
        &fragment_color_shader_src,
        None,
    )?;

    let display = frame::Frame::new(
        display,
        drawing_params,
        frame::Program {
            texture: program_texture,
            color: program_color,
        },
        &std::path::PathBuf::from({
            let mut tmp = run_path.clone();
            tmp.push(config.font.path);
            tmp
        }),
    )?;

    let mut timer = time::Instant::now();
    let mut frame_time = time::Duration::new(0, 0);
    let max_frame_time = time::Duration::from_secs_f32(1f32 / config.window.fps);

    // display.load_image(vec!["./resources/animations/guardian/idle/1.png".into()]);

    let scene_path = {
        let mut tmp = run_path.clone();
        tmp.push(config.scene.path);
        tmp
    };

    let mut engine = engine::Engine::new(display, scene_path)?;

    let mut mouse_position = engine::prelude::Vector::zero();

    events_loop.run(move |event, _, control_flow| {
        macro_rules! exit {
            () => {{
                *control_flow = glutin::event_loop::ControlFlow::Exit;
                return;
            }};
        }
        macro_rules! kill_if_error {
            ($e:expr) => {
                match $e {
                    Ok(x) => x,
                    Err(x) => {
                        println!("{}", x);
                        exit!();
                    }
                }
            };
        }

        let dt = timer.elapsed();
        timer = time::Instant::now();
        frame_time += dt;

        match event {
            glutin::event::Event::WindowEvent { event, .. } => match event {
                glutin::event::WindowEvent::CloseRequested => {
                    exit!();
                }
                glutin::event::WindowEvent::KeyboardInput { input, .. } => match input {
                    glutin::event::KeyboardInput {
                        virtual_keycode: Some(glutin::event::VirtualKeyCode::Escape),
                        ..
                    } => exit!(),
                    glutin::event::KeyboardInput {
                        virtual_keycode: Some(glutin::event::VirtualKeyCode::R),
                        ..
                    } => kill_if_error!(engine.reload()),
                    _ => {}
                },
                glutin::event::WindowEvent::MouseInput { button, state, .. } => {
                    match (button, state) {
                        (
                            glutin::event::MouseButton::Left,
                            glutin::event::ElementState::Pressed,
                        ) => {
                            let window_size =
                                engine.display.display.gl_window().window().inner_size();
                            engine.event_pool.push(engine::Event::LeftClickOn(
                                mouse_position
                                    / engine::prelude::Vector::new(
                                        (window_size.width as f32) / 2.0,
                                        (window_size.height as f32) / 2.0,
                                    )
                                    - engine::prelude::Vector::one(),
                            ));
                        }
                        _ => {}
                    }
                }
                glutin::event::WindowEvent::CursorMoved { position, .. } => {
                    mouse_position =
                        engine::prelude::Vector::new(position.x as f32, position.y as f32);
                }
                _ => {}
            },
            _ => {}
        }

        if frame_time > max_frame_time {
            kill_if_error!(engine.step(&frame_time));
            frame_time = time::Duration::new(0, 0);
        }
    });

    // Ok(())
}
