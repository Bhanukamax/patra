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

use app::PatraFileState;

fn main() {
    logger::info("Starting app");
    if let Err(e) = run() {
        logger::error(&format!("Error: {}", e));
    }
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    // let mut screen = stdout().into_alternate_screen()?;
    let mut display = Display::new();
    let _stdout = stdout().into_raw_mode();
    display.hide_cursor()?;

    let mut file_list_st = PatraFileState::new(String::from(
        std::env::current_dir().unwrap().to_str().unwrap(),
    ));

    file_list_st
        .list_dir()
        .expect("Something went wrong, check if you have permission to read the directory");

    display.render(&file_list_st)?;
    display.flush()?;
    let stdin = stdin();
    for c in stdin.events() {
        if let Event::Key(Key::Char(key)) = c.as_ref().unwrap() {
            match &key {
                'q' => break,
                'j' => file_list_st.next(),
                'k' => file_list_st.prev(),
                '-' | 'h' => file_list_st.up_dir()?,
                '\n' | 'l' => file_list_st.enter()?,
                _ => {}
            }
        }

        display.render(&file_list_st)?;

        display.flush()?;
    }
    display.show_cursor()?;
    Ok(())
}
