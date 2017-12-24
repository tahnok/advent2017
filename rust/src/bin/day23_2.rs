use std::collections::HashMap;
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
    mults: usize
    ) -> (usize, bool, usize) {
        let assembly = instructions[pc];
        let mut step_size = 1;
        let mut new_mults = mults;

        let mut parts = assembly.split_whitespace();
        let instruction = parts.next().unwrap();
        let raw = parts.next().unwrap();
        let reg = raw.chars().next().unwrap();

        match instruction {
            "set" => {
                let val = val_or_reg(parts.next().unwrap(), &registers);
                registers.insert(reg, val);
            },
            "mul" => {
                let reg_val = *registers.get(&reg).unwrap_or(&0);
                let val = val_or_reg(parts.next().unwrap(), &registers);
                registers.insert(reg, reg_val * val);
                new_mults += 1;
            },
            "sub" => {
                let reg_val = *registers.get(&reg).unwrap_or(&0);
                let val = val_or_reg(parts.next().unwrap(), &registers);
                registers.insert(reg, reg_val - val);
            },
            "jnz" => {
                let reg_val = val_or_reg(raw, &registers);
                if reg_val != 0 {
                    let val = val_or_reg(parts.next().unwrap(), &registers);
                    step_size = val;
                }
            },
            _ => panic!("unknown line {}", assembly)
        }
        let new_pc = (pc as isize) + step_size;
        if new_pc >= (instructions.len() as isize) || new_pc < 0 {
            return (pc, true, new_mults);
        }

    (new_pc as usize, false, new_mults)
}

fn solve(input: &str) -> isize {
    let mut instructions: Vec<&str> = input.lines().collect();
    optimize(&mut instructions);

    let mut pc_0 = 0;
    let mut registers_0 = HashMap::new();
    registers_0.insert('a', 1);
    let mut mults_0 = 0;

    loop {
        let instruction = instructions[pc_0];
        let (new_pc_0, stopped_0, new_mults_0) = step(pc_0, &instructions, &mut registers_0, mults_0);
        mults_0 = new_mults_0;
        if stopped_0 {
            println!("stopped");
            break;
        }
        println!("{0:<2} -> {1:<2} {2:<14} | {3:?}", pc_0, new_pc_0, instruction, registers_0);
        pc_0 = new_pc_0;
    }

    *registers_0.get(&'h').unwrap()
}

fn optimize(instructions: &mut Vec<&str>) -> () {
        let mut parts = assembly.split_whitespace();
        let instruction = parts.next().unwrap();
        let raw = parts.next().unwrap();
        let reg = raw.chars().next().unwrap();
}

