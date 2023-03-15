use std::io::Write;

use crate::patra::*;
use termion::{self, color, screen::AlternateScreen, style};

pub fn render_app<W: Write>(
    screen: &mut AlternateScreen<W>,
    file_list: &Vec<FileItem>,
    c_idx: u16,
) {
    let mut idx = 1;
    for item in file_list.clone() {
        render_item(screen, &item, idx, c_idx == idx);
        idx += 1;
    }
}

pub fn render_item<W: Write>(
    screen: &mut AlternateScreen<W>,
    item: &FileItem,
    idx: u16,
    selected: bool,
) {
    let file_icon = "";
    let folder_icon = "";
    let sym_icon = "";
    let unknown_icon = "";
    let icon = match item.file_type {
        FileItemType::Dir => folder_icon,
        FileItemType::File => file_icon,
        FileItemType::Sym => sym_icon,
        FileItemType::Unknown => unknown_icon,
    };
    let mut suffix = "";
    match item.file_type {
        FileItemType::Dir => {
            suffix = "/";
            set_style_dir(screen)
        }
        _ => set_style_file(screen),
    }
    if selected {
        write!(screen, "{}", style::Bold).unwrap();
        move_cursor_cursor(screen, 1, idx + 2);
        write!(screen, "{} ", icon).unwrap();
        write!(screen, "{}", style::Underline).unwrap();
        write!(screen, "{}{}", item.name, suffix).unwrap();
        write!(screen, "{}", style::NoBold).unwrap();
        write!(screen, "{}", style::NoUnderline).unwrap();
    } else {
        move_cursor_cursor(screen, 1, idx + 2);
        write!(screen, "{}", style::Bold).unwrap();
        write!(screen, "{}", icon).unwrap_or_default();
        write!(screen, "{}", style::NoBold).unwrap();
        write!(screen, " {}{}", item.name.to_string(), suffix).unwrap();
    }
}


pub fn set_style_dir<W: Write>(screen: &mut AlternateScreen<W>) {
    write!(screen, "{}", color::Fg(color::Blue)).unwrap();
    write!(screen, "{}", color::Bg(color::Black)).unwrap();
}

pub fn set_style_path<W: Write>(screen: &mut AlternateScreen<W>) {
    write!(screen, "{}", color::Fg(color::Yellow)).unwrap();
    write!(screen, "{}", color::Bg(color::Black)).unwrap();
}

pub fn set_style_file<W: Write>(screen: &mut AlternateScreen<W>) {
    write!(screen, "{}", color::Fg(color::White)).unwrap();
    write!(screen, "{}", color::Bg(color::Black)).unwrap();
}

pub fn move_cursor_cursor<W: Write>(screen: &mut AlternateScreen<W>, x: u16, y: u16) {
    write!(screen, "{}", termion::cursor::Goto(x, y)).unwrap_or_default();
}
