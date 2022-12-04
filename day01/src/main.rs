//use itertools::Itertools;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let f = File::open("../input/01.txt").unwrap();
    let file = BufReader::new(&f);
    for (idx, line) in file.lines().enumerate() {
        let x = line.unwrap();
        if x == "" {
            println!("New elf on: {}", idx);
            continue;
        }
    }
}
