#![allow(dead_code)]
#![allow(unused_imports)]
extern crate termion;

use std::io::{stdin, stdout, Write};
use termion::event::{Event, Key, MouseEvent};
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use termion::screen::IntoAlternateScreen;
use termion::{color, style};

fn main() {
    let mut screen = stdout().into_alternate_screen().unwrap();
    let mut stdout = stdout().into_raw_mode().unwrap();

    print!("{}", termion::clear::All);
    write!(screen, "{}", termion::cursor::Goto(1, 1)).unwrap();
    write!(screen, "Hey there.").unwrap();

    write!(screen, "{}One", termion::cursor::Goto(1,1)).unwrap();
    write!(screen, "{}Two", termion::cursor::Goto(1,2)).unwrap();
    write!(screen, "{}Three", termion::cursor::Goto(1,3)).unwrap();

    stdout.flush().unwrap();

    let stdin = stdin();

    for c in stdin.events() {
        let evt = c.unwrap();

        match evt {
            Event::Key(Key::Char('q')) => break,
            _ => {}
        }
        screen.flush().unwrap();
        stdout.flush().unwrap();
    }
    write!(screen, "{}", termion::clear::All).unwrap();
}
