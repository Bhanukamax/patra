use std::env;
use std::fs::OpenOptions;
use std::io::Write;

fn log(msg: &str) {
    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open("/tmp/patra-error.log")
        .unwrap();
    writeln!(file, "{}", msg).unwrap();
}

pub fn info(msg: &str) {
    if let Ok(v) = env::var("RUST_LOG") {
        match v.as_str() {
            "info" | "error" | "debug" | "all" => log(msg),
            _ => {}
        }
    }
}

pub fn debug(msg: &str) {
    if let Ok(v) = env::var("RUST_LOG") {
        match v.as_str() {
            "debug" | "all" => log(msg),
            _ => {}
        }
    }
}

pub fn error(msg: &str) {
    if let Ok(v) = env::var("RUST_LOG") {
        match v.as_str() {
            "error" | "all" => log(msg),
            _ => {}
        }
    }
}
