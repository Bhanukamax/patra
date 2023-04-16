// #![allow(dead_code)]
// #![allow(unused_imports)]
extern crate termion;

mod app;
mod config;
mod display;
mod logger;

use app::{CommandType, UiMode};
use clap::Parser;
use config::Config;
use display::Display;
use std::io::{stdin, stdout};
use termion::event::{Event, Key};
use termion::input::TermRead;
use termion::raw::IntoRawMode;

use serde::Deserialize;

use app::App;

#[derive(Deserialize, Debug, Parser)]
struct Args {
    // last selection path
    #[arg(short, long)]
    selection_path: Option<String>,
    // config override
    #[arg(short, long)]
    config: Option<String>,
    // File path
    #[clap(index(1))]
    starting_path: Option<String>,
}
type DebugMode = bool;

fn main() {
    let debug_mode: DebugMode = match std::env::var("DEBUG") {
        Ok(val) => matches!(val.as_str(), "1"),
        Err(_) => false,
    };

    logger::info("Starting app");
    let mut config = Config::load().unwrap_or(Config::default());

    dbg!(config.clone());
    if !debug_mode {
        if let Err(e) = run(&mut config) {
            logger::error(&format!("Error: {}", e));
        }
    }
}

fn run(config: &mut Config) -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    logger::debug(&format!("Args selection_path: {:?}", args.selection_path));
    logger::debug(&format!("Args starting_path: {:?}", args.starting_path));
    logger::debug(&format!("Args theme_file_focus_fg: {:?}", args.config));
    // let mut override_theme = config.theme.clone();

    // config.update_theme(override_theme);
    let mut app = App::default();

    if let Some(path) = args.config {
        config.load_from_path(path)?;
    }

    if let Some(path) = args.selection_path {
        app.set_should_write_to_file(path);
    }
    logger::debug(&format!("default starting_path: {:?}", app.state.path));
    if let Some(path) = args.starting_path {
        app.update_path(path);
        logger::debug(&format!("new starting_path: {:?}", app.state.path));
    }

    let mut display = Display::new(&config.theme);
    let _stdout = stdout().into_raw_mode();
    display.hide_cursor()?;

    app.list_dir()
        .expect("Something went wrong, check if you have permission to read the directory");

    display.render(&app)?;
    let stdin = stdin();
    for c in stdin.events() {
        logger::debug(&format!("{:?}", &c));
        if let Event::Key(key) = c.as_ref().unwrap() {
            match app.ui_mode {
                UiMode::Normal => match &key {
                    Key::Char('q') => app.quit(Some(0)),
                    Key::Char('j') => app.next(),
                    Key::Char('k') => app.prev(),
                    Key::Char('%') => app.run_command(CommandType::CreateFile),
                    Key::Char('d') => app.run_command(CommandType::CreateDir),
                    Key::Char('-') | Key::Char('h') => app.up_dir()?,
                    Key::Char('\n') | Key::Char('l') => app.enter()?,
                    _ => {}
                },
                UiMode::Command(CommandType::CreateDir, _) => match &key {
                    Key::Esc | Key::Ctrl('c') => app.run_command(CommandType::GoToNormalMode),
                    Key::Char('\n') => app.try_create_dir()?,
                    Key::Char(char) => app.insert_command_char(char),
                    Key::Backspace => app.delete_command_char(),
                    _ => {}
                },
                UiMode::Command(CommandType::CreateFile, _) => match &key {
                    Key::Esc | Key::Ctrl('c') => app.run_command(CommandType::GoToNormalMode),
                    // TODO: add select file and exit option for embedding
                    Key::Char('\n') => app.try_create_file()?,
                    Key::Char(char) => app.insert_command_char(char),
                    Key::Backspace => app.delete_command_char(),
                    _ => {}
                },
                _ => {}
            }
        }

        if app.should_quit {
            break;
        }

        display.render(&app)?;
    }
    display.show_cursor()?;
    std::process::exit(app.exit_code);
}
