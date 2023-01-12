use glam::Vec3;

use crate::camera::SimpleCamera;
use sdl2::video::Window;

pub struct Renderer {
    pub window: Window,
    pub main_camera: SimpleCamera,
}

impl Renderer {
    pub fn new(window: Window, main_camera: SimpleCamera) -> Self {
        Renderer {
            window,
            main_camera,
        }
    }

    pub fn render(&self, render_function: &dyn Fn() -> ()) {
        unsafe {
            let (window_width, window_height) = self.window.size();
            gl::Viewport(0, 0, window_width as i32, window_height as i32);
        }

        render_function();

        self.window.gl_swap_window();

        unsafe {
            gl::ClearColor(0.1, 0.1, 0.1, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }
    }
}
