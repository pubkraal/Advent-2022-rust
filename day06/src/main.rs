use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let f = File::open("../input/06.txt").unwrap();
    let mut file = BufReader::new(&f);

    let mut data = String::new();
    file.read_line(&mut data).unwrap();

    println!("p1: {}", find_marker(&data, 4));
    println!("p2: {}", find_marker(&data, 14));
}

fn find_marker(data: &String, size: usize) -> usize {
    let len = data.len();
    for n in 0..(len - size) {
        let slice = &data[n..(n + size)];
        let mut s = HashSet::new();
        for (_, c) in slice.chars().enumerate() {
            s.insert(c);
        }
        if s.len() == size {
            return n + size;
        }
    }
    return 0;
}
