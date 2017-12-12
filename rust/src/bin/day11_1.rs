use std::io;
use std::io::Read;

fn main() {
    let mut input = String::new();
    let _ = io::stdin().read_to_string(&mut input);
    println!("{}", steps(input.trim()));
}

fn steps(input: &str) -> isize {
    //https://www.redblobgames.com/grids/hexagons/
    let mut x: isize = 0;
    let mut y: isize = 0;
    let mut z: isize = 0;
    for dir in input.split(",") {
        match dir {
            "n" => {
                y += 1;
                z -= 1;
            },
            "s" => {
                y -= 1;
                z += 1;
            },
            "ne" => {
                x += 1;
                z -= 1;
            },
            "nw" => {
                x -= 1;
                y += 1;
            },
            "sw" => {
                x -= 1;
                z += 1;
            },
            "se" => {
                x += 1;
                y -= 1;
            },
            _ => panic!("unexpected direction")
        }
        //println!("moved {} to {},{},{}",dir, x,y,z);
    }
    (x.abs() + y.abs() + z.abs()) / 2
}

