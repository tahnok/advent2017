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

    let mut max = 0;
    for line in input.lines() {
        let mut parts = line.split(": ");
        let layer: usize = parts.next().unwrap().parse().unwrap();
        let depth: usize = parts.next().unwrap().parse().unwrap();
        layers.insert(layer, (depth, 0, DOWN));
        max = layer;
    }

    let mut states = HashMap::new();

    let mut delay = 31168;
    'outer: loop {
        delay += 1;
        println!("delay {}", delay);
        let mut layers_clone = layers.clone();

        if states.contains_key(&delay) {
            let temp: &HashMap<usize, (usize, usize, bool)> = states.get(&delay).unwrap();
            layers_clone = temp.clone();
        } else {
            'step: for state in 0..delay {
                if states.contains_key(&state) {
                    let temp: &HashMap<usize, (usize, usize, bool)> = states.get(&state).unwrap();
                    layers_clone = temp.clone();
                    continue 'step;
                }

                for layer in layers.keys() {
                    let &(depth, mut pos, mut dir) = layers_clone.get(layer).unwrap();
                    if dir == UP {
                        pos -= 1;
                    } else {
                        pos += 1;
                    }

                    if pos == 0 {
                        dir = DOWN;
                    } else if pos == (depth - 1) {
                        dir = UP;
                    }

                    //println!("{} is at {} moving {}", layer, pos, dir);
                    layers_clone.insert(*layer, (depth, pos, dir));
                }
                states.insert(state, layers_clone.clone());
            }
        }

        for step in 0..(max + 1) {
            //println!("looking at {}", step);
            //check if caught
            if layers_clone.contains_key(&step) {
                if layers_clone.get(&step).unwrap().1 == 0 {
                    continue 'outer;
                }
            }
            for layer in layers.keys() {
                let &(depth, mut pos, mut dir) = layers_clone.get(layer).unwrap();
                if dir == UP {
                    pos -= 1;
                } else {
                    pos += 1;
                }

                if pos == 0 {
                    dir = DOWN;
                } else if pos == (depth - 1) {
                    dir = UP;
                }

                //println!("{} is at {} moving {}", layer, pos, dir);
                layers_clone.insert(*layer, (depth, pos, dir));
            }
            states.insert(step + delay, layers_clone.clone());
        }
        break;
        //println!("{:?}", caught);
    }
    delay
}
