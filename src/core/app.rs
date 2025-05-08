use crate::render::graphics_state::{GraphicsState, World};
use crate::{platform::game_window::GameWindow, render::model::game_object::Object};
use glfw::{Action, Key};

pub async fn run() {
    let mut game_window = GameWindow::new("Rusty Knight");
    let mut graphics_state = GraphicsState::new(&mut game_window.window).await;

    graphics_state.window.set_framebuffer_size_polling(true);
    graphics_state.window.set_key_polling(true);
    graphics_state.window.set_mouse_button_polling(true);
    graphics_state.window.set_pos_polling(true);

    let mut world = World::new();
    world.quads.push(Object {
        position: glm::Vec3::new(0.5, 0.0, 0.0),
        angle: 0.0,
    });
    world.tris.push(Object {
        position: glm::Vec3::new(0.0, 0.0, 0.0),
        angle: 0.0,
    });
    graphics_state.build_ubos_for_objects(2);

    while !graphics_state.window.should_close() {
        game_window.glfw.poll_events();
        world.update(16.67);

        for (_, event) in glfw::flush_messages(&game_window.events) {
            match event {
                glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
                    graphics_state.window.set_should_close(true);
                }
                // Window was moved
                glfw::WindowEvent::Pos(..) => {
                    graphics_state.update_surface();
                    graphics_state.resize(graphics_state.size);
                }
                // Window was resized
                glfw::WindowEvent::FramebufferSize(width, height) => {
                    graphics_state.update_surface();
                    graphics_state.resize((width, height));
                }
                _ => {} //e => println!("Action: {e:?}"),
            }
        }

        match graphics_state.render(&world.quads, &world.tris) {
            Ok(_) => {}
            Err(wgpu::SurfaceError::Lost | wgpu::SurfaceError::Outdated) => {
                graphics_state.update_surface();
                graphics_state.resize(graphics_state.size);
            }
            Err(e) => eprintln!("{e:?}"),
        }
    }
}
