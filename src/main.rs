use std::env;
use std::io::{self, BufReader};
use std::io::prelude::*;
use std::fs::File;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(author = "John Crickett", version, about="rscat, a simple cat clone in Rust")]
struct Arguments {
    /// Number the output lines, starting at 1.
    #[arg(short, long, action = clap::ArgAction::SetFalse)]
    n: Option<bool>,
    files: Vec<String>,
}

fn main() -> io::Result<()> {
    let args = Arguments::parse();
    let mut line_number = 1;

    if args.files.len() == 0 || args.files[0] == "-" {
        let stdin = io::stdin();
        for line in stdin.lock().lines() {
            if args.n.unwrap_or(false) {
                println!("{}", line?);
            } else {
                println!("{} {}", line_number, line?);
                line_number += 1;
            }
        }
    } else {
        for file in args.files {
            let f = File::open(file).expect("unable to open file");
            let f = BufReader::new(f);

            for line in f.lines() {
                let line = line?;
                if args.n.unwrap_or(false) {
                    println!("{}", line);
                } else {
                    println!("{} {}", line_number, line);
                    line_number += 1;
                }
            }
        }
    }

    Ok(())
}
