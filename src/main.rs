use std::env;
use std::io::{self, BufReader};
use std::io::prelude::*;
use std::fs::File;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let file = &args[1];

    if file == "-" {
        let stdin = io::stdin();
        for line in stdin.lock().lines() {
            println!("{}", line?);
        }
    } else {
        let f = File::open(file).expect("unable to open file");
        let f = BufReader::new(f);

        for line in f.lines() {
            let line = line?;
            println!("{line}");
        }
    }

    Ok(())
}
