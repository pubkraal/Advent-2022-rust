//use itertools::Itertools;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let f = File::open("../input/01.txt").unwrap();
    let file = BufReader::new(&f);

    let mut elves = Vec::new();

    let mut curelf = 0;

    for (_, line) in file.lines().enumerate() {
        let x = line.unwrap();
        if x == "" {
            elves.push(curelf);
            curelf = 0;
            continue;
        }
        let val = x.parse::<i32>().unwrap();
        curelf += val;
    }
    elves.push(curelf);

    elves.sort();
    elves.reverse();

    println!("p1: {}", elves[0]);
    println!("p2: {}", elves[0] + elves[1] + elves[2]);
}
