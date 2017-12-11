use std::io;
use std::io::Read;


fn main() {
    let mut input = String::new();
    let _ = io::stdin().read_to_string(&mut input);
    println!("{}", hash(input.trim()));
}

fn hash(input: &str) -> usize {
    let lengths: Vec<usize> = input.split(",").map(|x| x.parse().unwrap()).collect();

    let mut position = 0;
    let mut rope: Vec<usize> = (0..5).collect();
    println!("{:?}", rope);
    let mut skip_size = 1;

    for length in lengths {
        let mut front: Vec<usize> = rope.iter().cycle().skip(position).take(length).map(|x| *x).collect();
        front.reverse();
        println!("front: {:?}", front);
        let mut back: Vec<usize> = rope.iter().cycle().skip(position + length).take(rope.len() - length).map(|x| *x).collect();
        println!("back: {:?}", back);
        front.append(&mut back);
        rope = front;
        println!("{:?}", rope);
        position = length + skip_size;

        skip_size += 1;
    }
    rope[0] * rope[1] 
}
