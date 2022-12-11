use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn count(buf_reader: BufReader<File>) {
    let mut max_calories = [0, 0, 0];
    let mut curr_calories = 0;

    for line in buf_reader.lines() {
        let str = line.expect("Cannot read file");
        if str.is_empty() {
            if curr_calories > max_calories[2] {
                if curr_calories > max_calories[0] {
                    max_calories[2] = max_calories[1];
                    max_calories[1] = max_calories[0];
                    max_calories[0] = curr_calories;
                } else if curr_calories > max_calories[1] {
                    max_calories[2] = max_calories[1];
                    max_calories[1] = curr_calories;
                } else {
                    max_calories[2] = curr_calories;
                };
            };
            curr_calories = 0;
            continue;
        }

        curr_calories += str.parse::<i32>().unwrap();
    }

    let mut total_calories = 0;

    for calories in max_calories {
        total_calories += calories;
    }

    println!("TOTAL TOP THREE CALORIES: {}", total_calories);
}
