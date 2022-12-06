use std::collections::HashSet;
use std::fs;

fn count_unique_chars(window: &[char]) -> usize {
    let hs: HashSet<char> = HashSet::from_iter(window.into_iter().cloned());
    return hs.len();
}

fn find_marker(text: &str, num_unqiue: usize) -> usize {
    for (i, chars) in text
        .chars()
        .collect::<Vec<char>>()
        .windows(num_unqiue)
        .enumerate()
    {
        if count_unique_chars(chars) == num_unqiue {
            return i + num_unqiue;
        }
    }
    panic!("Not found!");
}

fn main() {
    let text = fs::read_to_string("input.txt").unwrap();

    println!("Part 1: SOP at position {}", find_marker(&text, 4));
    println!("Part 2: SOM at position {}", find_marker(&text, 14));
}
