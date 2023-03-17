// #![allow(dead_code)]
// #![allow(unused_imports)]
extern crate termion;

use std::io::{stdin, stdout, Write};
use termion::event::{Event, Key};
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use termion::screen::IntoAlternateScreen;
mod patra;
use patra::app::PatraFileState;
use patra::display;

fn main() {
    let mut screen = stdout().into_alternate_screen().unwrap();
    if let Err(e) = run() {
        eprintln!("Error: {}", e);
        write!(screen, "{} ", termion::cursor::Show).unwrap()
    }
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let mut screen = stdout().into_alternate_screen().unwrap();
    let _stdout = stdout().into_raw_mode();
    write!(screen, "{} ", termion::cursor::Hide).unwrap();

    let mut file_list_st = PatraFileState::new(String::from(
        std::env::current_dir().unwrap().to_str().unwrap(),
    ));

    file_list_st
        .list_dir()
        .expect("Something went wrong, check if you have permission to read the directory");

    display::render(&mut screen, &file_list_st);

    screen.flush().unwrap();
    let stdin = stdin();
    for c in stdin.events() {
        if let Event::Key(Key::Char(key)) = c.unwrap() {
            match &key {
                'q' => break,
                'j' => file_list_st.next(),
                'k' => file_list_st.prev(),
                '-' | 'h' => file_list_st.up_dir()?,
                '\n' | 'l' => file_list_st.enter(&mut screen).unwrap(),
                _ => {}
            }
        }

        display::render(&mut screen, &file_list_st);

        screen.flush().unwrap();
    }
    write!(screen, "{} ", termion::cursor::Show).unwrap();
    Ok(())
}
