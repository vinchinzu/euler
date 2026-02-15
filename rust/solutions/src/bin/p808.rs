// Project Euler 808 - Reversible Prime Squares
// Find sum of first 50 prime squares whose reversal is also a different prime square

fn main() {
    const LIMIT: usize = 100_000_001;

    // Bitset sieve
    let mut sieve = vec![0u8; (LIMIT >> 3) + 2];
    macro_rules! is_composite {
        ($n:expr) => { sieve[$n >> 3] & (1 << ($n & 7)) != 0 }
    }
    macro_rules! set_composite {
        ($n:expr) => { sieve[$n >> 3] |= 1 << ($n & 7) }
    }

    set_composite!(0);
    set_composite!(1);
    let mut i = 2usize;
    while i * i < LIMIT {
        if !is_composite!(i) {
            let mut j = i * i;
            while j < LIMIT {
                set_composite!(j);
                j += i;
            }
        }
        i += 1;
    }

    fn reverse_num(mut n: u64) -> u64 {
        let mut rev = 0u64;
        while n > 0 {
            rev = rev * 10 + n % 10;
            n /= 10;
        }
        rev
    }

    fn isqrt(n: u64) -> u64 {
        let mut x = (n as f64).sqrt() as u64;
        while x > 0 && x * x > n { x -= 1; }
        while (x + 1) * (x + 1) <= n { x += 1; }
        x
    }

    let mut count = 0;
    let mut sum: u64 = 0;

    for p in 2..LIMIT {
        if count >= 50 { break; }
        if is_composite!(p) { continue; }
        let sq = p as u64 * p as u64;
        let rev = reverse_num(sq);
        if rev == sq { continue; }
        let sr = isqrt(rev);
        if sr * sr != rev { continue; }
        if sr as usize >= LIMIT { continue; }
        if !is_composite!(sr as usize) {
            sum += sq;
            count += 1;
        }
    }

    println!("{}", sum);
}
