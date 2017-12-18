use std::collections::HashMap;
use std::io;
use std::io::Read;

fn main() {
    let mut input = String::new();
    let _ = io::stdin().read_to_string(&mut input);
    println!("{}", solve(input.trim()));
}

fn solve(input: &str) -> usize {
    let mut registers = HashMap::new();
    let mut last_sound = 0;

    for assembly in input.lines() {
        let mut parts = assembly.split_whitespace();
        let instruction = parts.next().unwrap();
        match instruction {
            "set" => {
                let reg = parts.next().unwrap();
                let val: usize = parts.next().unwrap().parse().unwrap();
                registers.insert(reg, val);
            },
            "add" => {
            },
            "mul" => {
            },
            "mod" => {
            },
            "rcv" => {
            },
            "jgz" => {
            },
            "snd" => {
            },
            _ => panic!("unknown line {}", assembly)
        }
    }
    last_sound
}

