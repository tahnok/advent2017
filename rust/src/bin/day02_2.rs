use std::cmp;
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
        let val = each_pair(row.as_slice());
        acc + val
    })
}

fn each_pair(input: &[u32]) -> u32 {
    for (i, e1) in input.iter().enumerate() {
        for e2 in input.iter().skip(i + 1) {
            let min = cmp::min(e1,e2);
            let max = cmp::max(e1,e2);
            if max % min == 0 {
                return max / min
            }
        }
    }
    panic!("didn't find")
}
