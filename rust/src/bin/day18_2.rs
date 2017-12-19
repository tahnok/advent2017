use std::collections::HashMap;
use std::collections::VecDeque;
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

fn step<'a>(
    pc: usize,
    instructions: &'a Vec<&'a str>,
    registers: &'a mut HashMap<&'a str, isize>,
    input: &mut VecDeque<isize>,
    output: &mut VecDeque<isize>
    ) -> (usize, bool, bool) {
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
                output.push_back(*registers.get(reg).unwrap_or(&0));
            },
            "rcv" => {
                match input.pop_front() {
                    Some(val) => {
                        registers.insert(reg, val);
                    },
                    None => {
                        return (pc, true, false);
                    
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
            return (0, false, true);
        } else if new_pc < 0 {
            return (0, false, true);
        } else {
            pc = new_pc as usize;
        }

    (0, false, false)
}

fn solve(input: &str) -> isize {
    let instructions: Vec<&str> = input.lines().collect();

    let mut pc = 0;
    let mut registers = HashMap::new();
    let mut input = VecDeque::new();
    let mut output = VecDeque::new();

    loop {
        let (new_pc, deadlocked, stopped) = step(pc, &instructions, &mut registers, &mut input, &mut output);
    }
    panic!("not found");
}

