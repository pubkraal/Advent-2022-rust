use scanf::sscanf;
use std::fs::File;
use std::io::{BufRead, BufReader};

struct Pair {
    start: i32,
    end: i32,
}

fn main() {
    let f = File::open("../input/04.txt").unwrap();
    let file = BufReader::new(&f);

    let mut p1 = 0;
    let mut p2 = 0;

    let mut a_start = 0;
    let mut a_end = 0;
    let mut b_start = 0;
    let mut b_end = 0;

    for (_, line) in file.lines().enumerate() {
        let l = line.unwrap();
        sscanf!(
            &l,
            "{i32}-{i32},{i32}-{i32}",
            a_start,
            a_end,
            b_start,
            b_end
        )
        .unwrap();
        let a = Pair {
            start: a_start,
            end: a_end,
        };
        let b = Pair {
            start: b_start,
            end: b_end,
        };
        if subset(&a, &b) {
            p1 += 1;
        }
        if overlap(&a, &b) {
            p2 += 1;
        }
    }

    println!("p1: {}", p1);
    println!("p2: {}", p2);
}

fn subset(a: &Pair, b: &Pair) -> bool {
    return (a.start >= b.start && a.end <= b.end) || (b.start >= a.start && b.end <= a.end);
}

fn overlap(a: &Pair, b: &Pair) -> bool {
    return subset(a, b)
        || (b.start <= a.start && a.start <= b.end)
        || (b.start <= a.end && a.end <= b.end);
}
