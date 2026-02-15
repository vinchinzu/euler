// Project Euler 627 - Counting Products
// Ehrhart polynomial: brute-force small dilations, Lagrange interpolation

use std::collections::HashSet;

const MOD: i64 = 1_000_000_007;
const L_PRIMES: usize = 10;
const K_VAL: usize = 30;
const N_VAL: i64 = 10001;

fn powmod(mut base: i64, mut exp: i64, m: i64) -> i64 {
    let mut result = 1i64;
    base = ((base % m) + m) % m;
    while exp > 0 {
        if exp & 1 == 1 { result = (result as i128 * base as i128 % m as i128) as i64; }
        base = (base as i128 * base as i128 % m as i128) as i64;
        exp >>= 1;
    }
    result
}

static PLIST: [usize; 10] = [2, 3, 5, 7, 11, 13, 17, 19, 23, 29];

fn factor30(mut n: usize) -> [u8; 10] {
    let mut exp = [0u8; 10];
    for i in 0..10 {
        while n % PLIST[i] == 0 { exp[i] += 1; n /= PLIST[i]; }
    }
    exp
}

fn encode(exp: &[u8; 10]) -> u64 {
    let mut key = 0u64;
    for i in (0..10).rev() {
        key = (key << 5) | (exp[i] as u64 & 0x1f);
    }
    key
}

fn decode(mut key: u64) -> [u8; 10] {
    let mut exp = [0u8; 10];
    for i in 0..10 {
        exp[i] = (key & 0x1f) as u8;
        key >>= 5;
    }
    exp
}

fn main() {
    let d = L_PRIMES - 3; // floor(sqrt(10))=3, D=7
    let num_bf = L_PRIMES + 1 - d; // 4

    let fvecs: Vec<[u8; 10]> = (0..=K_VAL).map(|i| if i == 0 { [0u8; 10] } else { factor30(i) }).collect();

    let mut f = vec![0i64; num_bf];
    f[0] = 1;

    let mut cur: HashSet<u64> = HashSet::new();
    cur.insert(encode(&[0u8; 10]));

    for n in 1..num_bf {
        let mut next: HashSet<u64> = HashSet::new();
        let keys: Vec<u64> = cur.iter().copied().collect();
        for &base_key in &keys {
            let base_exp = decode(base_key);
            for fv in 1..=K_VAL {
                let mut new_exp = [0u8; 10];
                for i in 0..10 { new_exp[i] = base_exp[i] + fvecs[fv][i]; }
                next.insert(encode(&new_exp));
            }
        }
        cur = next;
        f[n] = cur.len() as i64;
    }

    let n_pts = L_PRIMES + 1; // 11
    let mut vals = vec![0i64; n_pts];
    for i in 0..n_pts {
        if i >= d { vals[i] = f[i - d] % MOD; }
    }

    let x = N_VAL + d as i64;

    let mut prefix = vec![1i64; n_pts + 1];
    for j in 0..n_pts {
        prefix[j + 1] = (prefix[j] as i128 * ((x - j as i64) % MOD + MOD) as i128 % MOD as i128) as i64;
    }
    let mut suffix = vec![1i64; n_pts + 1];
    for j in (0..n_pts).rev() {
        suffix[j] = (suffix[j + 1] as i128 * ((x - j as i64) % MOD + MOD) as i128 % MOD as i128) as i64;
    }

    let mut fact = vec![1i64; n_pts];
    for i in 1..n_pts { fact[i] = fact[i - 1] * i as i64 % MOD; }
    let mut inv_fact = vec![1i64; n_pts];
    inv_fact[n_pts - 1] = powmod(fact[n_pts - 1], MOD - 2, MOD);
    for i in (0..n_pts - 1).rev() { inv_fact[i] = inv_fact[i + 1] * (i + 1) as i64 % MOD; }

    let mut ans = 0i64;
    for i in 0..n_pts {
        if vals[i] == 0 { continue; }
        let numer = (prefix[i] as i128 * suffix[i + 1] as i128 % MOD as i128) as i64;
        let mut denom_inv = (inv_fact[i] as i128 * inv_fact[n_pts - 1 - i] as i128 % MOD as i128) as i64;
        if (n_pts - 1 - i) % 2 == 1 { denom_inv = (MOD - denom_inv) % MOD; }
        ans = (ans + (vals[i] as i128 * numer as i128 % MOD as i128 * denom_inv as i128 % MOD as i128) as i64) % MOD;
    }

    println!("{}", ans % MOD);
}
