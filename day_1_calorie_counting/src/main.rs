use std::fs::File;
use std::io::BufReader;

mod max_calories;
mod total_top_three_calories;

fn main() {
    let path = std::env::args().nth(1).expect("no path given");
    println!("Try reading file from {path}\n");

    let file = File::open(&path).expect("cannot open given file");
    max_calories::count(BufReader::new(file));

    let file = File::open(&path).expect("cannot open given file");
    total_top_three_calories::count(BufReader::new(file));
}
