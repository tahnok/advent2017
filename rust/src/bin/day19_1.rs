use std::io;
use std::io::Read;

fn main() {
    let mut input = String::new();
    let _ = io::stdin().read_to_string(&mut input);
    println!("{:?}", path(input.trim()));
}

#[derive(Debug, PartialEq)]
enum Square {
    Empty,
    Horizontal,
    Vertical,
    Turn,
    Letter(char)
}

enum Direction {
    Up,
    Down,
    Left,
    Right
}

fn path(input: &str) -> Vec<char> {
    let mut grid = Vec::new();
    for line in input.lines() {
        let mut row = Vec::new();
        for square in line.trim().chars() {
            let val = match square {
                ' ' => Square::Empty,
                '|' => Square::Vertical,
                '-' => Square::Horizontal,
                '+' => Square::Turn,
                _ => Square::Letter(square)
            };

            row.push(val);
        }
        grid.push(row);
    }
    
    let mut row = 0;
    let mut column = grid[0].iter().position(|square| *square == Square::Vertical).unwrap();
    let mut direction = Direction::Down;

    let mut seen = Vec::new();
    loop {
        let square = &grid[row][column];
        match *square {
            Square::Empty => panic!("out of bounds at {} {}", row, column),
            Square::Vertical => {
                match direction {
                    Direction::Down => row += 1,
                    Direction::Up => row -= 1,
                    _ => panic!("trying to move vertically while facing horizontally at {}, {}", row, column)
                }
            },
            _ => panic!("not done")

        }
        break;
    }

    seen
}

