use rusty_knight::core::app::App;

fn main() {
    pollster::block_on(App::run());
}
