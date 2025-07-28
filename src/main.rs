use std::io::{self, BufReader};
use std::io::prelude::*;
use std::fs::File;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(author = "John Crickett", version, about="rscat, a simple cat clone in Rust")]
struct Arguments {
    /// Number the output lines, starting at 1.
    #[arg(short, action = clap::ArgAction::SetTrue)]
    n: bool,

    /// Number the non-blank output lines, starting at 1
    #[arg(short, action = clap::ArgAction::SetTrue)]
    b: bool,

    files: Vec<String>,
}

fn cat(n: bool, b : bool, line_number: i32, line : String) -> i32 {
    if !n && !b {
        println!("{}", line);
        line_number
    } else {
        if b && line.len() == 0 {
            println!();
            return line_number;
        }
        println!("{} {}", line_number, line);
        line_number + 1
    }
}

fn main() -> io::Result<()> {
    let args = Arguments::parse();
    let mut line_number = 1;

    if args.files.len() == 0 || args.files[0] == "-" {
        let stdin = io::stdin();
        for line in stdin.lock().lines() {
            let line = match line {
                Ok(line) => line,
                Err(error) => panic!("Problem reading the input: {error:?}"),
            };
            line_number = cat(args.n, args.b, line_number, line);
        }
    } else {
        for file in args.files {
            let f = File::open(file).expect("unable to open file");
            let f = BufReader::new(f);

            for line in f.lines() {
                let line = match line {
                    Ok(line) => line,
                    Err(error) => panic!("Problem reading the input: {error:?}"),
                };
                line_number = cat(args.n, args.b, line_number, line);
            }
        }
    }
    Ok(())
}
