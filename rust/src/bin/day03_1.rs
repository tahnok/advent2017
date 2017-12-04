use std::io;
use std::io::Read;

fn main() {
    let mut input = String::new();
    let _ = io::stdin().read_to_string(&mut input);
    let result = spiral_distance(input.trim().parse::<u32>().unwrap());
    println!("{}", result);
}

fn spiral_distance(number: u32) -> u32 {
   number 
}
