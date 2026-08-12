// Problem 992: Another Frog Jumping
// Combinatorial journey counts with precomputed factorials mod 987898789.

use euler_utils::BinomialMod;

const MOD: u64 = 987_898_789;

fn endpoint_count(n: usize, k: usize, end: usize, comb: &BinomialMod) -> u64 {
    if n == 0 {
        return 1;
    }

    let mut right = vec![0i64; n];
    right[0] = k as i64 - if end == 0 { 1 } else { 0 };
    if n >= 2 {
        right[1] = 2 - if end == 1 { 1 } else { 0 };
    }
    for i in 2..n {
        right[i] = 1 + right[i - 2] - if end == i { 1 } else { 0 };
    }

    let mut ways = 1u64;
    for v in 1..n {
        let out_degree = (k + v - if end == v { 1 } else { 0 }) as i64;
        let rv = right[v];
        let c = if v < end {
            // Last departure from v must be to the right.
            if out_degree < 1 || rv < 1 {
                0
            } else {
                comb.choose((out_degree - 1) as usize, (rv - 1) as usize)
            }
        } else if v == end {
            if out_degree < 0 || rv < 0 {
                0
            } else {
                comb.choose(out_degree as usize, rv as usize)
            }
        } else {
            // Last departure from v must be to the left.
            if out_degree < 1 || rv < 0 {
                0
            } else {
                comb.choose((out_degree - 1) as usize, rv as usize)
            }
        };
        ways = ways * c % MOD;
    }
    ways
}

fn journey_count(n: usize, k: usize, comb: &BinomialMod) -> u64 {
    let mut total = 0u64;
    for end in 0..=n {
        total = (total + endpoint_count(n, k, end, comb)) % MOD;
    }
    total
}

fn solve() -> u64 {
    let n = 500usize;
    let ks = [1usize, 10, 100, 1000, 10000];
    let comb = BinomialMod::new(ks.iter().copied().max().unwrap() + n, MOD);

    debug_assert_eq!(journey_count(3, 2, &comb), 17);
    debug_assert_eq!(journey_count(6, 1, &comb), 1320);
    debug_assert_eq!(journey_count(6, 5, &comb), 16_793_280);

    let mut answer = 0u64;
    for &k in &ks {
        answer = (answer + journey_count(n, k, &comb)) % MOD;
    }
    answer
}

fn main() {
    println!("{}", solve());
}
