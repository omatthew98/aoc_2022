use std::env;
use std::fs::File;
use std::collections::LinkedList;
use std::io::{self, prelude::*, BufReader};
use regex::Regex;

fn stack_ind_to_char_pos(stack_ind: usize) -> usize {
    // return the character position for the corresponding zero-indexed stack
    return 1 + stack_ind * 4;
}

fn parse_start_state(state: &str) -> Vec<LinkedList<char>> {
    let lines = state.split("\n").collect::<Vec<_>>();
    let num_line = lines.last().unwrap_or_else(|| &"");

    /*
    leading space, last char and trailing space = 3, each other char grouping is 4 chars
    3 + 4(n-1) = l ==> n = (l+1) / 4
    */
    let num_stacks = (num_line.len() + 1) / 4;
    let mut stacks = vec![LinkedList::<char>::new(); num_stacks];


    let max_height = lines.len() - 1;
    for row_ind in (0..max_height).rev() {
        let line: Vec<char> = lines.get(row_ind).unwrap_or_else(|| &"").chars().collect();
        for stack_ind in 0..num_stacks {
            let stack = stacks.get_mut(stack_ind);
            let c = line.get(stack_ind_to_char_pos(stack_ind)).unwrap_or_else(|| &' ');
            if c.is_ascii_uppercase() && stack.is_some() {
                stack.unwrap().push_front(*c);
            }
        }
    }

    stacks
}

fn perform_move(stacks: &mut Vec<LinkedList<char>>, from_ind: usize, to_ind: usize, num_moved: usize) -> () {
    let from_stack = stacks.get_mut(from_ind - 1).unwrap();
    let to_move: Vec<char> = (0..num_moved).map(|_| from_stack.pop_front().unwrap()).collect();
    let to_stack = stacks.get_mut(to_ind - 1).unwrap();

    // pt 1, insert in order (this mimics crate by crate move)
    // to_move.iter().for_each(|c| to_stack.push_front(*c));

    // pt 2, insert in reverse order (this mimics block of crates move)
    to_move.iter().rev().for_each(|c| to_stack.push_front(*c));
}

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    let file = File::open(filename)?;
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents)?;

    let parts = contents.split("\n\n").collect::<Vec<_>>();

    let (start_state, moves) = match parts.len() {
        2 => (parts.get(0).unwrap_or_else(|| &""), parts.get(1).unwrap_or_else(|| &"")),
        _ => (&"", &"")
    };

    let mut stacks = parse_start_state(start_state);

    for mv in moves.split("\n") {
        let re = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();
        let caps = re.captures(mv);
        if caps.is_some() {
            let caps_res = caps.unwrap();
            let (num_moved, from_ind, to_ind) = (caps_res.get(1).map_or("", |m| m.as_str()).parse::<usize>().unwrap(),
                                                 caps_res.get(2).map_or("", |m| m.as_str()).parse::<usize>().unwrap(),
                                                 caps_res.get(3).map_or("", |m| m.as_str()).parse::<usize>().unwrap());
            perform_move(&mut stacks, from_ind, to_ind, num_moved);
        }
    }

    println!("ans: {:?}", stacks.iter().map(|l| (*l).front().unwrap()).collect::<String>());


    Ok(())
}