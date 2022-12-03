use std::env;
use std::fs::File;
use std::io::{self, prelude::*, BufReader};

const ROCK: &str = "A";
const PAPER: &str = "B";
const SCISSORS: &str = "C";

fn convert<'a>(opp_mv: &'a str, my_mov: &'a str) -> &'a str {
    match my_mov {
        "X" => match opp_mv {
            ROCK => SCISSORS,
            PAPER => ROCK,
            SCISSORS => PAPER,
            _ => ""
        },
        "Y" => opp_mv,
        "Z" => match opp_mv {
            ROCK => PAPER,
            PAPER => SCISSORS,
            SCISSORS => ROCK,
            _ => ""
        },
        _   => ""
    }
}

fn value_of_match(opp_mv: &str, my_mv: &str) -> i32 {
    let  my_mv_conv= convert(opp_mv, my_mv);
    let outcome_val = match (opp_mv, my_mv_conv) {
        // ties
        (ROCK, ROCK) | (PAPER, PAPER) | (SCISSORS, SCISSORS) => 3,
        // wins
        (ROCK, PAPER) | (PAPER, SCISSORS) | (SCISSORS, ROCK) => 6,
        _ => 0
    };
    let play_val = match my_mv_conv {
        ROCK => 1,
        PAPER => 2,
        SCISSORS => 3,
        _ => 0
    };
    return outcome_val + play_val;
}

fn value_of_line(line: String) -> i32 {
    let vec: Vec<&str> = line.split(" ").collect();

    return value_of_match(vec.get(0).unwrap_or_else(|| &""), vec.get(1).unwrap_or_else(|| &""));
}

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    let file = File::open(filename)?;
    let reader = BufReader::new(file);

    let mut score = 0;

    for line in reader.lines() {
        score += value_of_line(line.unwrap_or_default());
    }

    println!("{:?}", score);

    Ok(())
}