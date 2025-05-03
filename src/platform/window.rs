use winit::{
    event::WindowEvent,
    event_loop::ActiveEventLoop,
    window::{Window, WindowAttributes},
};

use crate::constants::graphics;

pub struct GameWindow {
    _inner: Window,
}

impl GameWindow {
    pub fn new(event_loop: &ActiveEventLoop) -> Self {
        let window = event_loop
            .create_window(
                WindowAttributes::default()
                    .with_title("Rusty Knight")
                    .with_inner_size(winit::dpi::LogicalSize::new(
                        graphics::MAP_WDITH_PIXELS,
                        graphics::MAP_HEIGHT_PIXELS,
                    )),
            )
            .unwrap();

        Self { _inner: window }
    }

    pub fn handle_event(&mut self, event_loop: &ActiveEventLoop, event: WindowEvent) {
        match event {
            WindowEvent::CloseRequested => event_loop.exit(),
            WindowEvent::RedrawRequested => self.render(),
            WindowEvent::Resized(size) => {
                println!("Widow resized to: {}x{}", size.width, size.height);
            }
            _ => (),
        }
    }

    fn render(&self) {
        println!("Rendering...");
    }
}
