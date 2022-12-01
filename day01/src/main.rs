use std::collections::BinaryHeap;
use std::fs::File;
use std::io::Read;
use std::path::Path;

fn main() {
    // Read input from file
    let mut file = File::open(Path::new("input.txt")).unwrap();
    let mut text = String::new();
    file.read_to_string(&mut text).unwrap();

    // Use a max heap to track the top elves
    let mut heap: BinaryHeap<i32> = BinaryHeap::new();

    // For each elf, tally up how many calories they are carrying
    for elf in text.split("\n\n") {
        heap.push(elf.split('\n').map(|c| c.parse::<i32>().unwrap_or(0)).sum())
    }

    println!("Max: {}", heap.peek().unwrap());
    println!(
        "Top 3: {}",
        (0..3).fold(0, |sum, _| sum + heap.pop().unwrap())
    );
}
