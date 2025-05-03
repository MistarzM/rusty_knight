use crate::platform::window::GameWindow;
use winit::{
    application::ApplicationHandler,
    event_loop::{ActiveEventLoop, EventLoop},
};

#[derive(Default)]
pub struct App {
    window: Option<GameWindow>,
}

impl App {
    /// Creates a new application instance
    fn new() -> Self {
        // default:
        // window: None
        Self::default()
    }

    /// Provides access to the window if it exist
    pub fn window(&self) -> Option<&GameWindow> {
        self.window.as_ref()
    }

    /// Start game loop
    pub fn run() {
        // EventLoop::new() :
        // - creates the event loop strucutre
        let event_loop = EventLoop::new().unwrap();
        let mut app = App::new();
        // run_app() :
        // - takes over program execution
        // - enters the OS event loop
        // - begins processing events and calling handlers
        // [ resumed() -> window_event() - continues - until - exit -> ... ]
        event_loop.run_app(&mut app).unwrap();
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
