extern crate primal;

fn main() {
    println!("{}", solve());
}

fn solve() -> usize {
    // see spreadsheet for work
    // but go from go from 106700 to 123700 stepping by 17
    // and check to see how many of those numbers have a * b = c
    let mut a = 106700;
    let mut found = 0;
    while a < 123701 {
        if !primal::is_prime(a) {
            found += 1;
        }

        a += 17;
    }
    found 
}
