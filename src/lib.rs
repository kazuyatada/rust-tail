use std::collections::VecDeque;
use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
pub struct Opt {
    #[structopt(short, default_value = "10")]
    number: usize,
    #[structopt(parse(from_os_str))]
    file: PathBuf,
}

pub fn run(opt: Opt) -> std::io::Result<()> {
    let file = File::open(opt.file)?;
    let reader = BufReader::new(file);

    let mut deque = VecDeque::with_capacity(opt.number + 1);

    for line in reader.lines() {
        deque.push_back(line?);

        if deque.len() > opt.number {
            deque.pop_front();
        }
    }

    for line in deque.iter() {
        println!("{}", line)
    }

    Ok(())
}
