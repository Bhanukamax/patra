// #![allow(dead_code)]
// #![allow(unused_imports)]
extern crate termion;

use std::io::{stdin, stdout, Write};
use termion::event::{Event, Key};
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use termion::screen::IntoAlternateScreen;
mod patra;
mod render;
use patra::*;
use render::*;

fn main() {
    let mut screen = stdout().into_alternate_screen().unwrap();
    let _stdout = stdout().into_raw_mode();
    write!(screen, "{}", termion::clear::All).unwrap();
    write!(screen, "{} ", termion::cursor::Hide).unwrap();

    let mut file_list_st = FileList {
        path: String::from(std::env::current_dir().unwrap().to_str().unwrap()),
        items: None,
        c_idx: 1,
    };
    file_list_st
        .list_dir()
        .expect("Something went wrong, check if you have permission to read the directory");

    if let Some(file_list_items) = &file_list_st.items {
        render_app(&mut screen, &file_list_items.as_ref(), file_list_st.c_idx);
    } else {
        println!("No listing! Press q to quit");
    }

    screen.flush().unwrap();
    let stdin = stdin();
    for c in stdin.events() {
        let evt = c.unwrap();
        if let Event::Key(Key::Char(key)) = evt {
            match &key {
                'q' => {
                    write!(screen, "{} ", termion::cursor::Show).unwrap();
                    break;
                }
                'j' => file_list_st.next(),
                'k' => file_list_st.prev(),
                '-' | 'h' => {
                    file_list_st.up_dir();
                    render_path(&mut screen, &file_list_st);
                }
                '\n' | 'l' => {
                    file_list_st.enter(&mut screen).unwrap_or_default();
                    render_path(&mut screen, &file_list_st);
                }
                _ => {}
            }
        }

        if let Some(file_list_items) = &file_list_st.items {
            render_app(&mut screen, &file_list_items.as_ref(), file_list_st.c_idx);
        } else {
            println!("No listing! Press q to quit");
        }

        screen.flush().unwrap();
    }
}
