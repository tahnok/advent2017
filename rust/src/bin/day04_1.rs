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
        let mut seen = HashSet::new();
        let mut valid = true;
        for word in phrase.split_whitespace() {
            if seen.contains(word) {
                valid = false;
                break;
            } else {
                seen.insert(word);
            }
        }
        if valid {
            total = total + 1;
        }
    }
    total
}
