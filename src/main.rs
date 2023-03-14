// #![allow(dead_code)]
// #![allow(unused_imports)]
extern crate termion;

use std::fs::read_dir;
use std::io::{stdin, stdout, Stdout, Write};
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

// TODO: make the path vector to easily go back and forth
#[derive(std::clone::Clone)]
struct FileList {
    items: Option<Vec<FileItem>>,
    path: String,
    c_idx: u16,
}

impl FileList {
    fn list_dir(&mut self) -> std::io::Result<()> {
        // let dir_list = read_dir(&self.path).unwrap();
        let dir_list = match read_dir(&self.path) {
            Ok(dir) => dir,
            Err(e) => return Err(e),
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
        Ok(())
    }
    fn enter(&mut self, screen: &mut AlternateScreen<Stdout>) -> Result<(), std::io::Error> {
        let idx: usize = self.c_idx as usize - 1;
        match &self.items {
            Some(x) => match x[idx].file_type {
                FileItemType::Dir => {
                    let original_path = String::from(&self.path);
                    self.path = if &self.path == "/" {
                        String::from(&self.path) + &x[idx].name
                    } else {
                        String::from(&self.path) + "/" + &x[idx].name
                    };
                    match self.list_dir() {
                        Err(e) => {
                            write!(screen, "{}{} ", termion::cursor::Goto(10, 2), e).unwrap();
                            self.path = original_path;
                            return Err(e);
                        }
                        Ok(res) => return Ok(res),
                    }
                }
                _ => Ok(()),
            },
            None => Ok(()),
        }
    }
    fn up_dir(&mut self) {
        if &self.path != "/" {
            self.path = std::path::Path::new(&self.path)
                .parent()
                .unwrap()
                .to_str()
                .unwrap()
                .to_string();
            self.list_dir().unwrap();
        }
        let out = std::io::stderr();
        writeln!(&out, "updated dir {:?}", self.path).unwrap();
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
    let sym_icon = "";
    let unknown_icon = "";

    for item in file_list.clone() {
        let icon = match item.file_type {
            FileItemType::Dir => folder_icon,
            FileItemType::File => file_icon,
            FileItemType::Sym => sym_icon,
            FileItemType::Unknown => unknown_icon,
        };

        match item.file_type {
            FileItemType::Dir => set_style_dir(screen),
            _ => set_style_file(screen),
        }

        if c_idx == idx {
            write!(screen, "{}{} ", termion::cursor::Goto(1, idx + 2), icon).unwrap();
            set_style_alt(screen);
            write!(screen, "{}", item.name).unwrap();
            set_style_main(screen);
        } else {
            write!(
                screen,
                "{}{} {}",
                termion::cursor::Goto(1, idx + 2),
                icon,
                item.name.to_string()
            )
            .unwrap();
        }
        idx += 1;
    }
}

fn set_style_main<W: Write>(screen: &mut AlternateScreen<W>) {
    // write!(screen, "{}", color::Fg(color::White)).unwrap();
    // write!(screen, "{}", color::Bg(color::Black)).unwrap();
    write!(screen, "{}", style::NoUnderline).unwrap();
}

fn set_style_dir<W: Write>(screen: &mut AlternateScreen<W>) {
    write!(screen, "{}", color::Fg(color::Blue)).unwrap();
    write!(screen, "{}", color::Bg(color::Black)).unwrap();
    write!(screen, "{}", style::NoUnderline).unwrap();
}

fn set_style_path<W: Write>(screen: &mut AlternateScreen<W>) {
    write!(screen, "{}", color::Fg(color::Green)).unwrap();
    write!(screen, "{}", color::Bg(color::Black)).unwrap();
    write!(screen, "{}", style::NoUnderline).unwrap();
}

fn set_style_file<W: Write>(screen: &mut AlternateScreen<W>) {
    write!(screen, "{}", color::Fg(color::White)).unwrap();
    write!(screen, "{}", color::Bg(color::Black)).unwrap();
    write!(screen, "{}", style::NoUnderline).unwrap();
}

fn set_style_alt<W: Write>(screen: &mut AlternateScreen<W>) {
    write!(screen, "{}", style::Underline).unwrap();
}
