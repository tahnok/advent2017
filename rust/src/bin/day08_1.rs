use std::collections::HashMap;
use std::io;
use std::io::Read;


fn main() {
    let mut input = String::new();
    let _ = io::stdin().read_to_string(&mut input);
    println!("{}", max_register(input.trim()));
}

fn max_register(input: &str) -> isize {
    let mut registers:HashMap<&str, isize> = HashMap::new();
    for line in input.lines() {
        let mut bits = line.split_whitespace();

        let reg = bits.next().unwrap();
        let dir = bits.next().unwrap();
        let val = bits.next().unwrap().parse::<isize>().unwrap();
        bits.next(); //skip
        let dest = bits.next().unwrap();
        let op = bits.next().unwrap();
        let operand = bits.next().unwrap().parse::<isize>().unwrap();

        let dest_val = *registers.get(dest).unwrap_or(&0);

        let success = match op {
            "<" => dest_val < operand,
            ">" => dest_val > operand,
            ">=" => dest_val >= operand,
            "<=" => dest_val <= operand,
            "!=" => dest_val != operand,
            "==" => dest_val == operand,
            _ => panic!("unexpected op {}", op)
        };

        if success {
            let existing_val = *registers.get(reg).unwrap_or(&0);
            let new_val = match dir {
                "inc" => existing_val + val,
                "dec" => existing_val - val,
                _ => panic!("unexpected dir {}", dir)
            };
            registers.insert(reg, new_val);
        }

    }
    *registers.values().max().unwrap()
}
