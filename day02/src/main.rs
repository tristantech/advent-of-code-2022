use std::fs;

const POINTS_LOSS: i32 = 0;
const POINTS_TIE: i32 = 3;
const POINTS_WIN: i32 = 6;

fn shape_to_int(s: &str) -> i32 {
    match s {
        "A" | "X" => 0, // Rock
        "B" | "Y" => 1, // Paper
        "C" | "Z" => 2, // Scissors
        _ => panic!("Invalid shape"),
    }
}

fn match_score(theirs: i32, ours: i32) -> i32 {
    if theirs == ours {
        return POINTS_TIE; // Tie
    } else if (theirs + 1).rem_euclid(3) == ours {
        return POINTS_WIN; // Win
    }
    return POINTS_LOSS; // Loss
}

fn main() {
    // Read input from file
    let text = fs::read_to_string("input.txt").unwrap();

    let mut sum_part1: i32 = 0;
    let mut sum_part2: i32 = 0;

    for l in text.lines() {
        let parts: Vec<&str> = l.split(' ').collect();
        let their_shape = shape_to_int(parts[0]);

        // Part 1
        let our_shape = shape_to_int(parts[1]);
        sum_part1 += (our_shape + 1) + match_score(their_shape, our_shape);

        // Part 2
        match parts[1] {
            "X" => sum_part2 += POINTS_LOSS + (their_shape - 1).rem_euclid(3) + 1, // Lose
            "Y" => sum_part2 += POINTS_TIE + their_shape + 1,                      // Tie
            "Z" => sum_part2 += POINTS_WIN + (their_shape + 1).rem_euclid(3) + 1,  // Win
            _ => panic!("Invalid token"),
        }
    }

    println!("Score (Part 1) {}", sum_part1);
    println!("Score (Part 2) {}", sum_part2);
}
