use std::env;
use std::io::{self, BufReader};
use std::io::prelude::*;
use std::fs::File;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let files = &args[1..];

    if files.len() == 0 || files[0] == "-" {
        let stdin = io::stdin();
        for line in stdin.lock().lines() {
            println!("{}", line?);
        }
    } else {
        for file in files {
            let f = File::open(file).expect("unable to open file");
            let f = BufReader::new(f);

            for line in f.lines() {
                let line = line?;
                println!("{line}");
            }
        }
    }

    Ok(())
}
