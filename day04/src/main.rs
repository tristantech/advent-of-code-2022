use scanf::sscanf;
use std::fs;

fn main() {
    // Read input from file
    let text = fs::read_to_string("input.txt").unwrap();
    let mut supersets = 0;
    let mut overlaps = 0;

    for line in text.lines() {
        let (mut a1, mut a2, mut b1, mut b2) = (0, 0, 0, 0);
        sscanf!(line, "{}-{},{}-{}", a1, a2, b1, b2).expect("Malformed");

        if (a1 >= b1 && a2 <= b2) || (b1 >= a1 && b2 <= a2) {
            supersets += 1;
        }

        if (a2 >= b1 && a1 < b2) || (b2 >= a1 && b1 < a2) {
            overlaps += 1;
        }
    }

    println!("Part 1: {}", supersets);
    println!("Part 2: {}", overlaps);
}
