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
    let node_weight = *weights.get(node).unwrap();
    if !children_by_name.contains_key(node) { //leaf node
        return node_weight;
    }

    let children = children_by_name.get(node).unwrap();
    let mut child_recursive_weights = Vec::new();

    for child in children.iter() {
        child_recursive_weights.push(check_weight(child, &children_by_name, &weights));
    }
    let total = node_weight + child_recursive_weights.iter().fold(0, |sum, x| sum + x);

    let len = children.len();
    if len == 2 {
        if child_recursive_weights[0] == child_recursive_weights[1] {
            return total;
        } else {
            panic!("2 child is unbalanced for {}", node)
        }
    }

    for (index, child) in children.iter().enumerate() {
        let left_index = if index == 0 {
            len - 1
        } else {
            index - 1
        };
        let left = child_recursive_weights[left_index];
        let right_index = if index == (len - 1) {
            0
        } else {
            index + 1
        };
        let right = child_recursive_weights[right_index];
        let child_weight = child_recursive_weights[index];
        if left != child_weight && right != child_weight {
            let x = weights.get(child).unwrap();
            if child_weight > left {
                panic!("answer is {}", x - (child_weight - left));
            } else {
                panic!("answer is {}", x + (left - child_weight));
            }
        }
    }

    total
}
