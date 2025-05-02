use crate::platform::window::GameWindow;
use winit::{application::ApplicationHandler, event_loop::ActiveEventLoop};

#[derive(Default)]
pub struct App {
    window: Option<GameWindow>,
}

impl App {
    /// Creates a new application instance
    pub fn new() -> Self {
        Self::default()
    }

    /// Provides access to the window if it exist
    pub fn window(&self) -> Option<&GameWindow> {
        self.window.as_ref()
    }
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        self.window = Some(GameWindow::new(event_loop));
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _window_id: winit::window::WindowId,
        event: winit::event::WindowEvent,
    ) {
        if let Some(window) = &mut self.window {
            window.handle_event(event_loop, event);
        }
    }
}
