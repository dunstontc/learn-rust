extern crate minigrep;
use minigrep::Config;

extern crate ansi_term;
use ansi_term::Color::{Blue, Red};

use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", Red.paint(err).to_string());
        process::exit(1);
    });

    println!("Searching for {}", Blue.paint(config.query.to_string()));
    println!("In file {}", Blue.paint(config.filename.to_string()));

    if let Err(err) = minigrep::run(config) {
        println!("Application error: {}", Red.paint(err.to_string()));
        process::exit(1);
    }
}


