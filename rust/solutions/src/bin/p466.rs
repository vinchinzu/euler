// Project Euler 466: Distinct products
// For m from 1 to K=64, count how many n <= N=10^16 have a unique
// representation as n*m among {m*1, m*2, ..., m*K}.
// Uses recursive inclusion-exclusion with factored coprime splitting.

const N: i64 = 10_000_000_000_000_000; // 10^16
const K: usize = 64;

fn gcd(mut a: i32, mut b: i32) -> i32 {
    while b != 0 { let t = b; b = a % b; a = t; }
    a
}

static mut FF: [i32; K + 1] = [0; K + 1];

fn preff() {
    unsafe {
        for i in 2..=K {
            if FF[i] == 0 {
                let mut j = i;
                while j <= K {
                    FF[j] = i as i32;
                    j += i;
                }
            }
        }
    }
}

fn num_not_divisible_by(n: i64, factors: &[i32]) -> i64 {
    let nf = factors.len();
    for &f in factors {
        if f == 1 { return 0; }
    }

    // Check for non-coprime pairs
    for i in 0..nf {
        for j in (i + 1)..nf {
            let p = unsafe { FF[gcd(factors[i], factors[j]) as usize] };
            if p > 1 {
                // Split on common prime p
                let mut new_factors1 = Vec::with_capacity(nf);
                let mut new_factors2 = Vec::with_capacity(nf);
                new_factors2.push(p);
                for (k, &f) in factors.iter().enumerate() {
                    let _ = k;
                    if f % p == 0 {
                        new_factors1.push(f / p);
                    } else {
                        new_factors1.push(f);
                        new_factors2.push(f);
                    }
                }
                return num_not_divisible_by(n / p as i64, &new_factors1)
                     + num_not_divisible_by(n, &new_factors2);
            }
        }
    }

    // All remaining factors are coprime - inclusion-exclusion
    let mut result: i64 = 0;
    for subset in 0..(1u64 << nf) {
        let mut count = n;
        for i in 0..nf {
            if subset & (1 << i) != 0 {
                count /= -(factors[i] as i64);
            }
        }
        result += count;
    }
    result
}

fn main() {
    preff();

    let mut ans: i64 = 0;
    for m in 1..=K {
        let mut factors = Vec::new();
        for i in (m + 1)..=K {
            factors.push((i as i32) / gcd(i as i32, m as i32));
        }
        ans += num_not_divisible_by(N, &factors);
    }

    println!("{}", ans);
}
