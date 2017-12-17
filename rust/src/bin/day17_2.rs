use std::io;
use std::io::Read;


fn main() {
    let mut input = String::new();
    let _ = io::stdin().read_to_string(&mut input);
    println!("{}", next_value(&input));
}

fn next_value(input: &str) -> usize {
    let step: usize = input.trim().parse().unwrap();

    let mut start = 0;
    let mut buffer = vec![0];
    for val in 1..50_000_000 {
        start = (start + step) % (buffer.len()) + 1;
        buffer.insert(start, val);
        println!("{}", val);
    }

    for (index, &val) in buffer.iter().enumerate() {
        if val == 0 {
            return buffer[(index + val) % buffer.len()];
        }
    }
    panic!("index of 0 not found");
}
