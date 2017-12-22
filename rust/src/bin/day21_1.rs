use std::collections::HashMap;
use std::io;
use std::io::Read;


fn main() {
    let mut input = String::new();
    let _ = io::stdin().read_to_string(&mut input);
    println!("{}", fractalize(input.trim()));
}

fn line_to_matrix(line: &str) -> Matrix {
    // ../.#
    let mut matrix = Vec::new();
    for raw_row in line.split("/") {
        let mut row = Vec::new();
        for x in raw_row.chars() {
            match x {
                '.' => row.push(false),
                '#' => row.push(true),
                _ => panic!("unexpected {} in {}", x, line)
            }
        }
        matrix.push(row);
    }
    matrix
}

fn print(matrix: &Matrix) -> () {
    for row in matrix.iter() {
        for column in row.iter() {
            if *column {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
}

fn count_on(matrix: &Matrix) -> usize {
    let mut count = 0;
    for row in matrix.iter() {
        for column in row.iter() {
            if *column {
                count += 1;
            }
        }
    }
    count
}

type Matrix = Vec<Vec<bool>>;

fn step(grid: &mut Matrix, transforms: &HashMap<Matrix,Matrix>) -> () {
    let slice_size = if grid[0].len() % 2 == 0 {
        2
    } else {
        3
    };
    let slices = grid[0].len() / slice_size;
    for row in 0..slices {
        let row_index = row * slice_size;
        for column in 0..slices {
            let column_index = column * slice_size;
            println!("{} {}", column_index, row_index);
            for item in grid[row_index].iter().skip(column_index).take(slice_size) {
                if *item {
                    print!("#");
                } else {
                    print!(".");
                }
                println!("");
            }
        }
    }

}

fn fractalize(input: &str) -> usize {
    //let mut grid: Matrix = vec![
        //vec![false, true, false],
        //vec![false, false, true],
        //vec![true, true, true]
    //];
    let mut grid: Matrix = vec![
        vec![true, false, true, false],
        vec![true, false, false, true],
        vec![true, true, true, true],
        vec![true, true, true, false]
    ];

    let mut transforms = HashMap::new();
    for line in input.lines() {
        //parsing ../.# => ##./#../...
        let mut parts = line.split(" => ");
        let from = line_to_matrix(parts.next().unwrap());
        let to = line_to_matrix(parts.next().unwrap());

        transforms.insert(from, to);
    }

    for _ in 0..1 {
        step(&mut grid, &transforms);
    }

    count_on(&grid)
}
