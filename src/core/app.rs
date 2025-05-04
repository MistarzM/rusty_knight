use crate::constants::graphics;
use crate::platform::game_window::GameWindow;

use glfw::{Action, Context, Key};

pub struct App;

impl App {
    pub fn run() {
        let GameWindow {
            mut glfw,
            mut window,
            events,
        } = GameWindow::new(
            "Rusty Knight",
            graphics::MAP_WDITH_PIXELS,
            graphics::MAP_HEIGHT_PIXELS,
        );

        while !window.should_close() {
            glfw.poll_events();

            for (_, event) in glfw::flush_messages(&events) {
                match event {
                    glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
                        window.set_should_close(true);
                    }
                    e => println!("Action: {e:?}"),
                }
            }

            window.swap_buffers();
        }
    }
}
