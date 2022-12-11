use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let path = std::env::args().nth(1).expect("no path given");

    let file = File::open(&path).expect("cannot open given file");
    let reader = BufReader::new(file);

    let mut fully_contains_count: u32 = 0;
    let mut overlap_count: u32 = 0;

    for line in reader.lines() {
        let str = line.expect("cannot read line");
        let sec_arr = to_sec_array(&str);
        if is_fully_contained(&sec_arr) {
            fully_contains_count += 1;
        }
        if is_overlap(&sec_arr) {
            overlap_count += 1;
        }
    }

    println!("Total fully contained assignments: {fully_contains_count}");
    println!("Total overlap assignments: {overlap_count}");
}

fn to_sec_array(str: &String) -> Vec<Vec<u8>> {
    let sec_arr = str
        .split(",")
        .map(|sec| {
            sec.split("-")
                .map(|sec_id| sec_id.parse().expect("cannot convert to number"))
                .collect()
        })
        .collect::<Vec<Vec<u8>>>();

    return sec_arr;
}

fn is_fully_contained(sec_arr: &Vec<Vec<u8>>) -> bool {
    if sec_arr[0][0] <= sec_arr[1][0] && sec_arr[0][1] >= sec_arr[1][1] {
        return true;
    };
    if sec_arr[0][0] >= sec_arr[1][0] && sec_arr[0][1] <= sec_arr[1][1] {
        return true;
    };

    return false;
}

fn is_overlap(sec_arr: &Vec<Vec<u8>>) -> bool {
    if sec_arr[0][0] >= sec_arr[1][0] && sec_arr[0][1] <= sec_arr[1][1] {
        return true;
    };
    if sec_arr[0][0] <= sec_arr[1][0] && sec_arr[0][1] >= sec_arr[1][0] {
        return true;
    };
    if sec_arr[0][1] >= sec_arr[1][1] && sec_arr[0][0] <= sec_arr[1][1] {
        return true;
    };

    return false;
}
