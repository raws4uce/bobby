fn main() {
    println!("where i test the util");

    use util_aoc::aoc_io;
    aoc_io::lickballs();

    match aoc_io::read_file("ballz.txt".to_string()) {
        Ok(l) => {}
        Err(z) => {
            eprintln!("{z:?}");
        }
    }
}
