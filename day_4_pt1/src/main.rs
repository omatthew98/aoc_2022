use std::env;
use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn tup_from_str(s: &str) -> (i32, i32) {
    let range_lims: Vec<&str> = s.split("-").collect();

    match range_lims.len() {
        2 => (range_lims.get(0).unwrap_or_else(|| &"0").parse().unwrap(),
              range_lims.get(1).unwrap_or_else(|| &"0").parse().unwrap()),
        _ => (0, 0)
    }
}

fn is_fully_contained(tup1: (i32, i32), tup2: (i32, i32)) -> i32 {
    // returns 1 if tup1 is fully contained in tup2 else 0
    return if contained_helper(tup1, tup2) || contained_helper(tup2, tup1) { 1 } else { 0 };
}

fn contained_helper (tup1: (i32, i32), tup2: (i32, i32)) -> bool {
    // returns true if tup1 is fully contained in tup2 else false
    return tup1.0 >= tup2.0 && tup1.1 <= tup2.1;
}

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    let file = File::open(filename)?;
    let reader = BufReader::new(file);

    let mut score = 0;

    for line in reader.lines() {
        let line_res = line.unwrap_or_default();

        let pairs: Vec<(i32, i32)> = line_res.split(",").map(|p| tup_from_str(p)).collect();

        let (tup1, tup2) = (
            pairs.get(0).unwrap_or_else(|| &(0i32, 0i32)),
            pairs.get(1).unwrap_or_else(|| &(0i32, 0i32))
        );

        score += is_fully_contained(*tup1, *tup2);
    }

    println!("{:?}", score);

    Ok(())
}
