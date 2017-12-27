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

fn step(grid: &Matrix, transforms: &HashMap<Matrix,Matrix>) -> Matrix {

    if grid.len() % 2 == 0 {
        let chunks = split2(&grid);
        let new_parts = find_rotations(&chunks, transforms);
        return join3(new_parts);
    } else {
        let chunks = split3(&grid);
        let new_parts = find_rotations(&chunks, transforms);
        return join4(new_parts);
    }
}

// 1 3x3 => 1 4x4
// 4 2x2 => 4 3x3 => 1 6x6
// 9 2x2 => 9 3x3 => 1 9x9
// 9 3x3 => 9 4x4 => 1 12x12
// 36 2x2 => 36 3x3 => 1 18x18
// 81 2x2 => 81 3x3 => 1 27x27

// number of squares is N*N
// size of new square is N*matrix size
// sqrt(N) times take sqrt(N) matrices and join them into one big row
fn join4(parts: Vec<Matrix>) -> Matrix {
    let mut new_matrix = Vec::new();
    let size = (parts.len() as f64).sqrt() as usize;
    for row in parts.chunks(size) {
        let mut one = Vec::new();
        let mut two = Vec::new();
        let mut three = Vec::new();
        let mut four = Vec::new();
        for grid in row.iter() {
            one.push(grid[0][0]);
            one.push(grid[0][1]);
            one.push(grid[0][2]);
            one.push(grid[0][3]);

            two.push(grid[1][0]);
            two.push(grid[1][1]);
            two.push(grid[1][2]);
            two.push(grid[1][3]);

            three.push(grid[2][0]);
            three.push(grid[2][1]);
            three.push(grid[2][2]);
            three.push(grid[2][3]);

            four.push(grid[3][0]);
            four.push(grid[3][1]);
            four.push(grid[3][2]);
            four.push(grid[3][3]);
        }
        new_matrix.push(one);
        new_matrix.push(two);
        new_matrix.push(three);
        new_matrix.push(four);
    }
    new_matrix
}

fn join3(parts: Vec<Matrix>) -> Matrix {
    let mut new_matrix = Vec::new();
    let size = (parts.len() as f64).sqrt() as usize;
    for row in parts.chunks(size) {
        let mut up = Vec::new();
        let mut middle = Vec::new();
        let mut down = Vec::new();
        for grid in row.iter() {
            up.push(grid[0][0]);
            up.push(grid[0][1]);
            up.push(grid[0][2]);
            middle.push(grid[1][0]);
            middle.push(grid[1][1]);
            middle.push(grid[1][2]);
            down.push(grid[2][0]);
            down.push(grid[2][1]);
            down.push(grid[2][2]);
        }
        new_matrix.push(up);
        new_matrix.push(middle);
        new_matrix.push(down);
    }
    new_matrix
}


fn find_rotations(chunks: &Vec<Matrix>, transforms: &HashMap<Matrix, Matrix>) -> Vec<Matrix> {
    let mut new_grid_parks = Vec::new();
    'outer: for chunk in chunks {
        if transforms.contains_key(chunk) {
            new_grid_parks.push(transforms.get(chunk).unwrap().clone());
            continue;
        }
        let mut rotated = rotate(&chunk);
        for _ in 0..3 {
            if transforms.contains_key(&rotated) {
                new_grid_parks.push(transforms.get(&rotated).unwrap().clone());
                continue 'outer;
            }
            rotated = rotate(&rotated);
        }
        rotated = mirror(&rotated);
        for _ in 0..4 {
            if transforms.contains_key(&rotated) {
                new_grid_parks.push(transforms.get(&rotated).unwrap().clone());
                continue 'outer;
            }
            rotated = rotate(&rotated);
        }
        print(&rotated);
        panic!("rotation not found");
    }
    new_grid_parks
}

// .#.
// ..#
// ###
//
// .#.
// #..
// ###
//
fn mirror(grid: &Matrix) -> Matrix {
    let mut new_grid = Vec::new();
    for row in 0..grid.len() { //0..len
        let mut new_row = Vec::new();
        for column in (0..grid.len()).rev() { //0..len
            new_row.push(grid[row][column]);
        }
        new_grid.push(new_row);
    }
    new_grid
}

fn rotate(grid: &Matrix) -> Matrix {
    // 0 1 2
    // 3 4 5
    // 6 7 8
    //
    //
    // getting
    // 0 3 6
    // 1 4 7
    // 2 5 8
    //
    // need
    // 6 3 0
    // 7 4 1
    // 8 5 2
    //
    // 0 1
    // 2 3
    //
    // 2 0
    // 3 1
    let mut new_grid = Vec::new();
    for column in 0..grid.len() { //0..len
        let mut new_row = Vec::new();
        for row in (0..grid.len()).rev() { //0..len
            new_row.push(grid[row][column]);
        }
        new_grid.push(new_row);
    }
    new_grid
}

fn split2(grid: &Matrix) -> Vec<Matrix> {
    let mut chunks = Vec::new();
    let mut rows = grid.iter().peekable();
    while rows.peek().is_some() {
        let mut first = rows.next().unwrap().chunks(2).peekable();
        let mut second = rows.next().unwrap().chunks(2);
        while first.peek().is_some() {
            let mut chunk = Vec::new();
            chunk.push(first.next().unwrap().to_vec());
            chunk.push(second.next().unwrap().to_vec());
            chunks.push(chunk);
        }
    }

    chunks
}


fn split3(grid: &Matrix) -> Vec<Matrix> {
    let mut chunks = Vec::new();
    let mut rows = grid.iter().peekable();
    while rows.peek().is_some() {
        let mut first = rows.next().unwrap().chunks(3).peekable();
        let mut second = rows.next().unwrap().chunks(3);
        let mut third = rows.next().unwrap().chunks(3);
        while first.peek().is_some() {
            let mut chunk = Vec::new();
            chunk.push(first.next().unwrap().to_vec());
            chunk.push(second.next().unwrap().to_vec());
            chunk.push(third.next().unwrap().to_vec());
            chunks.push(chunk);
        }
    }

    chunks
}

fn fractalize(input: &str) -> usize {
    let mut grid: Matrix = vec![
        vec![false, true, false],
        vec![false, false, true],
        vec![true, true, true]
    ];

    let mut transforms = HashMap::new();
    for line in input.lines() {
        //parsing ../.# => ##./#../...
        let mut parts = line.split(" => ");
        let from = line_to_matrix(parts.next().unwrap());
        let to = line_to_matrix(parts.next().unwrap());

        transforms.insert(from.clone(), to.clone());
    }

    for x in 0..5 {
        println!("step {}", x);
        print(&grid);
        grid = step(&grid, &transforms);
    }

    count_on(&grid)
}
