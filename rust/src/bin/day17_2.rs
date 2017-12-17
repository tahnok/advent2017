use std::io;
use std::io::Read;


fn main() {
    let mut input = String::new();
    let _ = io::stdin().read_to_string(&mut input);
    println!("{}", next_value(&input));
}

fn next_value(input: &str) -> usize {
    let step: usize = input.trim().parse().unwrap();

    let mut zero_plus_loc = 1;
    let mut zero_plus_val = 1;
    let mut location = 1;
    for val in 2..50_000_000 {
        location = ((location + step) % val) + 1;
        if location == zero_plus_loc {
            zero_plus_val = val;
        } else if location < zero_plus_loc {
            zero_plus_loc += 1;
        }
    }

    zero_plus_val
}
