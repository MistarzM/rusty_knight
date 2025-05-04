use crate::constants::{gameplay, graphics};
use crate::platform::game_window::GameWindow;

pub struct App;

impl App {
    pub async fn run() {
        let mut game_window = GameWindow::new(
            "Rusty Knight",
            graphics::MAP_WDITH_PIXELS,
            graphics::MAP_HEIGHT_PIXELS,
        );

        if gameplay::USE_MOUSE {
            game_window.window_mut().set_all_polling(true);
        } else {
            game_window.window_mut().set_key_polling(true);
        }

        game_window.run_game_loop().await;
    }
}
