use std::fs::File;
use std::io::{Error, Read};

fn open_file() -> Result<File, Error> {
    let f = File::open("/Users/ruipu/projects/Rusting/.claude/steering-state.json")?;
    Ok(f)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut file = open_file()?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    println!("{content}");
    Ok(())
}

