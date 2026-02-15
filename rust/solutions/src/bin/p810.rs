// Project Euler 810 - XOR-Primes
// Sieve for XOR-primes and find the 5,000,000th one

fn main() {
    const N: usize = 5_000_000;
    const L: usize = 1 << 27; // 134217728

    let mut sieve = vec![true; L];
    sieve[0] = false;
    sieve[1] = false;

    for i in 2..L {
        if sieve[i] {
            // Mark composites: XOR products of i with j >= i
            for j in i..L {
                // XOR product: m = XOR of j*(k & -k) for each set bit k in i
                let mut m: u64 = 0;
                let mut k = i as u64;
                while k > 0 {
                    let bit = k & k.wrapping_neg();
                    m ^= j as u64 * bit;
                    k -= bit;
                }
                if m >= L as u64 { break; }
                sieve[m as usize] = false;
            }
        }
    }

    let mut count = 0;
    let mut ans = 0;
    for i in 2..L {
        if sieve[i] {
            count += 1;
            if count == N {
                ans = i;
                break;
            }
        }
    }

    println!("{}", ans);
}
