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

    let root = "hlhomy";
    check_weight(&root, &children_by_name, &weights);

    panic!("not found");
}

fn check_weight(node: &str, children_by_name: &HashMap<&str, Vec<&str>>, weights: &HashMap<&str, usize>) -> usize {
    let weight = *weights.get(node).unwrap();
    if !children_by_name.contains_key(node) {
        return weight;
    }
    let children = children_by_name.get(node).unwrap();
    

    let left = check_weight(children[0], children_by_name, weights);
    let middle = check_weight(children[1], children_by_name, weights);
    let right = check_weight(children[2], children_by_name, weights);

    if left != middle && middle == right {
        let weight = weights.get(children[0]).unwrap();
        if left > middle {
            panic!("{} should be {}", weight, (weight - (left - middle)));
        } else {
            panic!("{} should be {}", weight, (weight + (middle - left)));
        }
    }

    if left == middle && middle != right {
        let weight = weights.get(children[2]).unwrap();
        if right > middle {
            panic!("{} should be {}", weight, (weight - (right - middle)));
        } else {
            panic!("{} should be {}", weight, (weight + (middle - right)));
        }
    }

    if left != middle && left == right {
        let weight = weights.get(children[1]).unwrap();
        if middle > right {
            panic!("{} should be {}", weight, (weight - (middle - right)));
        } else {
            panic!("{} should be {}", weight, (weight + (right - middle)));
        }
    }

    weight + left + middle + right
}
