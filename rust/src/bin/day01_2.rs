use std::io;

fn main() {
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_) => {
            println!("{}", count_matching_pairs(&input.trim()));
        }
        Err(error) => println!("error: {}", error),
    }
}

fn count_matching_pairs(input: &str) -> u32 {
    let first = input.trim().chars();
    let shifted = first.clone().cycle().skip((input.len()) / 2);

    first.zip(shifted).fold(0, |acc, (first,second)| {
        if first == second {
        acc + first.to_digit(10).unwrap()
        } else {
            acc
        }
    })
}
