use std::io;
use std::io::Read;

fn main() {
    let mut input = String::new();
    let _ = io::stdin().read_to_string(&mut input);
    println!("{}", path(&input));
}

#[derive(Debug, PartialEq)]
enum Square {
    Empty,
    LeftRight,
    UpDown,
    Turn,
    Letter(char)
}

enum Direction {
    Up,
    Down,
    Left,
    Right
}

fn path(input: &str) -> String {
    let mut grid = Vec::new();
    for line in input.lines() {
        let mut row = Vec::new();
        for square in line.chars() {
            let val = match square {
                ' ' => Square::Empty,
                '|' => Square::UpDown,
                '-' => Square::LeftRight,
                '+' => Square::Turn,
                _ => Square::Letter(square)
            };

            row.push(val);
        }
        grid.push(row);
    }
    
    let height = grid.len();
    let width = grid[0].len();

    for row in 0..height {
        print!("{}: ", row);
        for column in 0..width {
            let square = &grid[row][column];
            let val = match *square {
                Square::Empty => ' ',
                Square::LeftRight => '-',
                Square::UpDown => '|',
                Square::Turn => '+',
                Square::Letter(x) => x
            };
            print!("{}", val);
        }
        println!("");
    }
    
    let mut row = 0;
    let mut column = grid[0].iter().position(|square| *square == Square::UpDown).unwrap();
    let mut direction = Direction::Down;

    let mut seen = Vec::new();
    loop {
        let square = &grid[row][column];
        match *square {
            Square::Empty => break,
            Square::UpDown | Square::LeftRight => {
                match direction {
                    Direction::Down => row += 1,
                    Direction::Up => row -= 1,
                    Direction::Left => column -= 1,
                    Direction::Right => column += 1,
                }
            },
            Square::Letter(x) => {
                seen.push(x);
                match direction {
                    Direction::Down => row += 1,
                    Direction::Up => row -= 1,
                    Direction::Left => column -= 1,
                    Direction::Right => column += 1,
                }
            },
            Square::Turn => {
                match direction {
                    Direction::Up | Direction::Down => { //turning left or right
                        if (column as isize - 1) > 0 && grid[row][column - 1] != Square::Empty {
                            direction = Direction::Left;
                            column -= 1;
                        } else if (column + 1) < width && grid[row][column + 1] != Square::Empty {
                            direction = Direction::Right;
                            column += 1;
                        } else {
                            println!("{:?}", seen);
                            panic!("no left or right square at {} {}", row, column);
                        }
                    },
                    Direction::Left | Direction::Right => {
                        if (row as isize - 1) > 0 && grid[row - 1][column] != Square::Empty {
                            direction = Direction::Up;
                            row -= 1;
                        } else if (row + 1) < height && grid[row + 1][column] != Square::Empty {
                            direction = Direction::Down;
                            row += 1;
                        } else {
                            panic!("no up or right down at {} {}", row, column);
                        }
                    }
                }
            }
        }
    }

    seen.into_iter().collect()
}

