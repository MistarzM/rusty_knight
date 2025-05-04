use glfw::{fail_on_errors, Context, Glfw, GlfwReceiver, PWindow, WindowEvent};

pub struct GameWindow {
    pub glfw: Glfw,
    pub window: PWindow,
    pub events: GlfwReceiver<(f64, WindowEvent)>,
}

impl GameWindow {
    pub fn new(title: &str, width: u32, height: u32) -> Self {
        let mut glfw = glfw::init(fail_on_errors!()).unwrap();

        let (mut window, events) = glfw
            .create_window(width, height, title, glfw::WindowMode::Windowed)
            .unwrap();

        window.make_current();
        window.set_all_polling(true);

        Self {
            glfw,
            window,
            events,
        }
    }
}
