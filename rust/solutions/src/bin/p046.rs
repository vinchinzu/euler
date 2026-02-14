// Project Euler 46: Goldbach's other conjecture
// Find the smallest odd composite that cannot be written as prime + 2*square.
use euler_utils::is_prime;

fn main() {
    let mut n: u64 = 9;
    loop {
        if n % 2 == 1 && !is_prime(n) {
            let mut found = false;
            let mut s: u64 = 1;
            while 2 * s * s < n {
                let rem = n - 2 * s * s;
                if is_prime(rem) {
                    found = true;
                    break;
                }
                s += 1;
            }
            if !found {
                println!("{n}");
                return;
            }
        }
        n += 2;
    }
}
