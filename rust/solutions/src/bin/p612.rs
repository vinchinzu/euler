// Project Euler 612 - Friend numbers
// Inclusion-exclusion over digit bitmasks

const N_VAL: u64 = 18;
const M_VAL: u64 = 1_000_267_129;

fn pow_mod(mut base: u64, mut exp: u64, m: u64) -> u64 {
    let mut result = 1u64;
    base %= m;
    while exp > 0 {
        if exp & 1 == 1 { result = (result as u128 * base as u128 % m as u128) as u64; }
        base = (base as u128 * base as u128 % m as u128) as u64;
        exp >>= 1;
    }
    result
}

fn mod_inv(a: u64, m: u64) -> u64 {
    let (mut t, mut new_t) = (0i64, 1i64);
    let (mut r, mut new_r) = (m as i64, (a % m) as i64);
    while new_r != 0 {
        let q = r / new_r;
        let tmp = new_t; new_t = t - q * new_t; t = tmp;
        let tmp = new_r; new_r = r - q * new_r; r = tmp;
    }
    ((t % m as i64) + m as i64) as u64 % m
}

fn main() {
    let mut f = vec![0u64; 1 << 10];

    for subset in 1..(1u32 << 10) {
        let n = subset.count_ones() as u64;
        let has_zero = subset & 1 != 0;
        for d in 1..=N_VAL {
            f[subset as usize] = (f[subset as usize] + pow_mod(n, d, M_VAL)) % M_VAL;
            if has_zero {
                f[subset as usize] = (f[subset as usize] + M_VAL - pow_mod(n, d - 1, M_VAL)) % M_VAL;
            }
        }
        let mut ss = (subset.wrapping_sub(1)) & subset;
        while ss > 0 {
            f[subset as usize] = (f[subset as usize] + M_VAL - f[ss as usize]) % M_VAL;
            ss = (ss.wrapping_sub(1)) & subset;
        }
    }

    let mut ans = 0u64;
    for s1 in 1..(1u32 << 10) {
        for s2 in 1..(1u32 << 10) {
            if s1 & s2 > 0 {
                ans = (ans + (f[s1 as usize] as u128 * f[s2 as usize] as u128 % M_VAL as u128) as u64) % M_VAL;
            }
        }
    }

    let total = (pow_mod(10, N_VAL, M_VAL) + M_VAL - 1) % M_VAL;
    ans = (ans + M_VAL - total) % M_VAL;
    ans = (ans as u128 * mod_inv(2, M_VAL) as u128 % M_VAL as u128) as u64;

    println!("{}", ans);
}
