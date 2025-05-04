use rusty_knight::core::app;

fn main() {
    pollster::block_on(app::run());
}
