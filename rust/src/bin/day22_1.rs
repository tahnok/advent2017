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

const INFECTED: bool = true;
const CLEAN: bool = false;

fn infected(input: &str) -> usize {
    let mut grid = HashMap::new();
    for (y, line) in input.lines().enumerate() {
        for (x, square) in line.chars().enumerate() {
            let point = ((x as isize), (y as isize));
            match square {
                '#' => {
                    grid.insert(point, INFECTED);
                },
                '.' => {
                    grid.insert(point, CLEAN);
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
    for _ in 0..10_000 {
    //for _ in 0..6 {
        let cell = match grid.get(&(x,y)) {
            Some(cell) => *cell,
            None => CLEAN
        };
        if cell == INFECTED { //turn right
            match direction {
                Direction::Down => direction = Direction::Left,
                Direction::Up => direction = Direction::Right,
                Direction::Left => direction = Direction::Up,
                Direction::Right => direction = Direction::Down,
            }
        } else {
            match direction { //turn lfet
                Direction::Down => direction = Direction::Right,
                Direction::Up => direction = Direction::Left,
                Direction::Left => direction = Direction::Down,
                Direction::Right => direction = Direction::Up,
            }
        }

        if cell == INFECTED {
            grid.insert((x,y), CLEAN);
        } else { //not infected
            infected_count += 1;
            grid.insert((x,y), INFECTED);
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

