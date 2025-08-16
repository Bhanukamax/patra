use crate::app::{App, PatraFileItemType, PatraFileListItem, PatraFileState};
use crate::app::{CommandType, UiMode};
use std::io::{stdout, Write};
use termion::screen::IntoAlternateScreen;
use termion::{self, color, screen::AlternateScreen, style};

type Color = Box<dyn color::Color>;

pub enum ThemeColor {
    FileFg,
    FileBg,
    DirFg,
    _FileFocusFg,
    FileFocusBg,
    _DirSlash,
    CommandFg,
}

pub struct Theme {
    pub file_fg: Color,
    pub file_bg: Color,
    pub dir_fg: Color,
    pub file_focus_fg: Color,
    pub file_focus_bg: Color,
    pub dir_slash: Color,
    pub command_fg: Color,
}

fn hex_to_rgb(hex: &Option<String>) -> Option<(u8, u8, u8)> {
    if let Some(hex) = hex {
        let hex = hex.trim_start_matches('#');
        let r = u8::from_str_radix(&hex[0..2], 16);
        let g = u8::from_str_radix(&hex[2..4], 16);
        let b = u8::from_str_radix(&hex[4..6], 16);
        if let (Ok(r), Ok(g), Ok(b)) = (r, g, b) {
            return Some((r, g, b));
        }
    }
    None
}

pub fn color_from_string(value: &Option<String>) -> Option<Color> {
    if let Some((r, g, b)) = hex_to_rgb(value) {
        return Some(Box::new(color::Rgb(r, g, b)));
    }
    None
}

impl Theme {
    pub fn new(config_theme: &crate::config::Theme) -> Self {
        let value = 50;

        let focus_bg = color::Rgb(10, value, 100);
        let file_fg = color_from_string(&config_theme.file_fg).unwrap_or(Box::new(color::White));
        let dir_slash = color_from_string(&config_theme.dir_fg).unwrap_or(Box::new(color::Blue));
        let dir_fg = color_from_string(&config_theme.dir_fg).unwrap_or(Box::new(color::Blue));
        let file_bg = color_from_string(&config_theme.file_bg).unwrap_or(Box::new(color::Reset));
        let file_focus_fg =
            color_from_string(&config_theme.file_focus_fg).unwrap_or(Box::new(color::White));
        let file_focus_bg =
            color_from_string(&config_theme.file_focus_bg).unwrap_or(Box::new(focus_bg));
        let command_fg =
            color_from_string(&config_theme.command_fg).unwrap_or(Box::new(color::LightGreen));

        Self {
            file_fg,
            file_bg,
            dir_fg,
            file_focus_fg,
            file_focus_bg,
            dir_slash,
            command_fg,
        }
    }
}

#[derive(Default)]
pub struct Size {
    _w: u16,
    h: u16,
}

#[derive(Default)]
pub struct Position {
    x: u16,
    y: u16,
}

#[derive(Default)]
pub struct ListWidget {
    pub size: Size,
    pub screen_pos: Position,
    pub start_idx: u16,
}

pub struct Display {
    pub screen: AlternateScreen<std::io::Stdout>,
    pub list_widget: ListWidget,
    pub command_line: ListWidget,
    pub theme: Theme,
}

impl Display {
    pub fn new(config_theme: &crate::config::Theme) -> Self {
        let mut list_widget = ListWidget::default();
        let mut command_line = ListWidget::default();
        list_widget.size.h = 10_u16;
        if let Ok((_, rows)) = termion::terminal_size() {
            list_widget.size.h = rows - 1;
            command_line.screen_pos.y = rows;
            command_line.size.h = rows;
        }
        list_widget.screen_pos.y = 1_u16;
        list_widget.start_idx = 0;

        Self {
            theme: Theme::new(config_theme),
            screen: stdout().into_alternate_screen().unwrap(),
            list_widget,
            command_line,
        }
    }

    // Ensure terminal cleanup when Display is dropped
    pub fn cleanup(&mut self) -> Result<(), std::io::Error> {
        self.show_cursor()?;
        self.exit_alternate_screen()?;
        Ok(())
    }
    pub fn flush(&mut self) -> Result<(), std::io::Error> {
        self.screen.flush()
    }
    pub fn render(&mut self, app: &App) -> Result<(), std::io::Error> {
        let state = &app.state;
        let scroll_pos: u16 = state.c_idx.saturating_sub(self.list_widget.size.h);
        self.render_path(state)?;
        self.render_app(&state.list.clone(), state.c_idx, scroll_pos)?;
        match &app.ui_mode {
            UiMode::Command(CommandType::ConfirmDelete, Some(text)) => {
                self.render_cmd(text, &None).unwrap();
            }
            UiMode::Command(CommandType::CreateDir, Some(text))
            | UiMode::Command(CommandType::RenameNode, Some(text))
            | UiMode::Command(CommandType::CreateFile, Some(text)) => {
                self.render_cmd(text, &app.command_str)?;
            }
            _ => {
                self.hide_cursor()?;
            }
        }
        self.flush()?;
        Ok(())
    }
    pub fn draw_cursor(&mut self) {
        write!(self.screen, "{}", color::Fg(color::White)).unwrap();
        write!(self.screen, "█").unwrap();
    }

    pub fn hide_cursor(&mut self) -> Result<(), std::io::Error> {
        write!(self.screen, "{} ", termion::cursor::Hide)
    }
    pub fn show_cursor(&mut self) -> Result<(), std::io::Error> {
        write!(self.screen, "{} ", termion::cursor::Show)
    }

    pub fn exit_alternate_screen(&mut self) -> Result<(), std::io::Error> {
        // Exit alternate screen and restore main screen
        write!(self.screen, "{}", termion::screen::ToMainScreen)?;
        self.flush()
    }

    pub fn render_cmd(
        &mut self,
        prompt: &str,
        text: &Option<String>,
    ) -> Result<(), std::io::Error> {
        self.move_cursor_cursor(
            self.command_line.screen_pos.x,
            self.command_line.screen_pos.y,
        );
        self.set_style_command();
        write!(self.screen, "{}", termion::clear::CurrentLine)?;
        write!(self.screen, "{}", termion::clear::AfterCursor)?;
        write!(self.screen, "{}", prompt)?;
        self.set_style_file();
        if let Some(input) = text {
            write!(self.screen, "{}", input)?;
        }
        self.draw_cursor();
        Ok(())
    }

    pub fn render_app(
        &mut self,
        state_list: &[PatraFileListItem],
        c_idx: u16,
        scroll_pos: u16,
    ) -> Result<(), std::io::Error> {
        let filter_start: usize = scroll_pos.into();
        let filter_end: usize = self.list_widget.size.h as usize + scroll_pos as usize;
        let screen_start: u16 = self.list_widget.screen_pos.y;
        state_list
            .iter()
            .enumerate()
            .filter(|(idx, _)| idx >= &filter_start && idx < &filter_end)
            .for_each(|(idx, item)| {
                self.render_item(
                    item,
                    idx as u16 + screen_start - scroll_pos,
                    c_idx == idx as u16 + 1,
                )
                .unwrap()
            });
        Ok(())
    }

    pub fn render_path(&mut self, state: &PatraFileState) -> Result<(), std::io::Error> {
        self.set_style_path();
        write!(&mut self.screen, "{}", termion::clear::All)?;
        self.move_cursor_cursor(10, 1);
        write!(&mut self.screen, "{}", termion::clear::CurrentLine)?;
        write!(&mut self.screen, "[{}] {} ", &state.c_idx, &state.path)?;
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
            PatraFileItemType::File => {
                if let Some(ext) = std::path::Path::new(&item.name)
                    .extension()
                    .and_then(|ext| ext.to_str())
                {
                    match ext {
                        "rs" => ("\u{e7a8}", ""),
                        "ts" => ("\u{fbe4}", ""),
                        "scala" => ("\u{e68e}", ""),
                        "toml" | "json" | "py" => ("\u{eae9}", ""),
                        _ => ("", ""),
                    }
                } else {
                    ("", "")
                }
            }
            PatraFileItemType::Sym => ("", ""),
            PatraFileItemType::Unknown => ("⚠", ""),
        };

        if selected {
            self.set_style_focus();
        } else {
            self.set_style_unfocus();
        }
        self.move_cursor_cursor(1, idx + 2);
        write!(&mut self.screen, "{}", style::Bold)?;
        write!(&mut self.screen, "{}", icon)?;
        // write!(&mut self.screen, "{}", style::NoBold)?;
        write!(&mut self.screen, "{}", style::NoFaint)?;
        write!(&mut self.screen, " {}{}", item.name, suffix)?;
        self.set_style_unfocus();
        // write!(&mut self.screen, "{}", SteadyUnderline)?;

        Ok(())
    }

    pub fn set_style_dir(&mut self) {
        write!(&mut self.screen, "{}", style::NoUnderline).unwrap();
        self.set_color(ThemeColor::DirFg);
        self.set_color(ThemeColor::FileBg);
    }

    pub fn set_style_path(&mut self) {
        write!(&mut self.screen, "{}", color::Fg(color::Yellow)).unwrap();
    }

    pub fn set_style_command(&mut self) {
        write!(&mut self.screen, "{}", termion::style::Bold).unwrap();
        self.set_color(ThemeColor::CommandFg);
    }

    pub fn set_color(&mut self, color: ThemeColor) {
        let (fg_color, bg_color) = match color {
            ThemeColor::FileFg => (Some(self.theme.file_fg.as_ref()), None),
            ThemeColor::DirFg => (Some(self.theme.dir_fg.as_ref()), None),
            ThemeColor::_FileFocusFg => (Some(self.theme.file_focus_fg.as_ref()), None),
            ThemeColor::_DirSlash => (Some(self.theme.dir_slash.as_ref()), None),
            ThemeColor::CommandFg => (Some(self.theme.command_fg.as_ref()), None),
            ThemeColor::FileBg => (None, Some(self.theme.file_bg.as_ref())),
            ThemeColor::FileFocusBg => (None, Some(self.theme.file_focus_bg.as_ref())),
        };

        if let Some(fg) = fg_color {
            write!(&mut self.screen, "{}", color::Fg(fg)).unwrap();
        }
        if let Some(bg) = bg_color {
            write!(&mut self.screen, "{}", color::Bg(bg)).unwrap();
        }
    }

    pub fn set_style_file(&mut self) {
        self.set_color(ThemeColor::FileFg);
        self.set_color(ThemeColor::FileBg);
    }

    pub fn set_style_unfocus(&mut self) {
        self.set_color(ThemeColor::FileBg);
    }
    pub fn set_style_focus(&mut self) {
        self.set_color(ThemeColor::FileFocusBg);
    }

    pub fn move_cursor_cursor(&mut self, x: u16, y: u16) {
        write!(self.screen, "{}", termion::cursor::Goto(x, y)).unwrap();
    }
}

impl Drop for Display {
    fn drop(&mut self) {
        // Ensure terminal cleanup when Display is dropped
        let _ = self.cleanup();
    }
}
