// Project Euler 786 - Billiard Ball Bounces
// Mobius function sieve and lattice point counting.

fn main() {
    const BIG_N: i64 = 1_000_000_000;
    let l = (3 * BIG_N + 5) / 2;
    let g_limit = (l / 5) as usize;

    // Compute Mobius function
    let mut mobius = vec![1i8; g_limit + 1];
    let mut is_prime = vec![true; g_limit + 1];
    is_prime[0] = false;
    if g_limit >= 1 { is_prime[1] = false; }

    for i in 2..=g_limit {
        if is_prime[i] {
            for j in (i..=g_limit).step_by(i) {
                if j > i { is_prime[j] = false; }
                if (j / i) % i == 0 {
                    mobius[j] = 0;
                } else {
                    mobius[j] = -mobius[j];
                }
            }
        }
    }

    let mut ans: i64 = 0;

    for g in 1..=g_limit {
        if mobius[g] == 0 { continue; }
        let t = l / g as i64;
        let d: i64 = if g % 3 == 0 { 3 } else { 9 };

        // Count lattice points: sum_{x=1}^{t/5} floor((t - 5x) / d)
        let x_max = t / 5;
        let mut count: i64 = 0;
        for x in 1..=x_max {
            let remainder = t - 5 * x;
            if remainder >= d {
                count += remainder / d;
            }
        }

        ans += mobius[g] as i64 * count;
    }

    ans *= 4;
    ans += 2;

    println!("{}", ans);
}
