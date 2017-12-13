use std::collections::HashMap;
use std::io;
use std::io::Read;

fn main() {
    let mut input = String::new();
    let _ = io::stdin().read_to_string(&mut input);
    println!("{}", severity(&input.trim()));
}

fn severity(input: &str) -> usize {
    let mut layers = HashMap::new();

    for line in input.lines() {
        let mut parts = line.split(": ");
        let layer: usize = parts.next().unwrap().parse().unwrap();
        let depth: usize = parts.next().unwrap().parse().unwrap();
        layers.insert(layer,depth);
    }

    let mut delay = 0;
    'outer: loop {
        for (layer, depth) in layers.iter() {
            if (layer + delay) % (2 * (depth - 1)) == 0 {
                delay += 1;
                continue 'outer;
            }
        }
        break;
    }

    delay
}
