use std::io::{self, stdout, Write};

use crate::ui::Rect;
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

#[derive(Default)]
pub struct App {
    should_quite: bool,
}

impl App {
    pub fn run(&mut self) -> Result<(), std::io::Error> {
        println!("running the app");
        let _stdout = stdout().into_raw_mode().unwrap();
        println!("{}", termion::clear::All);
        print!("{}", termion::cursor::Goto(1, 1));

        let rect = Rect::new(5_u16, 5_u16, 20_u16, 10_u16);
        rect.draw();

        print!("{}", termion::color::Fg(termion::color::Blue));
        let rect = Rect::new(2_u16, 2_u16, 30_u16, 20_u16);
        rect.draw();

        io::stdout().flush()?;

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
        };
        Ok(())
    }

    pub fn handle_key(&mut self, key: &Key) {
        match key {
            Key::Char('q') => self.should_quite = true,
            _ => println!("{:?}\n", key),
        }
    }
}
