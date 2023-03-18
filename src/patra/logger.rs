use std::io::Write;
use std::fs::OpenOptions;

pub fn log(msg: &str) {
    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open("/tmp/patra-error.log").unwrap();
    writeln!(file, "{}", msg).unwrap();
}
