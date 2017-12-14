#![feature(slice_rotate)]
#![feature(inclusive_range,inclusive_range_syntax)]

use std::collections::HashMap;
use std::collections::HashSet;
use std::io;
use std::io::Read;


fn main() {
    let mut input = String::new();
    let _ = io::stdin().read_to_string(&mut input);
    println!("{}", regions(input.trim()));
}

fn regions(input: &str) -> usize {
    let mut grid = Vec::with_capacity(128);
    for row in 0..128 {
        let row_str = format!("{}-{}", input, row);
        let mut row = Vec::with_capacity(128);
        for byte in hash(&row_str) {
            for shift in (0..8).into_iter().rev() {
                if ((byte >> shift) & 1) == 1 {
                    row.push(true);
                } else {
                    row.push(false);
                }
            }
        }
        grid.push(row);
    }
    let mut regions = 0;
    let mut dup_regions = HashSet::new();
    let mut region_map: HashMap<(usize, usize), usize> = HashMap::new();
    for row in 0..128 {
        for column in 0..128 {
            let filled = grid[row][column];
            if !filled {
                continue;
            }

            let mut connected = false;
            let mut connected_region = 0;
            if row != 0 {
                let up = (row - 1, column);
                if region_map.contains_key(&up) {
                    let existing_region = *region_map.get(&up).unwrap();
                    region_map.insert((row, column), existing_region);
                    connected = true;
                    connected_region = existing_region;
                }
            }
            if column != 0 {
                let left = (row, column - 1);
                if region_map.contains_key(&left) {
                    let existing_region = *region_map.get(&left).unwrap();
                    if connected {
                        dup_regions.insert((existing_region, connected_region));
                        region_map.insert((row, column), connected_region);
                    } else {
                        region_map.insert((row, column), existing_region);
                    }
                    connected = true;
                }
            }

            if !connected {
                regions += 1;
                region_map.insert((row, column), regions);
            }

        }
    }
    for row in 0..8 {
        for column in 0..8 {
            match region_map.get(&(row,column)) {
                Some(region) => print!("{}", region),
                None => print!(".")
            }
        }
        println!();
    }
    println!("regions {}, dup_regions {}", regions, dup_regions.len());
    regions - dup_regions.len()
}

fn hash(input: &str) -> Vec<u8> {
    let mut lengths: Vec<u8> = input.bytes().collect();
    lengths.append(&mut vec![17, 31, 73, 47, 23]);

    let mut position = 0;
    let mut rope: Vec<u8> = (0..=255).collect();
    let rope_length = rope.len();

    let mut skip_size = 0;

    for _round in 0..64 {
        for length_u8 in lengths.iter() {
            let length = *length_u8 as usize;
            let mut front: Vec<u8> = rope.iter().cycle().skip(position).take(length).map(|x| *x).collect();
            front.reverse();
            let mut back: Vec<u8> = rope.iter().cycle().skip(position + length).take(rope_length - length).map(|x| *x).collect();

            front.append(&mut back);
            let rotate = rope_length - position;
            front.rotate(rotate);
            rope = front;
            position = (position + length + skip_size) % rope_length;
            skip_size += 1;
        }
    }
    let dense_hash = rope.chunks(16).map(|chunk| chunk.iter().fold(0, |acc, &x| acc ^ x));
    dense_hash.collect()
}
