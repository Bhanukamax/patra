#![allow(dead_code)]
use directories::ProjectDirs;
use serde::Deserialize;
use std::fs;

use crate::logger;

#[derive(Deserialize, Debug, Default, Clone)]
pub struct Theme {
    pub file_fg: Option<String>,
    pub file_bg: Option<String>,
    pub file_focus_fg: Option<String>,
    pub file_focus_bg: Option<String>,
    pub dir_fg: Option<String>,
    pub dir_slash: Option<String>,
    pub command_fg: Option<String>,
}

#[derive(Deserialize, Debug, Default, Clone)]
pub struct Config {
    pub theme: Theme,
}

impl Config {
    pub fn load_from_path(&mut self, path: String) -> Result<(), Box<dyn std::error::Error>> {
        let conf_content: String = fs::read_to_string(&path).unwrap_or("".to_string());
        let config: Config = toml::from_str(&conf_content).unwrap_or(Config::default());

        self.theme = config.theme.to_owned();
        self.update_theme(config.theme);
        // logger::debug(&format!("path {}", self.theme));
        logger::debug(&format!("path {}", path));
        Ok(())
    }

    pub fn load() -> Result<Self, Box<dyn std::error::Error>> {
        if let Some(proj_dirs) = ProjectDirs::from("com", "bmax", "Patra") {
            let conf = proj_dirs.config_dir().join("config.toml");
            let conf_content: String = fs::read_to_string(conf).unwrap_or("".to_string());
            let config: Config =
                toml::from_str(&conf_content).unwrap_or(Config::default());
            return Ok(config);
        }
        Err("No config file found".into())
    }

    pub fn update_theme(&mut self, theme: Theme) {
        if theme.file_fg.is_some() {
            self.theme.file_fg = theme.file_fg;
        }
        if theme.dir_fg.is_some() {
            self.theme.dir_fg = theme.dir_fg;
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
        if theme.command_fg.is_some() {
            self.theme.command_fg = theme.command_fg;
        }
        if theme.dir_slash.is_some() {
            self.theme.dir_slash = theme.dir_slash;
        }
    }
}
