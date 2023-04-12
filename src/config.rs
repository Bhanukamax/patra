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
    file_fg = '#ff0088'
    file_focus_fg = '#fafafa'
"#,
        )
        .unwrap_or(Config::default());
        Ok(config)
    }
}
