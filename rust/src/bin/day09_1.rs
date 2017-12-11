use std::io;
use std::io::Read;


fn main() {
    let mut input = String::new();
    let _ = io::stdin().read_to_string(&mut input);
    println!("{}", score(input.trim()));
}

fn score(input: &str) -> usize {
    let mut state = State::Group;
    let mut stack = Vec::new();
    stack.push(State::Done);

    let mut score = 0;
    let mut score_increment = 1;
    for symbol in input.chars().skip(1) {
        match state {
            State::Garbage => {
                match symbol {
                    '!' => {
                        stack.push(state);
                        state = State::Ignore;
                    },
                    '>' => state = stack.pop().unwrap(),
                    _ => ()
                }
            },
            State::Ignore => state = stack.pop().unwrap(),
            State::Group => {
                match symbol {
                    '!' => {
                        stack.push(state);
                        state = State::Ignore;
                    },
                    '<' => {
                        stack.push(state);
                        state = State::Garbage;
                    },
                    ',' => (), //continue
                    '{' => {
                        stack.push(state);
                        state = State::Group;
                        score_increment += 1;
                    },
                    '}' => {
                        state = stack.pop().unwrap();
                        score += score_increment;
                        score_increment -= 1;
                    },
                    _ => panic!("unexpected symbol {}", symbol)
                }
            },
            State::Done => {
                println!("done");
            }
        }
    }
    score 
}

enum State {
    Garbage,
    Ignore,
    Group,
    Done
}

//possible states
// - garbage:
//   - if > pop state stack
//   - if ! move to ingore state and push garbage to state stack
//   - else ignore
// - ignore:
//   - skip next symbol and pop state stack
// - group:
//   - if < move to garbage state
//   - if ! move to ignore state
//   - if { increase group depth and push group to state stack
//   - if } decrease group depth and pop state stack
//   - if , next
//   - else panic
