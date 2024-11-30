use anyhow::{Ok, Result};
use std::{
    fs::OpenOptions,
    io::{BufRead, BufReader, Read},
};
//for each new line
pub fn process_file<F>(file_name: String, mut operation: F) -> Result<()>
where
    F: FnMut(String),
{
    let file = OpenOptions::new()
        .read(true)
        .open(format!("/home/raws4uce/aoc_2024/z_dump/{}", file_name))?;

    let buf = BufReader::new(file);

    for line in buf.lines() {
        let line = line?;
        //println!("line : {line}");
        operation(line);
    }
    Ok(())
}
// for

//holy grail is not to be touched
pub fn lickballs() {
    println!("lickballs from io crate");
}
