// #![allow(dead_code)]
// #![allow(unused_imports)]
extern crate termion;

mod app;
mod display;
mod logger;

use clap::Parser;
use display::Display;
use std::io::{stdin, stdout};
use termion::event::{Event, Key};
use termion::input::TermRead;
use termion::raw::IntoRawMode;

use app::App;

#[derive(Parser)]
struct Args {
    #[arg(short, long)]
    selection_path: Option<String>,
    #[clap(index(1))]
    starting_path: Option<String>,
}

fn main() {
    logger::info("Starting app");
    if let Err(e) = run() {
        logger::error(&format!("Error: {}", e));
    }
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    logger::debug(&format!("Args selection_path: {:?}", args.selection_path));
    logger::debug(&format!("Args starting_path: {:?}", args.starting_path));
    let mut app = App::default();

    if let Some(path) = args.selection_path {
        app.set_should_write_to_file(path);
    }
    logger::debug(&format!("default starting_path: {:?}", app.state.path));
    if let Some(path) = args.starting_path {
        app.update_path(path);
        logger::debug(&format!("new starting_path: {:?}", app.state.path));
    }

    let mut display = Display::new();
    let _stdout = stdout().into_raw_mode();
    display.hide_cursor()?;

    app.list_dir()
        .expect("Something went wrong, check if you have permission to read the directory");

    display.render(&app.state)?;
    let stdin = stdin();
    for c in stdin.events() {
        if let Event::Key(Key::Char(key)) = c.as_ref().unwrap() {
            match &key {
                'q' => app.quit(Some(0)),
                'j' => app.next(),
                'k' => app.prev(),
                '-' | 'h' => app.up_dir()?,
                '\n' | 'l' => app.enter()?,
                _ => {}
            }
        }

        if app.should_quit {
            break;
        }

        display.render(&app.state)?;
    }
    display.show_cursor()?;
    std::process::exit(app.exit_code);
}
