use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn count(buf_reader: BufReader<File>) {
    let mut max_calories = 0;
    let mut curr_calories = 0;

    for line in buf_reader.lines() {
        let str = line.expect("Cannot read file");
        if str.is_empty() {
            max_calories = if curr_calories > max_calories {
                curr_calories
            } else {
                max_calories
            };
            curr_calories = 0;
            continue;
        }

        curr_calories += str.parse::<i32>().unwrap();
    }

    println!("MAX CALORIES: {}", max_calories);
}
