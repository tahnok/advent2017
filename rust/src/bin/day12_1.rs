use std::collections::HashMap;
use std::collections::HashSet;
use std::io;
use std::io::Read;

fn main() {
    let mut input = String::new();
    let _ = io::stdin().read_to_string(&mut input);
    println!("{}", steps_to_escape(input.trim()));
}

fn steps_to_escape(input: &str) -> usize {
    let mut edges = HashMap::new();

    for line in input.lines() {
        let mut parts = line.split(" <-> ");
        let item: usize = parts.next().unwrap().parse().unwrap();
        let connections: Vec<usize> = parts.next().unwrap().split(", ").map(|x| x.parse::<usize>().unwrap()).collect();
        edges.insert(item, connections);
    }

    let starting = 0;

    let mut count = 0;
    let mut to_search = edges.get(&starting).unwrap().clone();
    let mut seen = HashSet::new();

    while let Some(edge) = to_search.pop() {
        if seen.contains(&edge) {
            continue;
        }

        count += 1;
        seen.insert(edge);
        to_search.append(&mut edges.get(&edge).unwrap().clone());
    }
    count 
}

