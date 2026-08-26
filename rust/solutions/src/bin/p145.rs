// Project Euler 145: reversible numbers below 10^9.
// Carry analysis of n + reverse(n) (no trailing zeros):
//   even length 2k:     20 * 30^{k-1}
//   length ≡ 1 (mod 4): 0
//   length ≡ 3 (mod 4): 100 * 500^{(L-3)/4}

fn main() {
    let mut total: i64 = 0;
    let mut even = 20i64;
    for _ in 0..4 {
        total += even;
        even *= 30;
    }
    total += 100;
    total += 100 * 500;
    println!("{}", total);
}
