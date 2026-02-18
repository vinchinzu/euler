// Exploring binary partitions p(n)
// p(0)=1, p(2m+1)=p(2m), p(2m)=p(2m-1)+p(m)

const MOD: u64 = 1_000_000_007;

fn main() {
    // Compute p(n) for small n
    let limit = 10_000_000;
    let mut p = vec![0u64; limit + 1];
    p[0] = 1;
    for n in 1..=limit {
        if n % 2 == 1 {
            p[n] = p[n - 1];
        } else {
            p[n] = (p[n - 1] + p[n / 2]) % MOD;
        }
    }

    // Check p(7^k) for small k
    let mut pow7 = 1u64;
    for k in 0..=8 {
        if pow7 as usize <= limit {
            println!("p(7^{}) = p({}) = {}", k, pow7, p[pow7 as usize]);
        }
        pow7 *= 7;
    }

    // Let's also look at the binary representation of 7^k
    pow7 = 1;
    for k in 0..=8 {
        if pow7 <= 10_000_000 {
            println!("7^{} = {} = {:b}", k, pow7, pow7);
        }
        pow7 *= 7;
    }
}
