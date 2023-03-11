#![allow(dead_code)]
// #![allow(unused_imports)]
extern crate termion;

use std::io::{stdin, stdout, Write};
use termion::event::{Event, Key};
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use termion::screen::{AlternateScreen, IntoAlternateScreen};
use termion::{color, style};

fn main() {
    let mut screen = stdout().into_alternate_screen().unwrap();
    let mut stdout = stdout().into_raw_mode().unwrap();

    print!("{}", termion::clear::All);
    write!(screen, "{}", termion::cursor::Goto(1, 1)).unwrap();

    let mut c_idx : u16= 3;
    render(&mut screen, c_idx);
    stdout.flush().unwrap();

    let stdin = stdin();

    for c in stdin.events() {
        let evt = c.unwrap();

        match evt {
            Event::Key(Key::Char('q')) => break,
            Event::Key(Key::Char('j')) => c_idx += 1,
            Event::Key(Key::Char('k')) => c_idx -= 1,
            _ => {}
        }
        render(&mut screen, c_idx);
        screen.flush().unwrap();
        stdout.flush().unwrap();
    }
    write!(screen, "{}", termion::clear::All).unwrap();
}

fn render<W: Write>(screen: &mut AlternateScreen<W>, c_idx: u16) {
    let items = [
        "word1", "word2", "word3", "word4", "word5", "word6", "word7",
    ];
    let mut idx = 1;
    for item in items {
        if c_idx == idx {
            set_style_alt(screen);
            write!(screen, "{}{}", termion::cursor::Goto(1, idx), item).unwrap();
            set_style_main(screen);
        } else {
            write!(screen, "{}{}", termion::cursor::Goto(1, idx), item).unwrap();
        }
        idx += 1;
    }
}

fn set_style_main<W: Write>(screen: &mut AlternateScreen<W>) {
    write!(screen, "{}", color::Fg(color::White)).unwrap();
    write!(screen, "{}", color::Bg(color::Black)).unwrap();
    write!(screen, "{}", style::NoUnderline).unwrap();
}

fn set_style_alt<W: Write>(screen: &mut AlternateScreen<W>) {
    // write!(screen, "{}", color::Bg(color::White)).unwrap();
    // write!(screen, "{}", color::Fg(color::Black)).unwrap();
    write!(screen, "{}", style::Underline).unwrap();
}
