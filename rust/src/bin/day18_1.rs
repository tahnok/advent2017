use std::collections::HashMap;
use std::io;
use std::io::Read;

fn main() {
    let mut input = String::new();
    let _ = io::stdin().read_to_string(&mut input);
    println!("{}", solve(input.trim()));
}

fn val_or_reg(val: &str, register: &HashMap<&str, isize>) -> isize {
    let maybe_val = val.parse();
    match maybe_val {
        Ok(number) => number,
        Err(_) => *register.get(val).unwrap_or(&0)
    }
}

fn solve(input: &str) -> isize {
    let mut registers = HashMap::new();
    let mut last_sound = 0;
    let instructions: Vec<&str> = input.lines().collect();
    let mut pc = 0;

    loop {
        let assembly = instructions[pc];
        let mut step_size = 1;

        let mut parts = assembly.split_whitespace();
        let instruction = parts.next().unwrap();
        let reg = parts.next().unwrap();

        match instruction {
            "set" => {
                let val = val_or_reg(parts.next().unwrap(), &registers);
                registers.insert(reg, val);
            },
            "add" => {
                let reg_val = *registers.get(reg).unwrap_or(&0);
                let val = val_or_reg(parts.next().unwrap(), &registers);
                registers.insert(reg, reg_val + val);
            },
            "mul" => {
                let reg_val = *registers.get(reg).unwrap_or(&0);
                let val = val_or_reg(parts.next().unwrap(), &registers);
                registers.insert(reg, reg_val * val);
            },
            "mod" => {
                let reg_val = *registers.get(reg).unwrap_or(&0);
                let val = val_or_reg(parts.next().unwrap(), &registers);
                registers.insert(reg, reg_val % val);
            },
            "snd" => {
                last_sound = *registers.get(reg).unwrap_or(&0);
            },
            "rcv" => {
                let val = *registers.get(reg).unwrap_or(&0);
                if val != 0 {
                    return last_sound;
                }
            },
            "jgz" => {
                let reg_val = *registers.get(reg).unwrap_or(&0);
                if reg_val > 0 {
                    let val = val_or_reg(parts.next().unwrap(), &registers);
                    step_size = val;
                }
            },
            _ => panic!("unknown line {}", assembly)
        }
        let new_pc = (pc as isize) + step_size;
        if new_pc > (instructions.len() as isize) {
            break;
        } else if new_pc < 0 {
            break;
        } else {
            pc = new_pc as usize;
        }
    }
    panic!("not found");
}

