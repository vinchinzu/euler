// Project Euler 475: Music festival
use euler_utils::mod_pow;

const NN: usize = 600;
const KK: usize = 4;
const MOD: u64 = 1_000_000_007;

fn mulmod(a: u64, b: u64) -> u64 {
    (a as u128 * b as u128 % MOD as u128) as u64
}

fn main() {
    let limit = KK * NN;
    let mut fact = vec![0u64; limit + 1];
    let mut inv_fact = vec![0u64; limit + 1];
    fact[0] = 1;
    for i in 1..=limit {
        fact[i] = mulmod(fact[i - 1], i as u64);
    }
    inv_fact[limit] = mod_pow(fact[limit], MOD - 2, MOD);
    for i in (0..limit).rev() {
        inv_fact[i] = mulmod(inv_fact[i + 1], (i + 1) as u64);
    }

    let ncr = |n: usize, r: usize| -> u64 {
        if r > n { return 0; }
        mulmod(mulmod(fact[n], inv_fact[r]), inv_fact[n - r])
    };

    const MAX_M: usize = 201;
    let mut cache = vec![vec![vec![0u64; MAX_M]; MAX_M]; MAX_M];
    let mut cached = vec![vec![vec![false; MAX_M]; MAX_M]; MAX_M];

    fn f(m1: usize, m2: usize, m3: usize,
         cache: &mut Vec<Vec<Vec<u64>>>, cached: &mut Vec<Vec<Vec<bool>>>,
         ncr: &dyn Fn(usize, usize) -> u64) -> u64 {
        if m1 + m2 + m3 == 0 { return 1; }
        if cached[m1][m2][m3] { return cache[m1][m2][m3]; }
        cached[m1][m2][m3] = true;

        let mut result: u64 = 0;
        for d1 in 0..=m1.min(KK) {
            for d2 in 0..=m2.min(KK - d1) {
                let d3 = KK - d1 - d2;
                if d3 > m3 { continue; }
                let mut ways = ncr(m1, d1);
                ways = mulmod(ways, mod_pow(2, d2 as u64, MOD));
                ways = mulmod(ways, ncr(m2, d2));
                ways = mulmod(ways, mod_pow(3, d3 as u64, MOD));
                ways = mulmod(ways, ncr(m3, d3));
                let sub = f(m1 - d1 + d2, m2 - d2 + d3, m3 - d3, cache, cached, ncr);
                result = (result + mulmod(ways, sub)) % MOD;
            }
        }
        cache[m1][m2][m3] = result;
        result
    }

    let mut ans = f(0, 0, NN / 3, &mut cache, &mut cached, &ncr);

    // Multiply by (K!)^(N/K)
    ans = mulmod(ans, mod_pow(fact[KK], (NN / KK) as u64, MOD));
    // Multiply by (1/3!)^(N/3)
    ans = mulmod(ans, mod_pow(inv_fact[3], (NN / 3) as u64, MOD));
    // Multiply by 1/(N/3)!
    ans = mulmod(ans, inv_fact[NN / 3]);

    println!("{ans}");
}
