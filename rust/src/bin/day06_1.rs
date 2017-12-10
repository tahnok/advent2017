use std::collections::HashSet;
use std::io;
use std::io::Read;


fn main() {
    let mut input = String::new();
    let _ = io::stdin().read_to_string(&mut input);
    println!("{}", cycles(input.trim()));
}

fn cycles(input: &str) -> usize {
    let mut banks: Vec<usize> = input.split_whitespace().map(|x| x.parse().unwrap()).collect();
    let mut seen = HashSet::new();
    let mut count = 1;

    loop {
        let mut max = banks[0];
        let mut max_index = 0;
        for (index, &val) in banks.iter().enumerate() {
            if val > max {
                max = val;
                max_index = index;
            }
        }

        banks[max_index] = 0;

        while max > 0 {
            max_index = (max_index + 1) % banks.len();
            banks[max_index] += 1;
            max -= 1;
        }
        if seen.contains(&banks) {
            break;
        }
        seen.insert(banks.clone());
        count += 1;
    }

    count
}

