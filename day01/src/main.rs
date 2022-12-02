use std::collections::BinaryHeap;
use std::fs;

fn main() {
    // Read input from file
    let text = fs::read_to_string("input.txt").unwrap();

    // Use a max heap to track the top elves
    let mut heap: BinaryHeap<i32> = BinaryHeap::new();

    // For each elf, tally up how many calories they are carrying
    for elf in text.split("\n\n") {
        heap.push(elf.lines().map(|c| c.parse::<i32>().unwrap()).sum())
    }

    println!("Max: {}", heap.peek().unwrap());
    println!(
        "Top 3: {}",
        (0..3).map(|_| heap.pop().unwrap()).sum::<i32>()
    );
}
