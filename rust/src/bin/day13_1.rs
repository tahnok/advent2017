use std::collections::HashMap;
use std::io;
use std::io::Read;

fn main() {
    let mut input = String::new();
    let _ = io::stdin().read_to_string(&mut input);
    println!("{}", severity(&input.trim()));
}

const DOWN: bool = false;
const UP: bool = true;

fn severity(input: &str) -> usize {
    let mut layers = HashMap::new();
    let mut layer_pos = HashMap::new();
    let mut layer_dir = HashMap::new();

    let mut max = 0;
    for line in input.lines() {
        let mut parts = line.split(": ");
        let layer: usize = parts.next().unwrap().parse().unwrap();
        let depth: usize = parts.next().unwrap().parse().unwrap();
        layers.insert(layer,depth);
        layer_pos.insert(layer, 0);
        layer_dir.insert(layer, DOWN);
        max = layer;
    }

    let mut caught = Vec::new();
    for step in 0..(max + 1) {
        //println!("looking at {}", step);
        //check if caught
        if layers.contains_key(&step) {
            if *layer_pos.get(&step).unwrap() == 0 {
                caught.push(step);
            }
        }
        for (layer, depth) in layers.iter() {
            let mut pos = *layer_pos.get(&layer).unwrap();
            let mut dir = *layer_dir.get(&layer).unwrap();
            if dir == UP {
                pos -= 1;
            } else {
                pos += 1;
            }

            if pos == 0 {
                dir = DOWN;
            } else if pos == (*depth - 1) {
                dir = UP;
            }

            //println!("{} is at {} moving {}", layer, pos, dir);
            layer_pos.insert(*layer, pos);
            layer_dir.insert(*layer, dir);
        }
        //println!("============");
    }
    //println!("{:?}", caught);
    let mut cost = 0;
    for layer in caught {
        cost += layer * layers.get(&layer).unwrap();
    }
    cost
}
