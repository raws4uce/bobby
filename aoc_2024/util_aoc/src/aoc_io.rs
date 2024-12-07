use anyhow::{anyhow, Ok, Result};
use std::{
    any,
    collections::VecDeque,
    error::Error,
    fs::OpenOptions,
    io::{BufRead, BufReader},
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
pub fn process_lil_griddy(file_name: String) -> Result<Vec<Vec<Option<char>>>>
//test case grid
{
    let cols = 17;
    let rows = 16;
    let mut vec: Vec<Vec<Option<char>>> = vec![vec![None; cols]; rows];
    let file = OpenOptions::new()
        .read(true)
        .open(format!("/home/raws4uce/aoc_2024/z_dump/{}", file_name))?;

    let buf = BufReader::new(file);
    let mut wordsearch: Vec<String> = vec![];
    for line in buf.lines() {
        wordsearch.push(line?);
    }
    for i in 0..wordsearch.len() {
        let chars: Vec<char> = wordsearch.get(i).unwrap().chars().collect();
        if let Some(row) = vec.get_mut(i + 3) {
            for (j, char) in chars.iter().enumerate() {
                if let Some(cell) = row.get_mut(j + 3) {
                    *cell = Some(*char);
                }
            }
        }
    }

    //Err(anyhow!("bitch"))
    Ok(vec)
}

pub fn process_griddy(file_name: String) -> Result<Vec<Vec<Option<char>>>>
//this is for d4, idea to have a buffer zone, and a sliding window of a 7by7 grid the buffer
//zone should be 3k,3j,3h,4l (vim motions euphemism for how many rows/columns the grid would
//have), this is purely to evade the out of bounds error, I am yet to find out how to do such
//thing, so next best thing is mitigation.. lazy ik
{
    let cols = 146;
    let rows = 145;
    let mut vec: Vec<Vec<Option<char>>> = vec![vec![None; cols]; rows];
    let file = OpenOptions::new()
        .read(true)
        .open(format!("/home/raws4uce/aoc_2024/z_dump/{}", file_name))?;

    let buf = BufReader::new(file);
    let mut wordsearch: Vec<String> = vec![];
    for line in buf.lines() {
        wordsearch.push(line?);
    }
    for i in 0..wordsearch.len() {
        let chars: Vec<char> = wordsearch.get(i).unwrap().chars().collect();
        if let Some(row) = vec.get_mut(i + 3) {
            for (j, char) in chars.iter().enumerate() {
                if let Some(cell) = row.get_mut(j + 3) {
                    *cell = Some(*char);
                }
            }
        }
    }

    //Err(anyhow!("bitch"))
    Ok(vec)
}
pub fn mid_griddy(file_name: String) -> Result<Vec<Vec<Option<char>>>>
//pt 2, ig a 3x3, reduce buffer accordingly`
{
    let cols = 142;
    let rows = 141;
    let mut vec: Vec<Vec<Option<char>>> = vec![vec![None; cols]; rows];
    let file = OpenOptions::new()
        .read(true)
        .open(format!("/home/raws4uce/aoc_2024/z_dump/{}", file_name))?;

    let buf = BufReader::new(file);
    let mut wordsearch: Vec<String> = vec![];
    for line in buf.lines() {
        wordsearch.push(line?);
    }
    for i in 0..wordsearch.len() {
        let chars: Vec<char> = wordsearch.get(i).unwrap().chars().collect();
        if let Some(row) = vec.get_mut(i + 1) {
            for (j, char) in chars.iter().enumerate() {
                if let Some(cell) = row.get_mut(j + 1) {
                    *cell = Some(*char);
                }
            }
        }
    }

    //Err(anyhow!("bitch"))
    Ok(vec)
}
//let x = wordsearch.get(i).unwrap();
//let y = x.chars().nth(j);
//let ref mut z: Vec<Option<char>> = vec.get(i+3).unwrap().to_vec();
//z.push(y);
//vec.insert(i+3, z.to_vec());
//let ref mut z: Vec<Option<char>> = vec.get(i).unwrap().to_vec();
//z.push(None);
//vec.insert(i, z.to_vec());
// for
pub fn line_vec<F>(file_name: String, mut operation: F) -> Result<()>
where
    F: FnMut(Vec<isize>),
{
    let file = OpenOptions::new()
        .read(true)
        .open(format!("/home/raws4uce/aoc_2024/z_dump/{}", file_name))?;

    let buf = BufReader::new(file);

    for line in buf.lines() {
        let line = line?;
        // god bless rust
        let vnew: Vec<isize> = line
            .split_whitespace()
            .filter_map(|s| s.parse().ok())
            .collect();
        //println!("line : {:?}", vnew);
        operation(vnew);
    }
    Ok(())
}

//holy grail is not to be touched
pub fn lickballs() {
    println!("lickballs from io crate");
}
