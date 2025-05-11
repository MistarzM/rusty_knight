use crate::constants::graphics;
use glfw::{fail_on_errors, Glfw, GlfwReceiver, PWindow, WindowEvent, WindowHint};

pub struct GameWindow {
    pub glfw: Glfw,
    pub window: PWindow,
    pub events: GlfwReceiver<(f64, WindowEvent)>,
}

impl GameWindow {
    pub fn new(title: &str) -> Self {
        let mut glfw = glfw::init(fail_on_errors!()).unwrap();
        glfw.window_hint(WindowHint::ClientApi(glfw::ClientApiHint::NoApi));

        #[allow(unused_mut)]
        let (mut window, events) = glfw
            .create_window(
                graphics::MAP_WDITH_PIXELS,
                graphics::MAP_HEIGHT_PIXELS,
                title,
                glfw::WindowMode::Windowed,
            )
            .unwrap();

        //window.make_current();

        Self {
            glfw,
            window,
            events,
        }
    }
}
