use std::io::Write;

use crate::patra::*;
use termion::screen::AlternateScreen;
use termion::{color, style};

pub fn render_app<W: Write>(screen: &mut AlternateScreen<W>, file_list: &Vec<FileItem>, c_idx: u16) {
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
        write!(screen, "{}{} ", termion::cursor::Goto(1, idx + 2), icon).unwrap();
        set_style_alt(screen);
        write!(screen, "{}{}", item.name, suffix).unwrap();
        set_style_main(screen);
    } else {
        write!(
            screen,
            "{}{} {}{}",
            termion::cursor::Goto(1, idx + 2),
            icon,
            item.name.to_string(),
            suffix
        )
        .unwrap();
    }
}

pub fn set_style_main<W: Write>(screen: &mut AlternateScreen<W>) {
    // write!(screen, "{}", color::Fg(color::White)).unwrap();
    // write!(screen, "{}", color::Bg(color::Black)).unwrap();
    write!(screen, "{}", style::NoUnderline).unwrap();
}

pub fn set_style_dir<W: Write>(screen: &mut AlternateScreen<W>) {
    write!(screen, "{}", color::Fg(color::Blue)).unwrap();
    write!(screen, "{}", color::Bg(color::Black)).unwrap();
    write!(screen, "{}", style::NoUnderline).unwrap();
}

pub fn set_style_path<W: Write>(screen: &mut AlternateScreen<W>) {
    write!(screen, "{}", color::Fg(color::Yellow)).unwrap();
    write!(screen, "{}", color::Bg(color::Black)).unwrap();
    write!(screen, "{}", style::NoUnderline).unwrap();
}

pub fn set_style_file<W: Write>(screen: &mut AlternateScreen<W>) {
    write!(screen, "{}", color::Fg(color::White)).unwrap();
    write!(screen, "{}", color::Bg(color::Black)).unwrap();
    write!(screen, "{}", style::NoUnderline).unwrap();
}

pub fn set_style_alt<W: Write>(screen: &mut AlternateScreen<W>) {
    write!(screen, "{}", style::Underline).unwrap();
}
