use std::default;
use std::io::{self, stdin, stdout, Write};

use crate::terminal;
use crate::ui::Rect;
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

pub struct App {
    should_quite: bool,
}

impl Default for App {
    fn default() -> Self {
        Self {
            should_quite: false,
        }
    }
}

impl App {
    pub fn run(&mut self) {
        println!("running the app");
        let _stdout = stdout().into_raw_mode().unwrap();
        println!("{}", termion::clear::All);
        print!("{}", termion::cursor::Goto(1, 1));
        print!("{}", "bat");

        let rect = Rect::new(5 as u16, 5 as u16, 20 as u16, 10 as u16);
        rect.draw();

        print!("{}", termion::color::Fg(termion::color::Blue));
        let rect = Rect::new(2 as u16, 2 as u16, 30 as u16, 20 as u16);
        rect.draw();

        io::stdout().flush();

        for key in io::stdin().keys() {
            // println!("{}", termion::cursor::Goto(1, 1));
            // println!("{}", termion::clear::CurrentLine);
            match key {
                Ok(_key) => self.handle_key(key.as_ref().unwrap()),
                Err(_e) => println!("error"),
            }
            if self.should_quite {
                break;
            }
        }
    }

    pub fn handle_key(&mut self, key: &Key) {
        match key {
            Key::Char('q') => self.should_quite = true,
            _ => println!("{:?}\n", key),
        }
    }
}
