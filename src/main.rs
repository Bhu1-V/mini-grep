use std::env;
use std::process;

use minigrep::Config;

fn main() {
    let command:Vec<String> = env::args().collect();

    let config = Config::new(&command).unwrap_or_else(|err| {
        eprintln!("Problem Parsing Arguments : {} ",err);
        process::exit(1);
    });

    if let Err(e) = config.run() {
        eprintln!("Application Error : {} ",e);
        process::exit(1);
    }
}

