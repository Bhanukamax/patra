// #![allow(dead_code)]
// #![allow(unused_imports)]
extern crate termion;

use std::io::{self, stdin, Write};
use termion::event::{Event, Key};
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use termion::screen::IntoAlternateScreen;
mod patra;
use patra::app::PatraFileState;
use patra::display;
use patra::logger;
use tui::{backend::TermionBackend, Terminal};

fn main() {
    // let mut screen = stdout().into_alternate_screen().unwrap();
    logger::info("Starting app");
    if let Err(e) = run() {
        logger::error(&format!("Error: {}", e));
        // write!(screen, "{} ", termion::cursor::Show).unwrap()
    }
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let stdout = io::stdout().into_raw_mode()?;
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut screen = io::stdout().into_alternate_screen()?;
    // let _stdout = stdout().into_raw_mode();
    write!(screen, "{} ", termion::cursor::Hide)?;

    let mut file_list_st = PatraFileState::new(String::from(
        std::env::current_dir().unwrap().to_str().unwrap(),
    ));

    file_list_st.list_dir();

    display::render(&mut terminal, &file_list_st)?;

    screen.flush()?;
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
        if let Event::Key(Key::Ctrl(key)) = c? {
            match &key {
                // 'p' => display::scroll_up(&mut screen),
                // 'n' => display::scroll_down(&mut screen),
                _ => {}
            }
        }

        display::render(&mut terminal, &file_list_st)?;

        screen.flush().unwrap();
    }
    write!(screen, "{} ", termion::cursor::Show)?;
    Ok(())
}
