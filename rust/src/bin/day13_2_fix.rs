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
    let mut layer_pos_orig = HashMap::new();
    let mut layer_dir_orig = HashMap::new();

    let mut max = 0;
    for line in input.lines() {
        let mut parts = line.split(": ");
        let layer: usize = parts.next().unwrap().parse().unwrap();
        let depth: usize = parts.next().unwrap().parse().unwrap();
        layers.insert(layer,depth);
        layer_pos_orig.insert(layer, 0);
        layer_dir_orig.insert(layer, DOWN);
        max = layer;
    }

    let mut delay = 0;
    'outer: loop {
        delay += 1;
        let mut layer_pos = layer_pos_orig.clone();
        let mut layer_dir = layer_dir_orig.clone();
        for (layer, &depth) in layers.iter() {
            let offset = delay % (depth * 2);
            if offset > depth {
                let pos = depth - (offset - depth);
                let dir = UP;
                layer_pos.insert(*layer, pos);
                layer_dir.insert(*layer, dir);
            } else {
                let pos = offset;
                let dir = DOWN;
                layer_pos.insert(*layer, pos);
                layer_dir.insert(*layer, dir);
            }
        }

        for step in 0..(max + 1) {
            //println!("looking at {}", step);
            //check if caught
            if layers.contains_key(&step) {
                if *layer_pos.get(&step).unwrap() == 0 {
                    continue 'outer;
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
        break;
        //println!("{:?}", caught);
    }
    delay
}
