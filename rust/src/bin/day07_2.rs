extern crate regex;

use std::collections::HashMap;
use std::collections::HashSet;
use std::io;
use std::io::Read;

use regex::Regex;

fn main() {
    let mut input = String::new();
    let _ = io::stdin().read_to_string(&mut input);
    println!("{}", bottom_tower(&input));
}

fn bottom_tower(input: &str) -> String {
    let re = Regex::new(r"(\w+) \((\d+)\)(?: -> (.+))?").unwrap();
    let mut children_by_name = HashMap::new();
    let mut weights = HashMap::new();
    let mut leaves = HashSet::new();
    for line in input.lines() {
        let mut matches = re.captures(line).unwrap();
        let name = matches.get(1).map_or("", |m| m.as_str());
        let size = matches.get(2).map_or("", |m| m.as_str()).parse::<usize>().unwrap();
        weights.insert(name, size);

        let extras: Vec<&str> = matches.get(3).map_or("", |m| m.as_str()).split(", ").collect();
        if extras.len() != 1 {
            children_by_name.insert(name, extras);
        } else {
            leaves.insert(name);
        }
    }
    panic!("not found");
}
