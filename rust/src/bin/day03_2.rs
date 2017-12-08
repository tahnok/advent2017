use std::collections::HashMap;
use std::io;
use std::io::Read;

fn main() {
    let mut input = String::new();
    let _ = io::stdin().read_to_string(&mut input);
    let target = input.trim().parse::<u32>().unwrap();

    let mut map = HashMap::new();
    map.insert((0,0), 1);

    let mut dir = Dir::RIGHT;
    let mut loc: (i32, i32) = (0,0);
    loop {
        match dir {
            Dir::UP    => loc = (loc.0, loc.1 + 1),
            Dir::DOWN  => loc = (loc.0, loc.1 - 1),
            Dir::LEFT  => loc = (loc.0 - 1, loc.1),
            Dir::RIGHT => loc = (loc.0 + 1, loc.1),
        }
        //println!("placing {} at {:?} while facing {:?}", x, loc, dir);
        if map.contains_key(&loc) { //cell to my left is not filled, then turn left
            panic!("trying to insert at filled spot")
        }

        let mut total = 0;
        for &(x,y) in [(0,1),(0,-1),(-1,0),(1,0),(1,1),(1,-1),(-1,1),(-1,-1)].iter() {
            match map.get(&(loc.0+x, loc.1+y)) {
                Some(val) => total = total + val,
                None => (),
            }
        }

        if total > target {
            println!("{}", total);
            break;
        } else {
            map.insert(loc, total);
        }

        let neighour_loc = match dir {
            Dir::UP    => (loc.0 - 1, loc.1),
            Dir::DOWN  => (loc.0 + 1, loc.1),
            Dir::LEFT  => (loc.0, loc.1 - 1),
            Dir::RIGHT => (loc.0, loc.1 + 1),
        };

        if !map.contains_key(&neighour_loc) { //cell to my left is not filled, then turn left
            match dir {
                Dir::UP    => dir = Dir::LEFT,
                Dir::DOWN  => dir = Dir::RIGHT,
                Dir::LEFT  => dir = Dir::DOWN,
                Dir::RIGHT => dir = Dir::UP,
            }
        }
    }
}

#[derive(Debug)]
enum Dir {
    UP,
    DOWN,
    LEFT,
    RIGHT,
}
