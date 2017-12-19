use std::collections::HashMap;
use std::collections::VecDeque;
use std::io;
use std::io::Read;

fn main() {
    let mut input = String::new();
    let _ = io::stdin().read_to_string(&mut input);
    println!("{}", solve(input.trim()));
}

fn val_or_reg(val: &str, register: &HashMap<char, isize>) -> isize {
    let maybe_val = val.parse();
    match maybe_val {
        Ok(number) => number,
        Err(_) => *register.get(&val.chars().next().unwrap()).unwrap_or(&0)
    }
}

fn step(
    pc: usize,
    instructions: &Vec<&str>,
    registers: &mut HashMap<char, isize>,
    input: &mut VecDeque<isize>,
    output: &mut VecDeque<isize>,
    sends: usize
    ) -> (usize, bool, bool, usize) {
        let assembly = instructions[pc];
        let mut step_size = 1;
        let mut new_sends = sends;

        let mut parts = assembly.split_whitespace();
        let instruction = parts.next().unwrap();
        let raw = parts.next().unwrap();
        let reg = raw.chars().next().unwrap();

        match instruction {
            "set" => {
                let val = val_or_reg(parts.next().unwrap(), &registers);
                registers.insert(reg, val);
            },
            "add" => {
                let reg_val = *registers.get(&reg).unwrap_or(&0);
                let val = val_or_reg(parts.next().unwrap(), &registers);
                registers.insert(reg, reg_val + val);
            },
            "mul" => {
                let reg_val = *registers.get(&reg).unwrap_or(&0);
                let val = val_or_reg(parts.next().unwrap(), &registers);
                registers.insert(reg, reg_val * val);
            },
            "mod" => {
                let reg_val = *registers.get(&reg).unwrap_or(&0);
                let val = val_or_reg(parts.next().unwrap(), &registers);
                registers.insert(reg, reg_val % val);
            },
            "snd" => {
                let val = val_or_reg(raw, &registers);
                output.push_back(val);
                new_sends += 1;
            },
            "rcv" => {
                match input.pop_front() {
                    Some(val) => {
                        registers.insert(reg, val);
                    },
                    None => {
                        return (pc, true, false, new_sends);
                    }
                }
            },
            "jgz" => {
                let reg_val = *registers.get(&reg).unwrap_or(&0);
                if reg_val > 0 {
                    let val = val_or_reg(parts.next().unwrap(), &registers);
                    step_size = val;
                }
            },
            _ => panic!("unknown line {}", assembly)
        }
        let new_pc = (pc as isize) + step_size;
        if new_pc > (instructions.len() as isize) {
            return (0, false, true, new_sends);
        } else if new_pc < 0 {
            return (0, false, true, new_sends);
        }

    (new_pc as usize, false, false, new_sends)
}

fn solve(input: &str) -> usize {
    let instructions: Vec<&str> = input.lines().collect();

    let mut pc_0 = 0;
    let mut registers_0 = HashMap::new();
    registers_0.insert('p', 0);
    let mut channel_0 = VecDeque::new();
    let mut sends_0 = 0;

    let mut pc_1 = 0;
    let mut registers_1 = HashMap::new();
    registers_1.insert('p', 1);
    let mut channel_1 = VecDeque::new();
    let mut sends_1 = 0;

    loop {
        let (new_pc_0, deadlocked_0, stopped_0, new_sends_0) = step(pc_0, &instructions, &mut registers_0, &mut channel_1, &mut channel_0, sends_0);
        let (new_pc_1, deadlocked_1, stopped_1, new_sends_1) = step(pc_1, &instructions, &mut registers_1, &mut channel_0, &mut channel_1, sends_1);
        sends_0 = new_sends_0;
        sends_1 = new_sends_1;
        if stopped_0 || stopped_1 {
            panic!("stopped");
        }
        if deadlocked_0 && deadlocked_1 {
            println!("deadlocked");
            break;
        }
        pc_0 = new_pc_0;
        pc_1 = new_pc_1;
        println!("{} {}", pc_0, pc_1);
    }
    sends_1
}

