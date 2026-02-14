// Project Euler 12: First triangle number with over 500 divisors
use euler_utils::divisor_count;

fn main() {
    let mut i = 1u64;
    let mut tri = 1u64;

    while divisor_count(tri) <= 500 {
        i += 1;
        tri += i;
    }

    println!("{tri}");
}
