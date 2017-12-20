extern crate regex;

use std::io;
use std::io::Read;

use regex::Regex;

fn main() {
    let mut input = String::new();
    let _ = io::stdin().read_to_string(&mut input);
    closest(input.trim());
}

fn closest(input: &str) -> () {
    let re = Regex::new(r"-?\d+").unwrap(); 
    let mut particles = Vec::new();
    for line in input.lines() {
        let mut pieces = Vec::new();
        for cap in re.captures_iter(line) {
            pieces.push(cap[0].parse::<isize>().unwrap());
        }
        particles.push((
            (pieces[0], pieces[1], pieces[2]),
            (pieces[3], pieces[4], pieces[5]),
            (pieces[6], pieces[7], pieces[8])));

    }
    loop {
        let mut min = <isize>::max_value();
        let mut min_index = 0;
        for (index, particle) in particles.iter_mut().enumerate() {
            let (mut position, mut velocity, acceleration) = *particle;

            velocity.0 += acceleration.0;
            velocity.1 += acceleration.1;
            velocity.2 += acceleration.2;

            position.0 += velocity.0;
            position.1 += velocity.1;
            position.2 += velocity.2;

            particle.0 = position;
            particle.1 = velocity;

            let this_min = position.0.abs() + position.1.abs() + position.2.abs();
            if this_min < min {
                min_index = index;
                min = this_min;
            }
        }
        println!("{}: {:?}", min_index, particles[min_index].0);
    }
}
