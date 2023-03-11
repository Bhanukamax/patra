#![allow(dead_code)]
// #![allow(unused_imports)]
extern crate termion;

use std::io::{stdin, stdout, Write};
use termion::event::{Event, Key};
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use termion::screen::{AlternateScreen, IntoAlternateScreen};
use termion::{color, style};

struct FileList<'a> {
    items: Vec<&'a str>,
    c_idx: u16,
}

impl<'a> FileList<'a> {
    fn next(&mut self) {
        // self.c_idx = (self.c_idx + 1) % self.items.len() as u16
        if self.c_idx == self.items.len() as u16 {
            self.c_idx = 1;
        } else {
            self.c_idx = self.c_idx + 1
        }
    }

    fn prev(&mut self) {
        if self.c_idx == 1 {
            self.c_idx = self.items.len() as u16;
        } else {
            self.c_idx = self.c_idx - 1;
        }
    }
}

fn main() {
    let mut screen = stdout().into_alternate_screen().unwrap();
    let mut stdout = stdout().into_raw_mode().unwrap();

    let items = vec![
        "word1", "word2", "word3", "word4", "word5", "word6", "word7",
    ];

    let mut file_list = FileList {
        items: items.clone(),
        c_idx: 1,
    };

    print!("{}", termion::clear::All);
    write!(screen, "{}", termion::cursor::Goto(1, 1)).unwrap();

    render(&mut screen, &file_list);
    stdout.flush().unwrap();

    let stdin = stdin();

    for c in stdin.events() {
        let evt = c.unwrap();

        match evt {
            Event::Key(Key::Char('q')) => break,
            Event::Key(Key::Char('j')) => file_list.next(),
            Event::Key(Key::Char('k')) => file_list.prev(),
            _ => {}
        }
        render(&mut screen, &file_list);
        screen.flush().unwrap();
        stdout.flush().unwrap();
    }
    write!(screen, "{}", termion::clear::All).unwrap();
}

fn render<W: Write>(screen: &mut AlternateScreen<W>, file_list: &FileList) {
    let mut idx = 1;
    for item in file_list.items.clone() {
        if file_list.c_idx == idx {
            write!(screen, "{} ", termion::cursor::Goto(1, idx)).unwrap();
            set_style_alt(screen);
            write!(screen, "{}", item).unwrap();
            set_style_main(screen);
        } else {
            write!(screen, "{} {}", termion::cursor::Goto(1, idx), item).unwrap();
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
