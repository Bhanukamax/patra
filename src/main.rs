use crate::app::App;

pub mod app;
pub mod terminal;

fn main() {
    let mut app = App::default();
    app.run()
}
