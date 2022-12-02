use std::env;
use std::fs::File;
use std::io::{self, prelude::*, BufReader};


#[derive(Debug, PartialEq)]
enum Move {
    None,
    Rock,
    Paper,
    Scissors,
}

fn move_from_str(input: &str) -> Move {
    match input {
        "A" | "X" => Move::Rock,
        "B" | "Y" => Move::Paper,
        "C" | "Z" => Move::Scissors,
        _   => Move::None
    }
}

fn value_of_move(mv: &Move) -> i32 {
    match mv {
        Move::None => 0,
        Move::Rock => 1,
        Move::Paper => 2,
        Move::Scissors => 3
    }
}

fn value_of_match(opp_mv: &Move, my_mov: &Move) -> i32 {
    match (opp_mv, my_mov) {
        // ties
        (mv1, mv2) if mv1 == mv2 => 3,
        // wins
        (Move::Rock, Move::Paper) | (Move::Paper, Move::Scissors) | (Move::Scissors, Move:: Rock) => 6,
        _ => 0
    }
}

fn move_tuple_from_line(line: String) -> (Move, Move) {
    let vec: Vec<&str> = line.split(" ").collect();
    (
        move_from_str(vec.get(0).unwrap_or_else(|| &"")),
        move_from_str(vec.get(1).unwrap_or_else(|| &""))
    )
}

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    let file = File::open(filename)?;
    let reader = BufReader::new(file);

    let mut score = 0;

    for line in reader.lines() {
        let tup = move_tuple_from_line(line.unwrap_or_default());
        score += value_of_move(&tup.1);
        score += value_of_match(&tup.0, &tup.1);
    }

    println!("{:?}", score);

    Ok(())
}
