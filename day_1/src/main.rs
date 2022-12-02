use std::env;
use std::fs::File;
use std::io::{self, prelude::*, BufReader};

use std::collections::BinaryHeap;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    let file = File::open(filename)?;
    let reader = BufReader::new(file);

    let mut temp_sum = 0;
    let mut heap = BinaryHeap::new();

    println!("Heap: {:?}", heap);

    for line in reader.lines() {
        let val = line.unwrap();
        if val == "" {
            // end of elf's calorie counts, determine if should keep
            let min_heap_val = match heap.peek() {
                Some(val) => -val,
                None => std::i32::MIN
            };
            if heap.len() < 3  {
                heap.push(-temp_sum);
            } else if temp_sum > min_heap_val{
                heap.pop();
                heap.push(-temp_sum);
            }
            temp_sum = 0;
        } else {
            let cal_count = val.parse::<i32>().unwrap();
            temp_sum += cal_count;
        }
    }

    println!("Heap: {:?}", heap);
    println!("sum: {:?}", -1 * heap.iter().sum::<i32>());

    Ok(())
}
