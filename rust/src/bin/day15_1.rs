
fn main() {
    println!("{}", final_count());
}

const A: usize = 16807;
const B: usize = 48271;
const MOD: usize = 2147483647;
const MASK: usize = 0b1111_1111_1111_1111;
const COUNT: usize = 40_000_000;

fn final_count() -> usize {
    let mut a = 289;
    let mut b = 629;

    let mut count = 0;
    for x in 0..COUNT {
        a = (a * A) % MOD;
        b = (b * B) % MOD;
        if (a & MASK) == (b & MASK) {
            count += 1;
        }
    }
    count
}
