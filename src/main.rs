#![allow(dead_code)]
// #![allow(unused_imports)]
extern crate termion;

use std::fs::read_dir;
use std::io::{stdin, stdout, Write};
use std::path::PathBuf;
use termion::event::{Event, Key};
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use termion::screen::{AlternateScreen, IntoAlternateScreen};
use termion::{color, style};

// TODO: create my own FileListItem struct
// eg: FileListItem { name: str, type: File | Dir | SymLink, Meta: { size: str } }
// then use that as the item type for list, like items: Vec<FileListItem>
struct FileList<'a> {
    items: Vec<&'a str>,
    c_idx: u16,
}

#[derive(Debug)]
struct PathBufList<'a> {
    items: &'a Vec<PathBuf>,
    c_idx: u16,
}

impl<'a> PathBufList<'a> {
    fn next(&mut self) {
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

impl<'a> FileList<'a> {
    fn next(&mut self) {
        if self.c_idx == self.items.len() as u16 {
            self.c_idx = 1
        } else {
            self.c_idx = self.c_idx + 1
        }
    }

    fn prev(&mut self) {
        if self.c_idx == 1 {
            self.c_idx = self.items.len() as u16
        } else {
            self.c_idx = self.c_idx - 1
        }
    }
}

fn main() {
    let mut screen = stdout().into_alternate_screen().unwrap();
    let _stdout = stdout().into_raw_mode();
    write!(screen, "{}", termion::clear::All).unwrap();
    let dir_list = read_dir(".").unwrap();
    let file_list: Vec<PathBuf> = dir_list.into_iter().map(|i| i.unwrap().path()).collect();

    let mut file_list_st = PathBufList {
        items: &file_list,
        c_idx: 1,
    };

    render(&mut screen, &file_list_st);
    screen.flush().unwrap();
    let stdin = stdin();
    for c in stdin.events() {
        let evt = c.unwrap();
        match evt {
            Event::Key(Key::Char('q')) => break,
            Event::Key(Key::Char('j')) => file_list_st.next(),
            Event::Key(Key::Char('k')) => file_list_st.prev(),
            _ => {}
        }
        render(&mut screen, &file_list_st);
        screen.flush().unwrap();
    }
    // stdout.expect("should flush screen").flush().unwrap();
}

fn render<W: Write>(screen: &mut AlternateScreen<W>, file_list: &PathBufList) {
    let mut idx = 1;
    // let file_icon = "";
    // let file_icon = "";
    for item in file_list.items.clone() {
        if file_list.c_idx == idx {
            write!(screen, "{} ", termion::cursor::Goto(1, idx)).unwrap();
            set_style_alt(screen);
            write!(screen, "{:?}", item).unwrap();
            set_style_main(screen);
        } else {
            write!(screen, "{} {:?}", termion::cursor::Goto(1, idx), item).unwrap();
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
    write!(screen, "{}", style::Underline).unwrap();
}
