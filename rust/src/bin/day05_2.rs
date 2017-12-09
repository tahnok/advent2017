use std::io;
use std::io::Read;

fn main() {
    let mut input = String::new();
    let _ = io::stdin().read_to_string(&mut input);
    println!("{}", steps_to_escape(input.trim()));
}

fn steps_to_escape(input: &str) -> u32 {
    let mut digits: Vec<i32> = input.lines().map(|x| x.parse().unwrap()).collect();
    let mut index = 0;
    let mut count = 0;
    loop {
        count = count + 1;

        let jmp = digits[index];
        let new_index = (index as i32 + jmp) as usize;
        if new_index < digits.len() {
            let new_jmp = if jmp < 3 {
                jmp + 1
            } else {
                jmp - 1
            };

            digits[index] = new_jmp;
            index = new_index;
        } else {
            break;
        }
    }
    count
}

