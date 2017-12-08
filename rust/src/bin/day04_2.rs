use std::collections::HashSet;
use std::io;
use std::io::Read;

fn main() {
    let mut input = String::new();
    let _ = io::stdin().read_to_string(&mut input);
    println!("{}", valid_passphrases(&input.trim()));
}

fn valid_passphrases(input: &str) -> u32 {
    let mut total = 0;
    for phrase in input.lines() {
        let mut valid = true;
        let mut seen = HashSet::new();
        for word in phrase.split_whitespace() {
            let mut chars: Vec<char> = word.chars().collect();
            chars.sort();
            let sorted: String = chars.into_iter().collect();
            if seen.contains(&sorted) {
                valid = false;
                break;
            } else {
                seen.insert(sorted);
            }
        }
        if valid {
            total = total + 1;
        }
    }
    total
}
