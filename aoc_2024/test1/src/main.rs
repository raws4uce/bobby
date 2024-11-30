use anyhow::Result;
use util_aoc::aoc_io;
fn main() -> Result<()> {
    aoc_io::lickballs();
    aoc_io::process_file("ballz.txt".to_string(), |line| {
        if line.chars().into_iter().next().is_none() {
            println!("   ots rick james bich ");
        }
    })
}
