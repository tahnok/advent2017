extern crate regex;

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
    let mut children = HashSet::new();
    let mut names = Vec::new();
    for line in input.lines() {
        let mut matches = re.captures(line).unwrap();
        let name = matches.get(1).map_or("", |m| m.as_str());
        names.push(name);
        //let size = matches.get(2).map_or("", |m| m.as_str());
        //let extras: Vec<&str> = matches.get(3).map_or("", |m| m.as_str()).split(", ").collect();
        for child in matches.get(3).map_or("", |m| m.as_str()).split(", ") {
            children.insert(child);
        }
    }
    for name in names.iter() {
        if !children.contains(name) {
            return String::from(*name);
        }
    }
    panic!("not found");
}
