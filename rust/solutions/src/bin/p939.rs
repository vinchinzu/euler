// Project Euler 939 - Nim Variant
// E(N) using partition function.
// E(n) = sum_{k=1}^{n} sum_{s=ceil(k/2+1/2)}^{k} p(s)*p(k-s) mod MOD
// where p is the partition function computed via the pentagonal number theorem.

fn main() {
    const MOD: i64 = 1234567891;
    const N: usize = 5000;

    let mut p = vec![0i64; N + 1];
    p[0] = 1;

    // Compute partition function via pentagonal number theorem
    for i in 1..=N {
        let mut val: i64 = 0;
        for k in 1.. {
            let pent1 = k * (3 * k - 1) / 2;
            let pent2 = k * (3 * k + 1) / 2;
            let sign: i64 = if k % 2 == 1 { 1 } else { -1 };

            if pent1 <= i as i64 {
                val = (val + sign * p[(i as i64 - pent1) as usize]) % MOD;
            }
            if pent2 <= i as i64 {
                val = (val + sign * p[(i as i64 - pent2) as usize]) % MOD;
            }

            if pent1 > i as i64 && pent2 > i as i64 {
                break;
            }
        }
        p[i] = ((val % MOD) + MOD) % MOD;
    }

    let mut result: i64 = 0;
    for k in 1..=N {
        let s_start = (k + 1) / 2;
        for s in s_start..=k {
            let t = k - s;
            let ways = p[s] % MOD * (p[t] % MOD) % MOD;
            result = (result + ways) % MOD;
        }
    }

    println!("{}", result);
}
