use std::fs::File;
use std::path::Path;
use std::process::Command;

fn main() -> std::io::Result<()> {
    let days = std::env::args().skip(1);
    for day in days {
        let input = File::open(Path::new("input").join(&day))?;
        Command::new("cargo")
            .arg("run")
            .arg(&format!("--bin={}", &day[..2]))
            .stdin(input)
            .status()?;
    }
    Ok(())
}
