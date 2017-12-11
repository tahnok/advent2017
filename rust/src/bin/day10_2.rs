#![feature(slice_rotate)]

use std::io;
use std::io::Read;


fn main() {
    let mut input = String::new();
    let _ = io::stdin().read_to_string(&mut input);
    println!("{}", hash(input.trim()));
}

fn hash(input: &str) -> String {
    let mut lengths: Vec<u8> = input.bytes().collect();
    lengths.append(&mut vec![17, 31, 73, 47, 23]);

    let mut position = 0;
    let mut rope: Vec<usize> = (0..256).collect();
    let rope_length = rope.len();

    let mut skip_size = 0;

    for _round in 0..64 {
        for length_u8 in lengths.iter() {
            let length = *length_u8 as usize;
            let mut front: Vec<usize> = rope.iter().cycle().skip(position).take(length).map(|x| *x).collect();
            front.reverse();
            let mut back: Vec<usize> = rope.iter().cycle().skip(position + length).take(rope_length - length).map(|x| *x).collect();

            front.append(&mut back);
            let rotate = rope_length - position;
            front.rotate(rotate);
            rope = front;
            position = (position + length + skip_size) % rope_length;
            skip_size += 1;
        }
    }
    let dense_hash = rope.chunks(16).map(|chunk| chunk.iter().fold(0, |acc, &x| acc ^ x));
    dense_hash.map(|hex| format!("{:02x}", hex)).collect::<Vec<String>>().concat()
}
