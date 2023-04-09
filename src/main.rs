// #![allow(dead_code)]
// #![allow(unused_imports)]
extern crate termion;

mod app;
mod display;
mod logger;

use display::Display;
use std::io::{stdin, stdout};
use termion::event::{Event, Key};
use termion::input::TermRead;
use termion::raw::IntoRawMode;

use app::App;

fn main() {
    logger::info("Starting app");
    if let Err(e) = run() {
        logger::error(&format!("Error: {}", e));
    }
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let mut app = App::default();
    let mut display = Display::new();
    let _stdout = stdout().into_raw_mode();
    display.hide_cursor()?;

    app.state
        .list_dir()
        .expect("Something went wrong, check if you have permission to read the directory");

    display.render(&app.state)?;
    let stdin = stdin();
    for c in stdin.events() {
        if let Event::Key(Key::Char(key)) = c.as_ref().unwrap() {
            match &key {
                'q' => app.quit(),
                'j' => app.state.next(),
                'k' => app.state.prev(),
                '-' | 'h' => app.state.up_dir()?,
                '\n' | 'l' => app.state.enter()?,
                _ => {}
            }
        }

        if app.should_quit {
            break;
        }

        display.render(&app.state)?;
    }
    display.show_cursor()?;
    Ok(())
}
