use std::collections::HashSet;
use std::fs;

fn char_to_priority(c: char) -> u32 {
    if c.is_ascii_lowercase() {
        return (c as u32) - ('a' as u32) + 1;
    } else if c.is_ascii_uppercase() {
        return (c as u32) - ('A' as u32) + 27;
    }
    panic!("Invalid char");
}

fn part1(text: &str) -> u32 {
    let mut total: u32 = 0;

    for pack in text.lines() {
        let pack: Vec<char> = pack.chars().collect();
        let first_half: HashSet<&char> = HashSet::from_iter(pack[..(pack.len() / 2)].iter());
        let second_half: HashSet<&char> = HashSet::from_iter(pack[(pack.len() / 2)..].iter());

        total += char_to_priority(**first_half.intersection(&second_half).nth(0).unwrap());
    }

    return total;
}

fn intersect3(s1: &str, s2: &str, s3: &str) -> char {
    return *HashSet::from_iter(
        HashSet::<char>::from_iter(s1.chars())
            .intersection(&HashSet::<char>::from_iter(s2.chars()))
            .cloned(),
    )
    .intersection(&HashSet::<char>::from_iter(s3.chars()))
    .nth(0)
    .unwrap();
}

fn part2(text: &str) -> u32 {
    let lines: Vec<&str> = text.lines().collect();

    return lines
        .chunks_exact(3)
        .map(|chunk| char_to_priority(intersect3(chunk[0], chunk[1], chunk[2])))
        .sum();
}

fn main() {
    // Read input from file
    let text = fs::read_to_string("input.txt").unwrap();

    println!("Part 1: {}", part1(&text));
    println!("Part 2: {}", part2(&text));
}
