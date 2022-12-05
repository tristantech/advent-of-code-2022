use scanf::sscanf;
use std::fs;

fn execute_crane_commands(
    mut stacks: Vec<Vec<char>>,
    instructions: std::str::Lines,
    reverse: bool,
    debug: bool,
) -> String {
    for inst in instructions {
        let (mut crate_count, mut from_stack, mut to_stack) = (0, 0, 0);
        sscanf!(
            inst,
            "move {} from {} to {}",
            crate_count,
            from_stack,
            to_stack
        )
        .expect("Malformed line");

        let mut crates_to_move = Vec::<char>::new();
        for _i in 0..crate_count {
            crates_to_move.push(stacks[from_stack - 1].pop().unwrap());
        }
        if reverse {
            crates_to_move.reverse();
        }
        for c in crates_to_move {
            stacks[to_stack - 1].push(c);
        }

        if debug {
            println!(
                "Instruction: {} - {} {} {}",
                inst, crate_count, from_stack, to_stack
            );
            for s in &stacks {
                println!(" > {:?}", s);
            }
        }
    }

    return stacks.iter().map(|s| s.last().unwrap()).collect::<String>();
}

fn main() {
    let text = fs::read_to_string("input.txt").unwrap();
    let portions: Vec<&str> = text.split("\n\n").collect();

    // Step 1: Decode the containter stack state
    let lines: Vec<&str> = portions[0].lines().rev().collect();
    let num_stacks: usize = lines[0]
        .split_whitespace()
        .last()
        .unwrap()
        .parse::<usize>()
        .unwrap();

    println!("Number of stacks: {}", num_stacks);

    let mut stacks = vec![Vec::<char>::new(); num_stacks];

    // Go through the remaining lines (we are reading in reverse order,
    // so we are inserting crates from bottom to top.)
    for l in &lines[1..] {
        let mut i = 0;
        for c in l.chars().collect::<Vec<char>>().chunks(4) {
            if c[1] != ' ' {
                stacks[i].push(c[1]);
            }
            i += 1;
        }
    }

    println!("Initial conditions:");
    for s in &stacks {
        println!(" > {:?}", s);
    }

    println!(
        "Part 1: {}",
        execute_crane_commands(stacks.to_vec(), portions[1].lines(), false, false)
    );
    println!(
        "Part 2: {}",
        execute_crane_commands(stacks.to_vec(), portions[1].lines(), true, false)
    );
}
