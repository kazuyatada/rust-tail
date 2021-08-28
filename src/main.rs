use std::env;
use std::fs::File;
use std::io::{prelude::*, BufReader};

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();

    let filename = &args[1];

    let file = File::open(filename)?;
    let reader = BufReader::new(file);

    println!("File contents:");
    for line in reader.lines() {
        println!("{}", line?);
    }
    Ok(())
}
