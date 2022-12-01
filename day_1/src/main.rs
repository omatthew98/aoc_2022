use std::env;
use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    let file = File::open(filename)?;
    let reader = BufReader::new(file);

    let mut temp_sum = 0;
    let mut max_sum = 0;

    for line in reader.lines() {
        let val = line.unwrap();
        if val == "" {
            if temp_sum > max_sum {
                max_sum = temp_sum;
            }
            temp_sum = 0;
        } else {
            let cal_count = val.parse::<i32>().unwrap();
            temp_sum += cal_count;
        }
    }

    println!("max sum: {}", max_sum);

    Ok(())
}
