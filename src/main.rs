use crate::app::App;

pub mod app;
pub mod terminal;
pub mod ui;

fn main() {
    let mut app = App::default();
    app.run()
}
