use crate::render::graphics_state::GraphicsState;
use glfw::{fail_on_errors, Action, Context, Glfw, GlfwReceiver, Key, PWindow, WindowEvent};

pub struct GameWindow {
    glfw: Glfw,
    window: PWindow,
    events: GlfwReceiver<(f64, WindowEvent)>,
}

impl GameWindow {
    pub fn new(title: &str, width: u32, height: u32) -> Self {
        let mut glfw = glfw::init(fail_on_errors!()).unwrap();

        let (mut window, events) = glfw
            .create_window(width, height, title, glfw::WindowMode::Windowed)
            .unwrap();

        window.make_current();

        Self {
            glfw,
            window,
            events,
        }
    }

    pub fn window_mut(&mut self) -> &mut PWindow {
        &mut self.window
    }

    pub async fn run_game_loop(&mut self) {
        let mut graphics_state = GraphicsState::new(&mut self.window).await;

        while !graphics_state.window.should_close() {
            self.glfw.poll_events();

            for (_, event) in glfw::flush_messages(&self.events) {
                match event {
                    glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
                        graphics_state.window.set_should_close(true);
                    }
                    glfw::WindowEvent::Pos(..) => {
                        graphics_state.update_surface();
                        graphics_state.resize(graphics_state.size);
                    }
                    glfw::WindowEvent::FramebufferSize(width, height) => {
                        graphics_state.update_surface();
                        graphics_state.resize((width, height));
                    }
                    _ => {} //e => println!("Action: {e:?}"),
                }
            }

            match graphics_state.render() {
                Ok(_) => {}
                Err(wgpu::SurfaceError::Lost | wgpu::SurfaceError::Outdated) => {
                    graphics_state.update_surface();
                    graphics_state.resize(graphics_state.size);
                }
                Err(e) => eprintln!("{e:?}"),
            }
        }
    }
}
