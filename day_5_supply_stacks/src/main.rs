use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let path = std::env::args().nth(1).expect("no path given");

    let file = File::open(&path).expect("cannot open given file");
    let reader = BufReader::new(file);

    let mut stacks_one: Vec<Vec<char>> = vec![];
    let mut stacks_two: Vec<Vec<char>> = vec![];
    let mut stacks_added = false;

    for line in reader.lines() {
        let str = line.expect("cannot read line");
        if str.is_empty() {
            stacks_added = true;
            println!("Initial Stacks:\n{:?}", stacks_one);
            continue;
        }

        if stacks_added {
            move_cargo(&str_to_procedure(&str), &mut stacks_one);
            move_multi_cargo(&str_to_procedure(&str), &mut stacks_two);
        } else {
            let char_vec = str
                .split(",")
                .map(|c| c.chars().next().expect("something went wrong"))
                .collect::<Vec<char>>();
            stacks_one.push((&char_vec).to_owned());
            stacks_two.push((&char_vec).to_owned());
        }
    }

    println!("Final Stacks One:\n{:?}", stacks_one);
    println!("Final Stacks Two:\n{:?}", stacks_two);

    print!("\nFirst Result:\n");
    for stack in stacks_one {
        match stack.last() {
            Some(c) => print!("{c}"),
            _ => continue,
        }
    }
    print!("\nSecond Result:\n");
    for stack in stacks_two {
        match stack.last() {
            Some(c) => print!("{c}"),
            _ => continue,
        }
    }
    print!("\n");
}

fn str_to_procedure(str: &str) -> Vec<u8> {
    let mut procedure: Vec<u8> = vec![];
    let vec_str = str.split(" ").collect::<Vec<&str>>();

    for val in vec_str {
        match val.parse() {
            Ok(x) => procedure.push(x),
            _ => continue,
        }
    }

    return procedure;
}

fn move_cargo(procedure: &Vec<u8>, stacks: &mut Vec<Vec<char>>) {
    if procedure.len() != 3 {
        return;
    }

    let cargo_count = procedure[0] as usize;
    let from = (procedure[1] - 1) as usize;
    let to = (procedure[2] - 1) as usize;

    for _ in 0..cargo_count {
        let cargo = stacks[from]
            .pop()
            .expect(format!("No cargo left in stack {from}").as_str());

        stacks[to].push(cargo);
    }
}

fn move_multi_cargo(procedure: &Vec<u8>, stacks: &mut Vec<Vec<char>>) {
    if procedure.len() != 3 {
        return;
    }

    let cargo_taken = procedure[0] as usize;
    let from = (procedure[1] - 1) as usize;
    let cargo_left = stacks[from].len() - cargo_taken;
    let to = (procedure[2] - 1) as usize;
    let cargos = stacks[from].drain(cargo_left..).collect::<Vec<char>>();

    for cargo in cargos {
        stacks[to].push(cargo);
    }
}
