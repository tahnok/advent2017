#![feature(slice_rotate)]

use std::io;
use std::io::Read;


fn main() {
    let mut input = String::new();
    let _ = io::stdin().read_to_string(&mut input);
    println!("{}", dance(input.trim()));
}

fn dance(input: &str) -> String {
    let mut dancers: Vec<char> = "abcdefghijklmnop".chars().collect();
    //let mut dancers: Vec<char> = "abcde".chars().collect();
    let len = dancers.len();
    for step in input.split(",") {
        let mut chars = step.chars();
        let command = chars.next().unwrap();
        let rest = chars.as_str();
        match command {
            's' => {
                let shift: usize = rest.parse().unwrap();
                dancers.rotate(len - shift);
            },
            'x' => {
                let mut halves = rest.split("/");
                let first_index: usize = halves.next().unwrap().parse().unwrap();
                let second_index: usize = halves.next().unwrap().parse().unwrap();
                let first = dancers[first_index];
                let second = dancers[second_index];
                dancers[first_index] = second;
                dancers[second_index] = first;
            },
            'p' => {
                let mut bits = rest.chars();
                let first = bits.next().unwrap();
                bits.next();
                let second = bits.next().unwrap();

                let mut first_index = 0;
                let mut second_index = 0;
                for (index, &dancer) in dancers.iter().enumerate() {
                    if dancer == first {
                        first_index = index;
                    }
                    if dancer == second {
                        second_index = index;
                    }
                }
                dancers[first_index] = second;
                dancers[second_index] = first;
            },
            _ => panic!("unexpected char")
        }
    }

    dancers.iter().collect()
}

