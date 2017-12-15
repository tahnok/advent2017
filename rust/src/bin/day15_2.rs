
fn main() {
    println!("{}", final_count());
}

const A: usize = 16807;
const B: usize = 48271;
const MOD: usize = 2147483647;
const MASK: usize = 0b1111_1111_1111_1111;
const COUNT: usize = 5_000_000;

fn final_count() -> usize {
    let mut a = 289;
    //let mut a = 65;
    let mut b = 629;
    //let mut b = 8921;

    let mut count = 0;
    for x in 0..COUNT {
        let mut a_found = false;
        let mut b_found = false;
        while !a_found || !b_found {
            if !a_found {
                a = (a * A) % MOD;
                if a % 4 == 0 {
                    a_found = true;
                }
            }
            if !b_found {
                b = (b * B) % MOD;
                if b % 8 == 0 {
                    b_found = true;
                }
            }
        }
        if (a & MASK) == (b & MASK) {
            count += 1;
        }
    }
    count
}
