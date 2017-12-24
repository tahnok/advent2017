use std::io;
use std::io::Read;


fn main() {
    let mut input = String::new();
    let _ = io::stdin().read_to_string(&mut input);
    println!("{}", strongest_bridge(input.trim()));
}

fn strongest_bridge(input: &str) -> usize {
    let mut edges = Vec::new();
    for raw in input.lines() {
        let mut parts = raw.split("/");
        let first = parts.next().unwrap().parse::<usize>().unwrap();
        let second = parts.next().unwrap().parse::<usize>().unwrap();
        edges.push((first, second));
    }

    //starting chains (vecs because order)
    //for each chain
    //make a new chain for each unused element that matches the last item in the list and put it in starting chain
    //remove current chain from starting chains
    //continue until starting chains is empty
    let mut done = Vec::new();

    let mut to_search = Vec::new(); //tuple of chain so far and unused elements
    for (index, edge) in edges.iter().enumerate() {
        if edge.0 == 0 {
            let mut unused = edges.clone();
            unused.remove(index);
            to_search.push((vec![*edge], unused));
        } else if edge.1 == 0 {
            let mut unused = edges.clone();
            unused.remove(index);
            to_search.push((vec![(edge.1, edge.0)], unused));
        }
    }

    while let Some((chain, unused)) = to_search.pop() {
        done.push(chain.clone());
        let end = chain.last().unwrap().1;
        for (index, edge) in unused.iter().enumerate() {
            if edge.0 == end {
                let mut new_unused = unused.clone();
                new_unused.remove(index);
                let mut new_chain = chain.clone();
                new_chain.push(*edge);
                to_search.push((new_chain, new_unused));
            } else if edge.1 == end {
                let mut new_unused = unused.clone();
                new_unused.remove(index);
                let mut new_chain = chain.clone();
                let swapped = (edge.1, edge.0);
                new_chain.push(swapped);
                to_search.push((new_chain, new_unused));
            }
        }
    }

    let mut max_strength = 0;
    let mut max_length = 0;
    for list in done.iter() {
        if list.len() > max_length {
            max_length = list.len();
            let mut thing = 0;
            for edge in list.iter() {
                thing += edge.0;
                thing += edge.1;
            }
            max_strength = thing;
        } else if list.len() == max_length {
            let mut thing = 0;
            for edge in list.iter() {
                thing += edge.0;
                thing += edge.1;
            }
            if thing > max_strength {
                max_strength = thing;
            }
        }
    }

    max_strength
}
