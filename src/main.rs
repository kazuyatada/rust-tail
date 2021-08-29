use std::process;
use structopt::StructOpt;
use tail::*;

fn main() {
    let opt = Opt::from_args();
    if let Err(e) = run(opt) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}
