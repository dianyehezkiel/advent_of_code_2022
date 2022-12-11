use std::fs::File;
use std::io::{BufRead, BufReader};

const BASE_WIN_SCORE: u8 = 6;
const BASE_TIE_SCORE: u8 = 3;

#[derive(PartialEq)]
enum RPS {
    Rock,
    Paper,
    Scissors,
}

struct Move {
    win_move: RPS,
    defeat_move: RPS,
    score: u8,
}

impl Move {
    pub fn new(curr_move: &RPS) -> Self {
        match curr_move {
            RPS::Rock => Move {
                win_move: RPS::Paper,
                defeat_move: RPS::Scissors,
                score: 1,
            },
            RPS::Paper => Move {
                win_move: RPS::Scissors,
                defeat_move: RPS::Rock,
                score: 2,
            },
            RPS::Scissors => Move {
                win_move: RPS::Rock,
                defeat_move: RPS::Paper,
                score: 3,
            },
        }
    }
}

fn main() {
    let path = std::env::args().nth(1).expect("no path given");
    println!("Try reading file from {path}\n");

    let file = File::open(&path).expect("cannot open given file");
    let reader = BufReader::new(file);

    let mut total_score_1: u32 = 0;
    let mut total_score_2: u32 = 0;

    for line in reader.lines() {
        let str = line.expect("Cannot read line");
        let moves = str.split(" ").collect::<Vec<&str>>();
        let my_move = my_move(moves[1]);
        let opp_move = opp_move(moves[0]);
        let should_move = determine_move(moves[0], moves[1]);
        total_score_1 += count_score(&my_move, &opp_move) as u32;
        total_score_2 += count_score(&should_move, &opp_move) as u32;
    }

    println!("Total Score 1: {total_score_1}");
    println!("Total Score 2: {total_score_2}");
}

fn my_move(moves: &str) -> RPS {
    if moves == "X" {
        return RPS::Rock;
    } else if moves == "Y" {
        return RPS::Paper;
    } else {
        return RPS::Scissors;
    };
}

fn opp_move(moves: &str) -> RPS {
    if moves == "A" {
        return RPS::Rock;
    } else if moves == "B" {
        return RPS::Paper;
    } else {
        return RPS::Scissors;
    };
}

fn count_score(my_move: &RPS, opp_move: &RPS) -> u8 {
    let the_move = Move::new(my_move);

    if my_move == opp_move {
        return BASE_TIE_SCORE + the_move.score;
    }

    if &the_move.defeat_move == opp_move {
        return BASE_WIN_SCORE + the_move.score;
    }

    return the_move.score;
}

fn determine_move(opp: &str, strategy: &str) -> RPS {
    let opp_move = opp_move(opp);
    if strategy == "X" {
        return Move::new(&opp_move).defeat_move;
    } else if strategy == "Y" {
        return opp_move;
    } else {
        return Move::new(&opp_move).win_move;
    }
}
