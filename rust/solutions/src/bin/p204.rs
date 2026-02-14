// Project Euler 204: Generalised Hamming Numbers
//
// Count integers up to 10^9 with no prime factor > 100 (100-smooth numbers).

use euler_utils::primes_up_to;

fn main() {
    let n: u64 = 1_000_000_000;
    let primes = primes_up_to(100);

    let mut nums: Vec<u64> = vec![1];

    for &p in &primes {
        let p = p as u64;
        let sz = nums.len();
        for i in 0..sz {
            let mut prod = nums[i];
            loop {
                prod = match prod.checked_mul(p) {
                    Some(v) if v <= n => v,
                    _ => break,
                };
                nums.push(prod);
            }
        }
    }

    println!("{}", nums.len());
}
