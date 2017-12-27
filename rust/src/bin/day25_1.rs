extern crate itertools;

use std::collections::HashMap;
use std::io;
use std::io::Read;

use itertools::Itertools;

fn main() {
    let mut input = String::new();
    let _ = io::stdin().read_to_string(&mut input);
    println!("{}", checksum(&input.trim()));
}

#[derive(Debug)]
struct Change {
    value: bool,
    direction: bool,
    new_state: char
}

fn extract(raw_state: &mut itertools::Chunk<std::str::Lines>) -> Change {
    raw_state.next();
    let true_val = match raw_state.next().unwrap() {
        "    - Write the value 1." => true,
        "    - Write the value 0." => false,
        _ => panic!("unknown line")
    };
    let true_direction = match raw_state.next().unwrap() {
        "    - Move one slot to the right." => true,
        "    - Move one slot to the left." => false,
        _ => panic!("unknown line")
    };

    let raw_true_state = raw_state.next().unwrap();
    let true_state = raw_true_state.chars().nth(raw_true_state.len() - 2).unwrap();

    Change {
        value: true_val,
        direction: true_direction,
        new_state: true_state
    }
}

fn checksum(input: &str) -> usize {
    let mut lines = input.lines();

    let raw_start_state = lines.next().unwrap();
    let start_state = raw_start_state.chars().nth(raw_start_state.len() - 2).unwrap();

    let raw_steps = lines.next().unwrap();
    let steps: usize = raw_steps.split_whitespace().nth(5).unwrap().parse().unwrap();

    let mut states = HashMap::new();

    for mut raw_state in &lines.chunks(10) {
        let raw_state_name = raw_state.nth(1).unwrap();
        let state = raw_state_name.chars().nth(raw_state_name.len() - 2).unwrap();
        let false_state = extract(&mut raw_state);
        let true_state = extract(&mut raw_state);

        states.insert(state, (true_state, false_state));
    }

    let mut state = start_state;
    let mut tape = HashMap::new();
    let mut position = 0;

    for _ in 0..steps {
        let current_state = states.get(&state).unwrap();

        let current_value = *tape.get(&position).unwrap_or(&false);
        let to_apply = if current_value {
            &current_state.0
        } else {
            &current_state.1
        };

        tape.insert(position, to_apply.value);
        if to_apply.direction {
            position += 1;
        } else {
            position -= 1;
        }
        state = to_apply.new_state;
    }

    let mut count = 0;
    for (_, slot) in tape.iter() {
        if *slot {
            count += 1;
        }
    }
    
    count 
}
