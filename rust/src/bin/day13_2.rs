use std::io;
use std::io::Read;

fn main() {
    let mut input = String::new();
    let _ = io::stdin().read_to_string(&mut input);
    println!("{}", severity(&input.trim()));
}

fn severity(input: &str) -> usize {
    let lines = input.lines();
    let mut layers = Vec::with_capacity(100);

    for line in lines {
        let mut parts = line.split(": ");
        let layer: usize = parts.next().unwrap().parse().unwrap();
        let depth: usize = parts.next().unwrap().parse().unwrap();
        layers.push((layer, (2 * (depth - 1))));
    }

    let mut delay = 0;
    let iter = layers.iter();
    'outer: loop {
        for &(layer, depth) in iter.clone() {
            if (layer + delay) % depth == 0 {
                delay += 1;
                continue 'outer;
            }
        }
        break;
    }

    delay
}
