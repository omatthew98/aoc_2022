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

fn build_set(s: &str) -> HashSet<u32> {
    let mut set = HashSet::new();

    for c in s.chars() {
        set.insert(char_to_int(c));
    }

    return set;
}

fn priority_from_group(e1: &str, e2: &str, e3: &str) -> u32 {
    let s1 = build_set(e1);
    let s2 = build_set(e2);
    let s3 = build_set(e3);

    let int: Vec<u32> = s1.intersection(&s2)
                .copied().collect::<HashSet<_>>()
                .intersection(&s3)
                .copied().collect();

    let priority = int.get(0).unwrap_or_else(|| &0u32);

    return *priority;
}

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    let file = File::open(filename)?;
    let reader = BufReader::new(file);

    let mut score = 0;
    let mut reader_lines = reader.lines().map(|l| l.unwrap()).peekable();

    while reader_lines.peek().is_some() {
        let e1 = reader_lines.next().unwrap_or_default();
        let e2 = reader_lines.next().unwrap_or_default();
        let e3 = reader_lines.next().unwrap_or_default();

        score += priority_from_group(&e1, &e2, &e3);
    }

    println!("{:?}", score);

    Ok(())
}
