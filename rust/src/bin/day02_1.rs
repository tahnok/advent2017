use std::io;
use std::io::Read;

fn main() {
    let mut input = String::new();
    let _ = io::stdin().read_to_string(&mut input);
    println!("{}", spreadsheet_checksum(&input.trim()));
}

fn spreadsheet_checksum(input: &str) -> u32 {
    input.lines().fold(0, |acc, line| {
        let row = line
            .split_whitespace()
            .map(|n| n.parse::<u32>().unwrap())
            .collect::<Vec<u32>>();
        let min = row.iter().min().unwrap();
        let max = row.iter().max().unwrap();
        acc + (max - min)
    })
}
