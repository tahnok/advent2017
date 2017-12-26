use std::collections::HashMap;
use std::io;
use std::io::Read;

fn main() {
    let mut input = String::new();
    let _ = io::stdin().read_to_string(&mut input);
    println!("{}", infected(&input.trim()));
}

enum Direction {
    Up,
    Down,
    Left,
    Right
}

#[derive(Clone, Copy, Debug)]
enum Node {
    Clean,
    Weakened,
    Infected,
    Flagged
}

fn infected(input: &str) -> usize {
    let mut grid = HashMap::new();
    for (y, line) in input.lines().enumerate() {
        for (x, square) in line.chars().enumerate() {
            let point = ((x as isize), (y as isize));
            match square {
                '#' => {
                    grid.insert(point, Node::Infected);
                },
                '.' => {
                    grid.insert(point, Node::Clean);
                },
                _ => panic!("unexpected character {}", square)
            }
        }
    }
    
    let height = input.lines().count() as isize;
    let width = input.lines().next().unwrap().chars().count() as isize;

    let mut direction = Direction::Up;
    let mut x = width / 2;
    let mut y = height / 2;

    let mut infected_count = 0;
    for _ in 0..10_000_000 {
        let cell = match grid.get(&(x,y)) {
            Some(cell) => (*cell).clone(),
            None => Node::Clean
        };
        match cell {
            Node::Clean => {
                match direction { //turn lfet
                    Direction::Up => direction = Direction::Left,
                    Direction::Left => direction = Direction::Down,
                    Direction::Down => direction = Direction::Right,
                    Direction::Right => direction = Direction::Up,
                }
                grid.insert((x,y), Node::Weakened);
            },
            Node::Weakened => {
                grid.insert((x,y), Node::Infected);
                infected_count += 1;
            },
            Node::Infected => {
                match direction { //turn right
                    Direction::Down => direction = Direction::Left,
                    Direction::Up => direction = Direction::Right,
                    Direction::Left => direction = Direction::Up,
                    Direction::Right => direction = Direction::Down,
                }
                grid.insert((x,y), Node::Flagged);
            },
            Node::Flagged => {
                match direction { //turn around
                    Direction::Down => direction = Direction::Up,
                    Direction::Up => direction = Direction::Down,
                    Direction::Left => direction = Direction::Right,
                    Direction::Right => direction = Direction::Left,
                }
                grid.insert((x,y), Node::Clean);
            }
        }


        match direction {
            Direction::Down => y += 1,
            Direction::Up => y -= 1,
            Direction::Left => x -= 1,
            Direction::Right => x += 1,
        }
    }
    infected_count
}

