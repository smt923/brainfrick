mod bf;

use bf::BrainFrick;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::env;

fn main() {
    let path = match env::args().nth(1) {
        Some(val) => val,
        None => {
            eprintln!("Usage: brainfrick <path to program to run>");
            std::process::exit(1)
        }
    };
    let mut file = match File::open(&path) {
        Err(why) => {
            eprintln!("Could not open file for reading: {}", why.description());
            std::process::exit(1)
        },
        Ok(file) => file,
    };
    let mut program = String::new();
    if let Err(why) = file.read_to_string(&mut program) {
        eprintln!("Could not read file to string: {}", why.description());
        std::process::exit(1)
    }

    let mut bf = BrainFrick::new();
    bf.load(program);
    
    while !bf.exit {
        bf.eval();
    }
}
