// Project Euler 651 - Patterns of Rectangular Stickers
// Burnside's Lemma for counting patterns on a cylinder with Fibonacci params.
use euler_utils::*;

const MOD: i64 = 1_000_000_007;

fn power_mod(mut base: i64, mut exp: i64, m: i64) -> i64 {
    let mut r = 1i64;
    base = ((base % m) + m) % m;
    while exp > 0 {
        if exp & 1 == 1 { r = (r as i128 * base as i128 % m as i128) as i64; }
        base = (base as i128 * base as i128 % m as i128) as i64;
        exp >>= 1;
    }
    r
}

fn mod_inverse(a: i64, m: i64) -> i64 {
    power_mod(a, m - 2, m)
}

fn euler_phi_val(mut n: i64) -> i64 {
    let mut result = n;
    let mut temp = n;
    let mut p = 2i64;
    while p * p <= temp {
        if temp % p == 0 {
            while temp % p == 0 { temp /= p; }
            result = result / p * (p - 1);
        }
        p += 1;
    }
    if temp > 1 { result = result / temp * (temp - 1); }
    result
}

fn get_divisors(n: i64) -> Vec<i64> {
    let mut divs = Vec::new();
    let mut i = 1i64;
    while i * i <= n {
        if n % i == 0 {
            divs.push(i);
            if i != n / i { divs.push(n / i); }
        }
        i += 1;
    }
    divs.sort();
    divs
}

struct CycleInfo {
    pairs: Vec<(i64, i64)>, // (len, cnt)
    multiplicity: i64,
}

fn get_cycle_infos(n: i64, reflect: bool) -> Vec<CycleInfo> {
    let mut infos = Vec::new();
    if reflect {
        if n % 2 == 1 {
            infos.push(CycleInfo {
                pairs: vec![(2, n / 2), (1, 1)],
                multiplicity: n,
            });
        } else {
            infos.push(CycleInfo {
                pairs: vec![(2, n / 2)],
                multiplicity: n / 2,
            });
            infos.push(CycleInfo {
                pairs: vec![(2, n / 2 - 1), (1, 2)],
                multiplicity: n / 2,
            });
        }
    } else {
        let divs = get_divisors(n);
        for d in divs {
            infos.push(CycleInfo {
                pairs: vec![(n / d, d)],
                multiplicity: euler_phi_val(n / d),
            });
        }
    }
    infos
}

fn f_func(m: i64, a: i64, b: i64, ncr_table: &[[i64; 50]; 50]) -> i64 {
    let mut result = 0i64;
    for rw in 0..2 {
        for rb in 0..2 {
            let infos_a = get_cycle_infos(a, rw == 1);
            let infos_b = get_cycle_infos(b, rb == 1);
            for ia in &infos_a {
                for ib in &infos_b {
                    let mut num_cycles = 0i64;
                    for &(la, ca) in &ia.pairs {
                        for &(lb, cb) in &ib.pairs {
                            num_cycles += gcd(la as u64, lb as u64) as i64 * ca * cb;
                        }
                    }
                    for i in 0..m as usize {
                        let sign = if i % 2 == 0 { 1i64 } else { MOD - 1 };
                        let mut term = sign % MOD;
                        term = (term as i128 * ncr_table[m as usize][i] as i128 % MOD as i128) as i64;
                        term = (term as i128 * power_mod(m - i as i64, num_cycles, MOD) as i128 % MOD as i128) as i64;
                        term = (term as i128 * (ia.multiplicity % MOD) as i128 % MOD as i128) as i64;
                        term = (term as i128 * (ib.multiplicity % MOD) as i128 % MOD as i128) as i64;
                        result = (result + term) % MOD;
                    }
                }
            }
        }
    }
    let inv = mod_inverse(4 * (a % MOD) % MOD * (b % MOD) % MOD, MOD);
    (result as i128 * inv as i128 % MOD as i128) as i64
}

fn main() {
    let n = 40;
    let mut ncr_table = [[0i64; 50]; 50];
    for i in 0..=45 {
        ncr_table[i][0] = 1;
        for j in 1..=i {
            ncr_table[i][j] = (ncr_table[i-1][j-1] + ncr_table[i-1][j]) % MOD;
        }
    }

    let mut fib = [0i64; 50];
    fib[0] = 0; fib[1] = 1;
    for i in 2..=n+1 { fib[i] = fib[i-1] + fib[i-2]; }

    let mut ans = 0i64;
    for i in 4..=n {
        ans = (ans + f_func(i as i64, fib[i-1], fib[i], &ncr_table)) % MOD;
    }
    println!("{}", ans);
}
