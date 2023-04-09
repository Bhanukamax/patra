use crate::app::{PatraFileItemType, PatraFileListItem, PatraFileState};
use std::io::{stdout, Write};
use termion::screen::IntoAlternateScreen;
use termion::{self, color, screen::AlternateScreen, style};

pub struct Display {
    pub screen: AlternateScreen<std::io::Stdout>,
}

impl Display {
    pub fn new() -> Self {
        Self {
            screen: stdout().into_alternate_screen().unwrap(),
        }
    }
    pub fn flush(&mut self) -> Result<(), std::io::Error> {
        self.screen.flush()
    }
    pub fn render(&mut self, state: &PatraFileState) -> Result<(), std::io::Error> {
        self.render_path(state)?;
        self.render_app(&state.list.clone(), state.c_idx)?;
        Ok(())
    }
    pub fn hide_cursor(&mut self) -> Result<(), std::io::Error> {
        write!(self.screen, "{} ", termion::cursor::Hide)
    }
    pub fn show_cursor(&mut self) -> Result<(), std::io::Error> {
        write!(self.screen, "{} ", termion::cursor::Show)
    }

    pub fn render_app(
        &mut self,
        file_list: &[PatraFileListItem],
        c_idx: u16,
    ) -> Result<(), std::io::Error> {
        file_list.iter().enumerate().for_each(|(idx, item)| {
            self.render_item(item, idx as u16 + 1, c_idx == idx as u16 + 1)
                .unwrap()
        });
        Ok(())
    }

    pub fn render_path(&mut self, file_list: &PatraFileState) -> Result<(), std::io::Error> {
        self.set_style_path();
        write!(&mut self.screen, "{}", termion::clear::All)?;
        self.move_cursor_cursor(10, 1);
        write!(&mut self.screen, "                   ")?;
        self.move_cursor_cursor(10, 1);
        write!(&mut self.screen, "{} ", &file_list.path)?;
        Ok(())
    }

    pub fn render_item(
        &mut self,
        item: &PatraFileListItem,
        idx: u16,
        selected: bool,
    ) -> Result<(), std::io::Error> {
        self.set_style_file();
        let (icon, suffix) = match item.file_type {
            PatraFileItemType::Dir => {
                self.set_style_dir();
                ("", "/")
            }
            PatraFileItemType::File => ("", ""),
            PatraFileItemType::Sym => ("", ""),
            PatraFileItemType::Unknown => ("⚠", ""),
        };

        if selected {
            write!(&mut self.screen, "{}", style::Bold)?;
            self.move_cursor_cursor(1, idx + 2);
            write!(&mut self.screen, "{} ", icon)?;
            write!(&mut self.screen, "{}", style::Underline)?;
            write!(&mut self.screen, "{}{}", item.name, suffix)?;
            write!(&mut self.screen, "{}", style::NoBold)?;
            write!(&mut self.screen, "{}", style::NoUnderline)?;
        } else {
            self.move_cursor_cursor(1, idx + 2);
            write!(&mut self.screen, "{}", style::Bold)?;
            write!(&mut self.screen, "{}", icon)?;
            write!(&mut self.screen, "{}", style::NoBold)?;
            write!(&mut self.screen, " {}{}", item.name, suffix)?;
        }
        Ok(())
    }

    pub fn set_style_dir(&mut self) {
        write!(&mut self.screen, "{}", color::Fg(color::Blue)).unwrap();
        write!(&mut self.screen, "{}", color::Bg(color::Black)).unwrap();
    }

    pub fn set_style_path(&mut self) {
        write!(&mut self.screen, "{}", color::Fg(color::Yellow)).unwrap();
        write!(&mut self.screen, "{}", color::Bg(color::Black)).unwrap();
    }

    pub fn set_style_file(&mut self) {
        write!(&mut self.screen, "{}", color::Fg(color::White)).unwrap();
        write!(&mut self.screen, "{}", color::Bg(color::Black)).unwrap();
    }

    pub fn move_cursor_cursor(&mut self, x: u16, y: u16) {
        write!(&mut self.screen, "{}", termion::cursor::Goto(x, y)).unwrap();
    }
}
