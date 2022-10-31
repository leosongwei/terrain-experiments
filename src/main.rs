mod shader;
mod mesh;
mod camera;
mod renderer;

use glam::{Vec3, Vec2};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
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

    let window = video_subsystem.window("Window", 800, 600)
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
    let shader_program = ShaderProgram::from_shader_strings(&vs, &fs).unwrap();

    let mut mesh = mesh::Mesh::new(vec![
        mesh::Vertex::new(0, Vec3::new(0.5, -0.5, 0.0), Vec3::new(0.0, 0.0, 1.0), [255, 0, 0, 255], Vec2::new(0.3, 0.0)),
        mesh::Vertex::new(0, Vec3::new(-0.5, -0.5, 0.0), Vec3::new(0.0, 0.0, 1.0), [0, 255, 0, 255], Vec2::new(0.3, 0.0)),
        mesh::Vertex::new(0, Vec3::new(0.0, 0.5, 0.0), Vec3::new(0.0, 0.0, 1.0), [0, 0, 255, 255], Vec2::new(0.3, 0.0)),
    ]);

    mesh.load_without_ebo();
    mesh.bind();

    let render_function = || {
        shader_program.use_program();
            mesh.bind();
            mesh.draw();
        };

    let mut renderer = renderer::Renderer::new(window, SimpleCamera::new(Vec3::new(0f32, 0f32, 0f32), Vec3::new(0f32, 0f32, 0f32)));


    'running: loop {

        renderer.render(&render_function);
        
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                _ => {}
            }
        }
        ::std::thread::sleep(::std::time::Duration::new(0, 1_000_000_000u32 / 60));
    }
    mesh.unload();
    log::info!("quit");
}