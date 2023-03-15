// #![allow(dead_code)]
// #![allow(unused_imports)]
extern crate termion;

use std::fs::read_dir;
use std::io::{stdin, stdout, Write};
use termion::event::{Event, Key};
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use termion::screen::IntoAlternateScreen;
mod patra;
mod render;
use patra::*;
use render::*;

fn _main() {
    println!("{:?}", std::env::current_dir().unwrap().parent());
    let dir_list = read_dir(".").unwrap();

    for i in dir_list.into_iter() {
        println!("{}", i.unwrap().file_name().to_str().unwrap());
    }
}

fn main() {
    let mut screen = stdout().into_alternate_screen().unwrap();
    let _stdout = stdout().into_raw_mode();
    write!(screen, "{}", termion::clear::All).unwrap();

    let mut file_list_st = FileList {
        path: String::from(std::env::current_dir().unwrap().to_str().unwrap()),
        items: None,
        c_idx: 1,
    };
    file_list_st
        .list_dir()
        .expect("Something went wrong, check if you have permission to read the directory");

    // render(&mut screen, &file_list_st);
    match &file_list_st.items {
        Some(_) => render_app(
            &mut screen,
            &file_list_st.items.as_ref().unwrap(),
            file_list_st.c_idx,
        ),
        None => println!("No listing! Press q to quit"),
    }
    screen.flush().unwrap();
    let stdin = stdin();
    for c in stdin.events() {
        let evt = c.unwrap();
        match evt {
            Event::Key(Key::Char('q')) => break,
            Event::Key(Key::Char('j')) => file_list_st.next(),
            Event::Key(Key::Char('k')) => file_list_st.prev(),
            Event::Key(Key::Char('-')) => {
                set_style_path(&mut screen);
                write!(screen, "{}", termion::clear::All).unwrap();
                file_list_st.up_dir();
                write!(
                    screen,
                    "{}{} ",
                    termion::cursor::Goto(10, 1),
                    "                   "
                )
                .unwrap();
                write!(
                    screen,
                    "{}{} ",
                    termion::cursor::Goto(10, 1),
                    &file_list_st.path
                )
                .unwrap();
            }
            Event::Key(Key::Char('\n')) => {
                set_style_path(&mut screen);
                write!(screen, "{}", termion::clear::All).unwrap();
                file_list_st.enter(&mut screen).unwrap_or_default();
                write!(
                    screen,
                    "{}{} ",
                    termion::cursor::Goto(10, 1),
                    "                   "
                )
                .unwrap();
                write!(
                    screen,
                    "{}{} ",
                    termion::cursor::Goto(10, 1),
                    &file_list_st.path
                )
                .unwrap();
            }
            _ => {}
        }

        match &file_list_st.items {
            Some(_) => render_app(
                &mut screen,
                &file_list_st.items.as_ref().unwrap(),
                file_list_st.c_idx,
            ),
            None => (),
        }
        screen.flush().unwrap();
    }
}
