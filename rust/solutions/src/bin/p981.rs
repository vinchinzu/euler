// Problem 981: The Quaternion Group II
// Ported from python/981.py.

const MOD: u64 = 888_888_883;

fn mod_pow(mut base: u64, mut exp: u64, modu: u64) -> u64 {
    let mut res = 1u64;
    base %= modu;
    while exp > 0 {
        if exp & 1 == 1 {
            res = ((res as u128 * base as u128) % modu as u128) as u64;
        }
        base = ((base as u128 * base as u128) % modu as u128) as u64;
        exp >>= 1;
    }
    res
}

#[inline]
fn comb_mod(n: usize, k: usize, fact: &[u32], invfact: &[u32]) -> u64 {
    if k > n {
        return 0;
    }
    let a = fact[n] as u64;
    let b = invfact[k] as u64;
    let c = invfact[n - k] as u64;
    (((a as u128 * b as u128) % MOD as u128 * c as u128) % MOD as u128) as u64
}

fn solve() -> u64 {
    let cubes: Vec<usize> = (0usize..88).map(|i| i * i * i).collect();
    let max_n = 3 * cubes[cubes.len() - 1];

    let mut fact = vec![1u32; max_n + 1];
    for i in 1..=max_n {
        fact[i] = ((fact[i - 1] as u64 * i as u64) % MOD) as u32;
    }

    let mut invfact = vec![1u32; max_n + 1];
    invfact[max_n] = mod_pow(fact[max_n] as u64, MOD - 2, MOD) as u32;
    for i in (1..=max_n).rev() {
        invfact[i - 1] = ((invfact[i] as u64 * i as u64) % MOD) as u32;
    }

    let inv2 = (MOD + 1) / 2;

    let halves: Vec<usize> = cubes.iter().map(|&c| c >> 1).collect();
    let invf: Vec<u64> = cubes.iter().map(|&c| invfact[c] as u64).collect();
    let par: Vec<usize> = (0usize..88).map(|i| i & 1).collect();

    let mut total_sum = 0u64;

    for (ai, &x) in cubes.iter().enumerate() {
        let hx = halves[ai];
        let inv_x = invf[ai];
        let px = par[ai];

        for (bj, &y) in cubes.iter().enumerate() {
            let hy = halves[bj];
            let inv_y = invf[bj];
            let py = par[bj];

            for (ck, &z) in cubes.iter().enumerate() {
                if px != py || py != par[ck] {
                    continue;
                }

                let hz = halves[ck];
                let inv_z = invf[ck];
                let n = x + y + z;

                let mut t = fact[n] as u64;
                t = ((t as u128 * inv_x as u128) % MOD as u128) as u64;
                t = ((t as u128 * inv_y as u128) % MOD as u128) as u64;
                t = ((t as u128 * inv_z as u128) % MOD as u128) as u64;

                let d: u64 = if (n & 1) == 0 && (x & 1) == 1 {
                    0
                } else {
                    let d1 = comb_mod(n >> 1, hx, &fact, &invfact);
                    let n2 = n - x;
                    if (n2 & 1) == 0 && (y & 1) == 1 {
                        0
                    } else {
                        let d2 = comb_mod(n2 >> 1, hy, &fact, &invfact);
                        ((d1 as u128 * d2 as u128) % MOD as u128) as u64
                    }
                };

                let mut nmod = if ((hx + hy + hz) & 1) == 0 {
                    (t + d) % MOD
                } else {
                    (t + MOD - d) % MOD
                };
                nmod = ((nmod as u128 * inv2 as u128) % MOD as u128) as u64;

                total_sum += nmod;
                if total_sum >= MOD {
                    total_sum -= MOD;
                }
            }
        }
    }

    total_sum % MOD
}

fn main() {
    println!("{}", solve());
}
