use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let f = File::open("../input/02.txt").unwrap();
    let file = BufReader::new(&f);

    let mut p1 = 0;
    let mut p2 = 0;

    let score1 = HashMap::from([
        ("A X", 4),
        ("A Y", 8),
        ("A Z", 3),
        ("B X", 1),
        ("B Y", 5),
        ("B Z", 9),
        ("C X", 7),
        ("C Y", 2),
        ("C Z", 6),
    ]);
    let score2 = HashMap::from([
        ("A X", 3),
        ("A Y", 4),
        ("A Z", 8),
        ("B X", 1),
        ("B Y", 5),
        ("B Z", 9),
        ("C X", 2),
        ("C Y", 6),
        ("C Z", 7),
    ]);

    for (_, line) in file.lines().enumerate() {
        let l = line.unwrap();
        let s = l.as_str();
        let s1 = score1.get(s).unwrap();
        let s2 = score2.get(s).unwrap();

        p1 += s1;
        p2 += s2;
    }

    println!("p1: {}", p1);
    println!("p2: {}", p2);
}
