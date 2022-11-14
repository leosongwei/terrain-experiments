mod assets;
mod backend_gl;
mod camera;
mod mesh;
mod renderer;
mod shader;
mod terrain;

use std::collections::HashMap;

use glam::{Vec2, Vec3};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::mouse::{MouseButton, RelativeMouseState};
use sdl2::video::GLProfile;

use simple_logger::SimpleLogger;

use crate::camera::SimpleCamera;
use crate::shader::ShaderProgram;

fn main() {
    SimpleLogger::new().with_colors(true).init().unwrap();
    log::info!("start.");

    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let gl_attr = video_subsystem.gl_attr();
    gl_attr.set_context_profile(GLProfile::Core);
    gl_attr.set_context_version(4, 5);

    let window = video_subsystem
        .window("Window", 800, 600)
        .opengl()
        .build()
        .unwrap();

    // Unlike the other example above, nobody created a context for your window, so you need to create one.
    let _ctx = window.gl_create_context().unwrap();
    gl::load_with(|name| video_subsystem.gl_get_proc_address(name) as *const _);

    debug_assert_eq!(gl_attr.context_profile(), GLProfile::Core);
    debug_assert_eq!(gl_attr.context_version(), (4, 5));

    let mut event_pump = sdl_context.event_pump().unwrap();

    let vs = std::fs::read_to_string("src/vs.glsl").unwrap();
    let fs = std::fs::read_to_string("src/fs.glsl").unwrap();
    let uniforms = vec!["view_projection", "model"];
    let shader_program = ShaderProgram::from_shader_strings(&vs, &fs, uniforms).unwrap();

    let mut mesh = mesh::Mesh::new(vec![
        mesh::Vertex::new(
            0,
            0,
            0,
            Vec3::new(0.5, -0.5, 0.0),
            Vec3::new(0.0, 0.0, 1.0),
            [255, 0, 0, 255],
            Vec2::new(0.3, 0.0),
        ),
        mesh::Vertex::new(
            0,
            0,
            0,
            Vec3::new(-0.5, -0.5, 0.0),
            Vec3::new(0.0, 0.0, 1.0),
            [0, 255, 0, 255],
            Vec2::new(0.3, 0.0),
        ),
        mesh::Vertex::new(
            0,
            0,
            0,
            Vec3::new(0.0, 0.5, 0.0),
            Vec3::new(0.0, 0.0, 1.0),
            [0, 0, 255, 255],
            Vec2::new(0.3, 0.0),
        ),
    ]);

    mesh.load_without_ebo();
    mesh.bind();

    let render_function = || {
        shader_program.use_program();
        mesh.bind();
        mesh.draw();
    };

    let mut renderer = renderer::Renderer::new(
        window,
        SimpleCamera::new(
            Vec3::new(0f32, 0f32, 2f32),
            Vec3::new(0f32, 0f32, 0f32),
            (50.0 / 360.0) * 2.0 * 3.14,
            4.0 / 3.0,
            0.01,
        ),
    );

    let mut mouse_captured = false;

    'running: loop {
        if mouse_captured {
            let relative: RelativeMouseState = event_pump.relative_mouse_state();
            let (width, height) = renderer.window.drawable_size();
            let rotation = Vec3::new(
                relative.y() as f32 / height as f32,
                relative.x() as f32 / width as f32,
                0f32,
            ) * 0.4;
            renderer.main_camera.rotate_camera(rotation);
        }

        if event_pump
            .keyboard_state()
            .is_scancode_pressed(sdl2::keyboard::Scancode::W)
        {
            renderer
                .main_camera
                .move_camera(Vec3::new(0f32, 0f32, -0.05));
        }
        if event_pump
            .keyboard_state()
            .is_scancode_pressed(sdl2::keyboard::Scancode::S)
        {
            renderer
                .main_camera
                .move_camera(Vec3::new(0f32, 0f32, 0.05));
        }
        if event_pump
            .keyboard_state()
            .is_scancode_pressed(sdl2::keyboard::Scancode::A)
        {
            renderer
                .main_camera
                .move_camera(Vec3::new(-0.05, 0f32, 0f32));
        }
        if event_pump
            .keyboard_state()
            .is_scancode_pressed(sdl2::keyboard::Scancode::D)
        {
            renderer
                .main_camera
                .move_camera(Vec3::new(0.05, 0f32, 0f32));
        }
        if event_pump
            .keyboard_state()
            .is_scancode_pressed(sdl2::keyboard::Scancode::LShift)
        {
            renderer
                .main_camera
                .move_camera(Vec3::new(0f32, 0.05, 0f32));
        }
        if event_pump
            .keyboard_state()
            .is_scancode_pressed(sdl2::keyboard::Scancode::LCtrl)
        {
            renderer
                .main_camera
                .move_camera(Vec3::new(0f32, -0.05, 0f32));
        }

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => break 'running,
                Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => {
                    if !mouse_captured {
                        break 'running;
                    } else {
                        mouse_captured = false;
                        sdl_context.mouse().set_relative_mouse_mode(false);
                    }
                }
                Event::MouseButtonDown {
                    mouse_btn: MouseButton::Left,
                    ..
                } => {
                    mouse_captured = true;
                    sdl_context.mouse().set_relative_mouse_mode(true);
                }
                _ => {}
            }
        }

        shader_program.set_uniforms(HashMap::from([(
            "view_projection",
            shader::ShaderParam::Mat4(renderer.main_camera.get_view_mat()),
        )]));

        renderer.render(&render_function);

        ::std::thread::sleep(::std::time::Duration::new(0, 1_000_000_000u32 / 60));
    }
    mesh.unload();
    log::info!("quit");
}
