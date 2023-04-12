#![allow(dead_code)]
use serde::Deserialize;

#[derive(Deserialize, Debug, Default, Clone)]
pub struct Theme {
    pub file_fg: Option<String>,
    pub file_bg: Option<String>,
    pub file_focus_fg: Option<String>,
    pub file_focus_bg: Option<String>,
}

#[derive(Deserialize, Debug, Default, Clone)]
pub struct Config {
    pub theme: Theme,
}

impl Config {
    pub fn load() -> Result<Self, Box<dyn std::error::Error>> {
        let config: Config = toml::from_str(
            r#"
    [theme]
    file_fg = '#ffffff'
    file_focus_fg = '#fafafa'
    file_focus_bg = '#666666'
"#,
        )
        .unwrap_or(Config::default());
        Ok(config)
    }
    pub fn update_theme(&mut self, theme: Theme) {
        if theme.file_fg.is_some() {
            self.theme.file_fg = theme.file_fg;
        }
        if theme.file_bg.is_some() {
            self.theme.file_bg = theme.file_bg;
        }
        if theme.file_focus_fg.is_some() {
            self.theme.file_focus_fg = theme.file_focus_fg;
        }
        if theme.file_focus_bg.is_some() {
            self.theme.file_focus_bg = theme.file_focus_bg;
        }
    }
}
