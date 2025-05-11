use crate::model::world::World;
use crate::renderer::renderer::State;
use crate::{model::game_object::Object, platform::game_window::GameWindow};
use glfw::{Action, Key};

pub async fn run() {
    let mut game_window = GameWindow::new("Rusty Knight");
    let mut graphics_state = State::new(&mut game_window.window).await;

    graphics_state.window.set_framebuffer_size_polling(true);
    graphics_state.window.set_key_polling(true);
    graphics_state.window.set_mouse_button_polling(true);
    graphics_state.window.set_pos_polling(true);
    graphics_state
        .window
        .set_cursor_mode(glfw::CursorMode::Hidden);

    graphics_state.load_assets();

    let mut world = World::new();
    world.quads.push(Object {
        position: glm::Vec3::new(0.5, 0.0, -1.5),
        angle: 0.0,
    });
    world.tris.push(Object {
        position: glm::Vec3::new(0.0, 0.0, -1.0),
        angle: 0.0,
    });
    graphics_state.build_ubos_for_objects(2);
    world.keys.insert(glfw::Key::W, false);
    world.keys.insert(glfw::Key::A, false);
    world.keys.insert(glfw::Key::S, false);
    world.keys.insert(glfw::Key::D, false);

    while !graphics_state.window.should_close() {
        game_window.glfw.poll_events();
        world.update(16.67, graphics_state.window);

        for (_, event) in glfw::flush_messages(&game_window.events) {
            match event {
                glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
                    graphics_state.window.set_should_close(true);
                }

                // Movement
                glfw::WindowEvent::Key(key, _, Action::Press, _) => {
                    world.set_key(key, true);
                }
                glfw::WindowEvent::Key(key, _, Action::Release, _) => {
                    world.keys.insert(key, false);
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

        match graphics_state.render(&world.quads, &world.tris, &world.camera) {
            Ok(_) => {}
            Err(wgpu::SurfaceError::Lost | wgpu::SurfaceError::Outdated) => {
                graphics_state.update_surface();
                graphics_state.resize(graphics_state.size);
            }
            Err(e) => eprintln!("{e:?}"),
        }
    }
}
