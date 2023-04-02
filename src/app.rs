use std::io::{self, stdout, Write};

// use crate::ui::Rect;
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use crate::screen;
use crate::patra::logger;

#[derive(Default)]
pub struct App {
    should_quite: bool,
}

impl App {
    pub fn run(&mut self) -> Result<(), std::io::Error> {
        println!("running the app");
        let _stdout = stdout().into_raw_mode().unwrap();
        screen::Screen::render()?;
        for key in io::stdin().keys() {
            // println!("{}", termion::cursor::Goto(1, 1));
            // println!("{}", termion::clear::CurrentLine);
            match key {
                Ok(_key) => self.handle_key(key.as_ref().unwrap()),
                Err(_e) => println!("error"),
            }
            if self.should_quite {
                println!("{}", termion::clear::All);
                io::stdout().flush()?;
                break;
            }
        }
        Ok(())
    }

    pub fn handle_key(&mut self, key: &Key) {
        match key {
            Key::Char('q') => self.should_quite = true,
            Key::Char(char) => {
                logger::debug(&(format!("{:?}", char)));
            },
            _ => {
                logger::debug(&(format!("{:?}", key)));
            }
        }
    }
}
