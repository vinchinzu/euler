// Project Euler 862 - Sum of T(n) for k-digit numbers
// S(12) via digit frequency enumeration

fn main() {
    let k = 12;
    let mut fact = [0i64; 13];
    fact[0] = 1;
    for i in 1..=k {
        fact[i] = fact[i - 1] * i as i64;
    }

    let mut total: i64 = 0;
    let mut counts = [0usize; 10];

    fn dfs(pos: usize, rem: usize, k: usize, counts: &mut [usize; 10], fact: &[i64; 13], total: &mut i64) {
        if pos == 9 {
            counts[9] = rem;

            let mut denom: i64 = 1;
            for i in 0..10 {
                denom *= fact[counts[i]];
            }

            let total_perms = fact[k] / denom;

            let c0 = counts[0];
            let num_valid = if c0 == 0 {
                total_perms
            } else {
                let denom2 = denom / c0 as i64;
                let zero_first = fact[k - 1] / denom2;
                total_perms - zero_first
            };

            if num_valid > 1 {
                *total += num_valid * (num_valid - 1) / 2;
            }
            return;
        }

        for v in 0..=rem {
            counts[pos] = v;
            dfs(pos + 1, rem - v, k, counts, fact, total);
        }
    }

    dfs(0, k, k, &mut counts, &fact, &mut total);
    println!("{}", total);
}
