use std::env;
use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use std::collections::HashSet;

fn char_to_int(c: char) -> u32 {
    if c.is_ascii_uppercase() {
        return (c as u32) - 65 + 27;
    } else if c.is_ascii_lowercase() {
        return (c as u32) - 97 + 1;
    } else {
        return 0;
    }
}

fn priority_from_compartments(cmp1: &str, cmp2: &str) -> u32 {
    let mut set = HashSet::new();

    for c in cmp1.chars() {
        set.insert(c);
    }

    for c in cmp2.chars() {
        if set.contains(&c) {
            return char_to_int(c);
        }
    }

    return 0;
}

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    let file = File::open(filename)?;
    let reader = BufReader::new(file);

    let mut score = 0;

    for line in reader.lines() {
        let line_res = line.unwrap_or_default();
        let len = line_res.len();

        let cmp1 = &line_res[..len / 2];
        let cmp2 = &line_res[len / 2..];

        score += priority_from_compartments(cmp1, cmp2);
    }

    println!("{:?}", score);

    Ok(())
}
