// Problem 976: XO Game
// Ported from python/976.py.

const MOD: u64 = 1_234_567_891;

fn build_inverses(n: usize) -> Vec<u32> {
    let mut inv = vec![0u32; n + 1];
    if n >= 1 {
        inv[1] = 1;
    }
    for i in 2..=n {
        let val = MOD - (MOD / i as u64) * inv[(MOD % i as u64) as usize] as u64 % MOD;
        inv[i] = val as u32;
    }
    inv
}

fn solve() -> u64 {
    let n: usize = 10_000_000;
    let k: usize = 10_000_000;

    let e = n / 2;
    let a_cnt = (n + 3) / 4;
    let b_cnt = (n + 1) / 4;
    let c = b_cnt as i64 - a_cnt as i64;

    if e == 0 {
        let inv = build_inverses(k + 2);
        let inv2 = (MOD + 1) / 2;
        let mut h = 1u64;
        let mut q = 1u64;
        let mut sum_odd_a = 0u64;
        let mut ans = 0u64;

        for s in 0..=k {
            if s > 0 {
                h = ((h as u128 * (a_cnt + b_cnt + s - 1) as u128) % MOD as u128) as u64;
                h = ((h as u128 * inv[s] as u128) % MOD as u128) as u64;
                if s % 2 == 0 {
                    let r = s / 2;
                    q = ((q as u128 * (a_cnt + r - 1) as u128) % MOD as u128) as u64;
                    q = ((q as u128 * inv[r] as u128) % MOD as u128) as u64;
                }
            }

            let coeff = if c == 0 {
                if s % 2 == 0 {
                    q
                } else {
                    0
                }
            } else if c == 1 {
                q
            } else if s % 2 == 0 {
                q
            } else {
                (MOD - q) % MOD
            };

            let h_odd_a = ((h + MOD - coeff) % MOD * inv2) % MOD;
            sum_odd_a += h_odd_a;
            if sum_odd_a >= MOD {
                sum_odd_a -= MOD;
            }
            ans = sum_odd_a;
        }

        return ans % MOD;
    }

    let max_inv = e + k + 2;
    let inv = build_inverses(max_inv);
    let inv2 = (MOD + 1) / 2;

    let mut total_even = 1u64;
    for m in 0..k {
        total_even = ((total_even as u128 * (e + m) as u128) % MOD as u128) as u64;
        total_even = ((total_even as u128 * inv[m + 1] as u128) % MOD as u128) as u64;
    }

    let qmax = k / 2;
    let mut e0 = 1u64;
    for q in 0..qmax {
        e0 = ((e0 as u128 * (e + q) as u128) % MOD as u128) as u64;
        e0 = ((e0 as u128 * inv[q + 1] as u128) % MOD as u128) as u64;
    }

    let mut h = 1u64;
    let mut q = 1u64;
    let mut qsum = 1u64;
    let mut sum_even = 0u64;
    let mut sum_odd = 0u64;
    let mut sum_odd_a = 0u64;
    let mut ans = 0u64;
    let ab = a_cnt + b_cnt;

    for s in 0..=k {
        if s > 0 {
            h = ((h as u128 * (ab + s - 1) as u128) % MOD as u128) as u64;
            h = ((h as u128 * inv[s] as u128) % MOD as u128) as u64;
            if s % 2 == 0 {
                let r = s / 2;
                q = ((q as u128 * (a_cnt + r - 1) as u128) % MOD as u128) as u64;
                q = ((q as u128 * inv[r] as u128) % MOD as u128) as u64;
                if c == 1 {
                    qsum = ((qsum as u128 * (a_cnt + r) as u128) % MOD as u128) as u64;
                    qsum = ((qsum as u128 * inv[r] as u128) % MOD as u128) as u64;
                }
            }
        }

        let coeff = if c == 0 {
            if s % 2 == 0 {
                q
            } else {
                0
            }
        } else if c == 1 {
            qsum
        } else if s % 2 == 0 {
            q
        } else {
            (MOD - q) % MOD
        };

        let h_odd_a = ((h + MOD - coeff) % MOD * inv2) % MOD;

        if s % 2 == 0 {
            sum_even += h;
            if sum_even >= MOD {
                sum_even -= MOD;
            }
        } else {
            sum_odd += h;
            if sum_odd >= MOD {
                sum_odd -= MOD;
            }
        }
        sum_odd_a += h_odd_a;
        if sum_odd_a >= MOD {
            sum_odd_a -= MOD;
        }

        let m = k - s;
        let (t0, t1) = if m % 2 == 0 {
            let e0_m = e0;
            let t0 = ((e0_m as u128 * sum_odd_a as u128) % MOD as u128) as u64;
            let t1 =
                (((total_even + MOD - e0_m) % MOD) as u128 * sum_even as u128 % MOD as u128) as u64;
            (t0, t1)
        } else {
            let t1 = ((total_even as u128 * sum_odd as u128) % MOD as u128) as u64;
            (0, t1)
        };
        ans += t0;
        if ans >= MOD {
            ans -= MOD;
        }
        ans += t1;
        if ans >= MOD {
            ans -= MOD;
        }

        if m > 0 {
            total_even = ((total_even as u128 * m as u128) % MOD as u128) as u64;
            total_even = ((total_even as u128 * inv[e + m - 1] as u128) % MOD as u128) as u64;
        }
        if m % 2 == 0 && m >= 2 {
            let qcur = m / 2;
            e0 = ((e0 as u128 * qcur as u128) % MOD as u128) as u64;
            e0 = ((e0 as u128 * inv[e + qcur - 1] as u128) % MOD as u128) as u64;
        }
    }

    ans % MOD
}

fn main() {
    println!("{}", solve());
}
