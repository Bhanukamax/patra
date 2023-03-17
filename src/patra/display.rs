use super::app::{PatraFileItemType, PatraFileListItem, PatraFileState};
use std::io::Write;
use termion::{self, color, screen::AlternateScreen, style};

pub fn render<W: Write>(
    screen: &mut AlternateScreen<W>,
    state: &PatraFileState,
) -> Result<(), std::io::Error> {
    render_path(screen, state)?;
    render_app(screen, &state.list.clone().unwrap(), state.c_idx)?;
    Ok(())
}

pub fn render_app<W: Write>(
    screen: &mut AlternateScreen<W>,
    file_list: &Vec<PatraFileListItem>,
    c_idx: u16,
) -> Result<(), std::io::Error> {
    file_list.iter().enumerate().for_each(|(idx, item)| {
        render_item(screen, &item, idx as u16 + 1, c_idx == idx as u16 + 1).unwrap()
    });
    Ok(())
}

pub fn render_path<W: Write>(
    screen: &mut AlternateScreen<W>,
    file_list: &PatraFileState,
) -> Result<(), std::io::Error> {
    set_style_path(screen);
    write!(screen, "{}", termion::clear::All)?;
    move_cursor_cursor(screen, 10, 1);
    write!(screen, "{} ", "                   ")?;
    move_cursor_cursor(screen, 10, 1);
    write!(screen, "{} ", &file_list.path)?;
    Ok(())
}

pub fn render_item<W: Write>(
    screen: &mut AlternateScreen<W>,
    item: &PatraFileListItem,
    idx: u16,
    selected: bool,
) -> Result<(), std::io::Error> {
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
        write!(screen, "{}", style::Bold)?;
        move_cursor_cursor(screen, 1, idx + 2);
        write!(screen, "{} ", icon)?;
        write!(screen, "{}", style::Underline)?;
        write!(screen, "{}{}", item.name, suffix)?;
        write!(screen, "{}", style::NoBold)?;
        write!(screen, "{}", style::NoUnderline)?;
    } else {
        move_cursor_cursor(screen, 1, idx + 2);
        write!(screen, "{}", style::Bold)?;
        write!(screen, "{}", icon)?;
        write!(screen, "{}", style::NoBold)?;
        write!(screen, " {}{}", item.name.to_string(), suffix)?;
    }
    Ok(())
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
    write!(screen, "{}", termion::cursor::Goto(x, y)).unwrap();
}
