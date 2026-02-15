// Project Euler 796 - Expected Cards Drawn
// Inclusion-exclusion over ranks, suits, jokers, and decks.

fn ncr(n: i32, k: i32) -> f64 {
    if k < 0 || k > n { return 0.0; }
    let k = k.min(n - k) as usize;
    let mut result = 1.0f64;
    for i in 0..k {
        result = result * (n as f64 - i as f64) / (i as f64 + 1.0);
    }
    result
}

fn main() {
    let r = 13;
    let s = 4;
    let j = 2;
    let d = 10;
    let l = (r * s + j) * d;

    let mut ans = 1.0f64;
    for n in 1..=l {
        for ri in 0..=r {
            for si in 0..=s {
                for di in 0..=d {
                    if ri + si + di > 0 {
                        let mut res = 1.0f64;
                        let limit = ((r - ri) * (s - si) + j) * (d - di);
                        for k in (limit + 1..=l).rev() {
                            res *= (k - n) as f64 / k as f64;
                        }
                        let sign: f64 = if (ri + si + di) % 2 == 0 { 1.0 } else { -1.0 };
                        ans -= sign * ncr(r, ri) * ncr(s, si) * ncr(d, di) * res;
                    }
                }
            }
        }
    }
    println!("{:.8}", ans);
}
