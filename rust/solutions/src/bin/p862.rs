// Project Euler 862 - Sum of T(n) for k-digit numbers
// S(12) via digit frequency enumeration

const FACT: [i64; 13] = [
    1, 1, 2, 6, 24, 120, 720, 5040, 40320, 362880, 3628800, 39916800, 479001600,
];

fn dfs(pos: usize, rem: usize, k: usize, counts: &mut [usize; 10], denom: i64, total: &mut i64) {
    if pos == 9 {
        counts[9] = rem;
        let denom = denom * FACT[rem];

        let total_perms = FACT[k] / denom;

        let c0 = counts[0];
        let num_valid = if c0 == 0 {
            total_perms
        } else {
            let denom2 = denom / c0 as i64;
            let zero_first = FACT[k - 1] / denom2;
            total_perms - zero_first
        };

        if num_valid > 1 {
            *total += num_valid * (num_valid - 1) / 2;
        }
        return;
    }

    for v in 0..=rem {
        counts[pos] = v;
        dfs(pos + 1, rem - v, k, counts, denom * FACT[v], total);
    }
}

fn main() {
    let k = 12;
    let mut total: i64 = 0;
    let mut counts = [0usize; 10];

    dfs(0, k, k, &mut counts, 1, &mut total);
    println!("{}", total);
}
