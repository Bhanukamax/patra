#![allow(dead_code)]
use termion::color;

type Color = Box<dyn color::Color>;

pub struct Theme {
    pub file_fg: Color,
    pub file_bg: Color,
    pub file_focus_fg: Color,
    pub file_focus_bg: Color,
}

pub struct Config {
    pub theme: Theme,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            theme: Theme::new(),
        }
    }
}

impl Theme {
    pub fn new() -> Self {
        let value = 50;
        // let focus_bg = color::Rgb(value, value, value);
        let focus_bg = color::Rgb(10, value, 100);
        Self {
            file_fg: Box::new(color::White),
            file_bg: Box::new(color::Reset),
            file_focus_fg: Box::new(color::White),
            file_focus_bg: Box::new(focus_bg),
        }
    }
}
