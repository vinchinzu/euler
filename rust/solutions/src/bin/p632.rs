// Project Euler 632 - Square prime factors
// Sieve Mobius/omega up to sqrt(N)=10^8, then count C_k(N)

const N: i64 = 10_000_000_000_000_000;
const M: i64 = 1_000_000_007;
const L: usize = 100_000_000;

fn main() {
    let mut mobius = vec![1i8; L + 1];
    let mut omega = vec![0u8; L + 1];

    let mut is_prime = vec![true; L + 1];
    for i in 2..=L {
        if is_prime[i] {
            for j in (i..=L).step_by(i) {
                is_prime[j] = false;
                mobius[j] *= -1;
                omega[j] += 1;
            }
            let mut j = i as u64 * i as u64;
            while j <= L as u64 {
                mobius[j as usize] = 0;
                j += i as u64 * i as u64;
            }
        }
    }
    drop(is_prime);

    let max_k = (L as f64).log2() as usize;

    let mut ncr = vec![vec![0i64; max_k + 1]; max_k + 1];
    for i in 0..=max_k {
        ncr[i][0] = 1;
        for j in 1..=i { ncr[i][j] = (ncr[i - 1][j - 1] + ncr[i - 1][j]) % M; }
    }

    let mut c = vec![0i64; max_k + 1];
    c[0] = N % M;

    for n in 2..=L {
        if mobius[n] == 0 { continue; }
        let k = omega[n] as usize;
        let count = (N / (n as i64 * n as i64)) % M;
        for i in 0..=k {
            let parity = if i % 2 == 0 { 1i64 } else { -1 };
            c[k - i] = ((c[k - i] + parity * (ncr[k][i] as i128 * count as i128 % M as i128) as i64) % M + M) % M;
        }
    }

    let mut ans = 1i64;
    for i in 0..=max_k {
        if c[i] != 0 {
            ans = (ans as i128 * c[i] as i128 % M as i128) as i64;
        }
    }

    println!("{}", ans);
}
