#![feature(slice_rotate)]
#![feature(inclusive_range,inclusive_range_syntax)]

use std::collections::HashSet;
use std::collections::HashMap;
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


    let mut region_map = HashMap::new();
    for row in 0..128 {
        for column in 0..128 {

            if !grid[row][column] {
                continue;
            }
            let mut neighbours = Vec::new();

            if column > 0 {
                let neighbour_column = column - 1;
                let neighbour_row = row;
                if grid[neighbour_row][neighbour_column] {
                    neighbours.push((neighbour_row, neighbour_column));
                }
            }
            if column < 127 {
                let neighbour_column = column + 1;
                let neighbour_row = row;
                if grid[neighbour_row][neighbour_column] {
                    neighbours.push((neighbour_row, neighbour_column));
                }
            }
            if row > 0 {
                let neighbour_column = column;
                let neighbour_row = row - 1;
                if grid[neighbour_row][neighbour_column] {
                    neighbours.push((neighbour_row, neighbour_column));
                }
            }
            if row < 127 {
                let neighbour_column = column;
                let neighbour_row = row + 1;
                if grid[neighbour_row][neighbour_column] {
                    neighbours.push((neighbour_row, neighbour_column));
                }
            }

            region_map.insert((row,column), neighbours);
        }
    }

    connected_regions(region_map)
}

fn connected_regions(edges: HashMap<(usize, usize), Vec<(usize, usize)>>) -> usize {
    let mut count = 0;

    let mut seen = HashSet::new();

    for starting in edges.keys() {
        let mut to_search = edges.get(&starting).unwrap().clone();
        to_search.push(*starting);
        let mut group = false;
        while let Some(edge) = to_search.pop() {
            if seen.contains(&edge) {
                continue;
            }
            group = true;

            seen.insert(edge);
            to_search.append(&mut edges.get(&edge).unwrap().clone());
        }
        if group {
            count += 1;
        }
    }
    count 
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
