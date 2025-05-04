use crate::platform::game_window::GameWindow;
use crate::render::graphics_state::GraphicsState;
use glfw::{Action, Key};

pub async fn run() {
    let mut game_window = GameWindow::new("Rusty Knight");
    let mut graphics_state = GraphicsState::new(&mut game_window.window).await;

    graphics_state.window.set_framebuffer_size_polling(true);
    graphics_state.window.set_key_polling(true);
    graphics_state.window.set_mouse_button_polling(true);
    graphics_state.window.set_pos_polling(true);

    while !graphics_state.window.should_close() {
        game_window.glfw.poll_events();

        for (_, event) in glfw::flush_messages(&game_window.events) {
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
