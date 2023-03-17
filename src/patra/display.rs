use super::app::{PatraFileItem, PatraFileItemType, PatraFileList};
use std::io::Write;
use termion::{self, color, screen::AlternateScreen, style};

pub fn render_app<W: Write>(
    screen: &mut AlternateScreen<W>,
    file_list: &Vec<PatraFileItem>,
    c_idx: u16,
) {
    file_list.iter().enumerate().for_each(|(idx, item)| {
        render_item(screen, &item, idx as u16 + 1, c_idx == idx as u16 + 1);
    });
}

pub fn render_item<W: Write>(
    screen: &mut AlternateScreen<W>,
    item: &PatraFileItem,
    idx: u16,
    selected: bool,
) {
    set_style_file(screen);
    let (icon, suffix) = match item.file_type {
        PatraFileItemType::Dir => {
            set_style_dir(screen);
            ("", "/")
        }
        PatraFileItemType::File => ("", ""),
        PatraFileItemType::Sym => ("", ""),
        PatraFileItemType::Unknown => ("⚠", ""),
    };

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
        write!(screen, "{}", icon).unwrap();
        write!(screen, "{}", style::NoBold).unwrap();
        write!(screen, " {}{}", item.name.to_string(), suffix).unwrap();
    }
}

pub fn render_path<W: Write>(screen: &mut AlternateScreen<W>, file_list: &PatraFileList) {
    set_style_path(screen);
    write!(screen, "{}", termion::clear::All).unwrap();
    move_cursor_cursor(screen, 10, 1);
    write!(screen, "{} ", "                   ").unwrap();
    move_cursor_cursor(screen, 10, 1);
    write!(screen, "{} ", &file_list.path).unwrap();
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
