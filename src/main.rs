use std::collections::VecDeque;
use std::env;
use std::fs::File;
use std::io::{prelude::*, BufReader};

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();

    let filename = &args[1];

    let file = File::open(filename)?;
    let reader = BufReader::new(file);

    let number_lines = 10;

    let mut deque = VecDeque::with_capacity(number_lines + 1);

    for line in reader.lines() {
        deque.push_back(line?);

        if deque.len() > number_lines {
            deque.pop_front();
        }
    }

    for line in deque.iter() {
        println!("{}", line)
    }

    Ok(())
}
