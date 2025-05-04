use crate::constants::graphics;
use glfw::{fail_on_errors, Context, Glfw, GlfwReceiver, PWindow, WindowEvent};

pub struct GameWindow {
    pub glfw: Glfw,
    pub window: PWindow,
    pub events: GlfwReceiver<(f64, WindowEvent)>,
}

impl GameWindow {
    pub fn new(title: &str) -> Self {
        let mut glfw = glfw::init(fail_on_errors!()).unwrap();

        let (mut window, events) = glfw
            .create_window(
                graphics::MAP_WDITH_PIXELS,
                graphics::MAP_HEIGHT_PIXELS,
                title,
                glfw::WindowMode::Windowed,
            )
            .unwrap();

        window.make_current();

        Self {
            glfw,
            window,
            events,
        }
    }
}
