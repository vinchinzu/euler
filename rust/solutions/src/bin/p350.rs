// Project Euler 350 - Constrained sequences
// f(G, L, N) mod 101^4, where G=10^6, L=10^12, N=10^18.

use euler_utils::mod_pow;

const G: i64 = 1_000_000;
const L: i64 = 1_000_000_000_000;
const NN: u64 = 1_000_000_000_000_000_000;
const M: u64 = 104_060_401; // 101^4

fn main() {
    let r = L / G; // 10^6

    // Sieve smallest prime factor
    let mut spf = vec![0u32; (r + 1) as usize];
    for i in 0..=r as usize { spf[i] = i as u32; }
    {
        let mut i = 2i64;
        while i * i <= r {
            if spf[i as usize] == i as u32 {
                let mut j = i * i;
                while j <= r {
                    if spf[j as usize] == j as u32 {
                        spf[j as usize] = i as u32;
                    }
                    j += i;
                }
            }
            i += 1;
        }
    }

    // Precompute powers of e mod M for e up to ~22
    let max_e = 22usize;
    let mut pow_n = vec![0u64; max_e + 2];
    for e in 0..=max_e + 1 {
        pow_n[e] = mod_pow(e as u64, NN, M);
    }

    let mut ans: u64 = 0;
    for ri in 1..=r {
        let mut temp = ri as u32;
        let mut res = 1u64;
        while temp > 1 {
            let p = spf[temp as usize];
            let mut e = 0u32;
            while temp % p == 0 { temp /= p; e += 1; }
            let factor = if e > 0 {
                (pow_n[(e + 1) as usize] + M + M - 2 * pow_n[e as usize] + pow_n[(e - 1) as usize]) % M
            } else {
                (pow_n[1] + M + M - 2 * pow_n[0] + 0) % M // shouldn't happen
            };
            res = (res as u128 * factor as u128 % M as u128) as u64;
        }

        let count = ((L / ri - G + 1) % M as i64 + M as i64) as u64 % M;
        ans = (ans as u128 + count as u128 * res as u128) as u64 % M;
    }

    println!("{}", ans % M);
}
