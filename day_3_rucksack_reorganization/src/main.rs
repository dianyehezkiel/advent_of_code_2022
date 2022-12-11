use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let path = std::env::args().nth(1).expect("no path given");
    println!("Try reading file from {path}\n");

    let file = File::open(&path).expect("cannot open given file");
    let reader = BufReader::new(file);

    let mut same_items_count: u32 = 0;
    let mut badges_count: u32 = 0;
    let mut str_vec: Vec<String> = vec![];

    for line in reader.lines() {
        let str = line.expect("Cannot read line");
        same_items_count += count_same_items(&str);
        str_vec.push(str);

        if str_vec.len() == 3 {
            badges_count += count_badges(&str_vec);
            str_vec.clear();
        }
    }

    println!("Same Items Count: {same_items_count}");
    println!("Badges Count: {badges_count}");
}

fn char_to_num(c: &char) -> u32 {
    let n = *c as u32;

    if n > 96 {
        return n - 96;
    }

    return n - 64 + 26;
}

fn count_same_items(str: &str) -> u32 {
    let half_len = str.len() / 2;
    let mut hashmap = HashMap::new();
    let mut res: u32 = 0;

    for (i, c) in str.chars().enumerate() {
        if i < half_len {
            hashmap.insert(c, char_to_num(&c));
        } else {
            match hashmap.get(&c) {
                Some(&n) => {
                    res += n;
                    hashmap.remove(&c);
                }
                _ => continue,
            }
        }
    }

    return res;
}

fn count_badges(v: &Vec<String>) -> u32 {
    let mut hashmap: HashMap<char, usize> = HashMap::new();

    for (i, str) in v.iter().enumerate() {
        for c in str.chars() {
            if i == 0 {
                hashmap.insert(c, i);
            } else {
                match hashmap.get(&c) {
                    Some(&n) => {
                        if n == i - 1 {
                            hashmap.insert(c, i);
                        }

                        if hashmap[&c] == 2 {
                            return char_to_num(&c);
                        }
                    }
                    _ => continue,
                }
            }
        }
    }

    return 0;
}
