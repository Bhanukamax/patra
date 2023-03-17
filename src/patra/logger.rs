use std::io::Write;

pub fn log(msg: &str) -> Result<(), std::io::Error> {
    let out = std::io::stderr();
    writeln!(&out, "{}", msg)?;
    Ok(())
}
