use crate::app::App;

pub mod app;
pub mod terminal;
pub mod ui;

fn main() {
    let mut app = App::default();
    if let Err(e) = app.run() {
        println!("Something went wrong {}", e)
    }
}
