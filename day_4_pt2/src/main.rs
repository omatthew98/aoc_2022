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

fn overlaps(tup1: (i32, i32), tup2: (i32, i32)) -> i32 {
    return if is_disjoint(tup1, tup2) || is_disjoint(tup2, tup1) { 0 } else { 1 };
}

fn is_disjoint(tup1: (i32, i32), tup2: (i32, i32)) -> bool {
    return tup1.0 > tup2.0 && tup1.0 > tup2.1;
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

        score += overlaps(*tup1, *tup2);
    }

    println!("{:?}", score);

    Ok(())
}
