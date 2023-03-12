#![allow(dead_code)]
#![allow(unused_imports)]
extern crate termion;

use std::fs::read_dir;
use std::io::{stdin, stdout, Write};
use termion::event::{Event, Key};
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use termion::screen::{AlternateScreen, IntoAlternateScreen};
use termion::{color, style};

#[derive(Clone, Debug)]
enum FileItemType {
    File,
    Dir,
    Sym,
    Unknown,
}

#[derive(Clone, Debug)]
struct FileItem {
    name: String,
    file_type: FileItemType,
}

#[derive(std::clone::Clone)]
struct FileList {
    items: Option<Vec<FileItem>>,
    path: String,
    c_idx: u16,
}

impl FileList {
    fn list_dir(&mut self) {
        // let dir_list = read_dir(&self.path).unwrap();
        let dir_list = match read_dir(&self.path) {
            Ok(dir) => dir,
            Err(e) => {
                eprintln!("Field to open {}", e);
                return;
            }
        };

        self.items = Some(
            dir_list
                .into_iter()
                .map(|x| FileItem {
                    name: String::from(x.as_ref().unwrap().file_name().to_str().unwrap()),
                    file_type: if x.as_ref().unwrap().path().is_dir() {
                        FileItemType::Dir
                    } else if x.as_ref().unwrap().path().is_file() {
                        FileItemType::File
                    } else if x.unwrap().path().is_symlink() {
                        FileItemType::Sym
                    } else {
                        FileItemType::Unknown
                    },
                })
                .collect(),
        );
        self.c_idx = 1;
    }
    fn get_c_idx(&self) -> u16 {
        self.c_idx
    }
    fn enter(&mut self) {
        let idx: usize = self.c_idx as usize - 1;
        match &self.items {
            Some(x) => match x[idx].file_type {
                FileItemType::Dir => {
                    self.path = String::from(&self.path) + "/" + &x[idx].name;
                    self.list_dir();
                }
                _ => (),
            },
            None => (),
        }
    }
    fn next(&mut self) {
        match self.items {
            Some(_) => {
                if self.c_idx == self.items.as_ref().unwrap().len() as u16 {
                    self.c_idx = 1
                } else {
                    self.c_idx = self.c_idx + 1
                }
            }
            None => (),
        }
    }
    fn prev(&mut self) {
        match self.items {
            Some(_) => {
                if self.c_idx == 1 {
                    self.c_idx = self.items.as_ref().unwrap().len() as u16
                } else {
                    self.c_idx = self.c_idx - 1
                }
            }
            None => (),
        }
    }
}

fn _main() {
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
        path: ".".to_string(),
        items: None,
        c_idx: 1,
    };
    file_list_st.list_dir();

    // render(&mut screen, &file_list_st);
    match &file_list_st.items {
        Some(_) => render(
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
            Event::Key(Key::Char('\n')) => {
                write!(screen, "{}", termion::clear::All).unwrap();
                file_list_st.enter();
                // file_list_st.enter();
                write!(
                    screen,
                    "{}{} ",
                    termion::cursor::Goto(10, 20),
                    "                   "
                )
                .unwrap();
                write!(
                    screen,
                    "{}{} ",
                    termion::cursor::Goto(10, 20),
                    &file_list_st.path
                )
                .unwrap();
            }
            _ => {}
        }

        match &file_list_st.items {
            Some(_) => render(
                &mut screen,
                &file_list_st.items.as_ref().unwrap(),
                file_list_st.c_idx,
            ),
            None => (),
        }
        screen.flush().unwrap();
    }
}

fn render<W: Write>(screen: &mut AlternateScreen<W>, file_list: &Vec<FileItem>, c_idx: u16) {
    let mut idx = 1;

    let file_icon = "";
    let folder_icon = "";
    let sym_icon = "";
    let unknown_icon = "";

    for item in file_list.clone() {
        let icon = match item.file_type {
            FileItemType::File => file_icon,
            FileItemType::Dir => folder_icon,
            FileItemType::Sym => sym_icon,
            FileItemType::Unknown => unknown_icon,
        };

        if c_idx == idx {
            write!(screen, "{}{} ", termion::cursor::Goto(1, idx + 2), icon).unwrap();
            set_style_alt(screen);
            write!(screen, "{:?}", item.name).unwrap();
            set_style_main(screen);
        } else {
            write!(
                screen,
                "{}{} {:?}",
                termion::cursor::Goto(1, idx + 2),
                icon,
                item.name
            )
            .unwrap();
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
