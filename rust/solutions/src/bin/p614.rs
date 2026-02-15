// Project Euler 614 - Special partitions II
// Paged DP with pentagonal-like recurrence

const N: usize = 10_000_000;
const M: i64 = 1_000_000_007;
const L: usize = 1 << 14;

fn isqrt(n: i64) -> i64 {
    let mut x = (n as f64).sqrt() as i64;
    while x * x > n { x -= 1; }
    while (x + 1) * (x + 1) <= n { x += 1; }
    x
}

fn is_sq(n: i64) -> bool {
    let r = isqrt(n);
    r * r == n
}

fn tr(t: i64) -> i64 {
    t * (t + 1) / 2
}

fn parity(n: i64) -> i64 {
    if n % 2 == 0 { 1 } else { -1 }
}

fn main() {
    let mut p = vec![0i64; N + L];

    let num_pages = (N + L - 1) / L;
    for page in 0..num_pages {
        // Process previous pages
        for prev_page in 0..page {
            let min_t = (((8.0 * (page - (prev_page + 1)) as f64 * L as f64 + 1.0).sqrt() + 1.0) / 2.0) as i64;
            let mut t = min_t;
            while tr(t) < ((page + 1 - prev_page) * L) as i64 {
                let tr_t = tr(t) as usize;
                let start = std::cmp::max(page * L, prev_page * L + tr_t);
                let end = std::cmp::min((page + 1) * L, N + 1);
                for i in start..end {
                    if i < tr_t { continue; }
                    if i - tr_t >= (prev_page + 1) * L { continue; }
                    p[i] = (p[i] + parity((t - 1) / 2) * p[i - tr_t] % M + M) % M;
                }
                t += 1;
            }
        }

        // Process current page
        let start = page * L;
        let end = std::cmp::min((page + 1) * L, N + 1);
        for i in start..end {
            let res = 4 * i as i64 + 1;
            if is_sq(res) {
                let root = isqrt(res);
                p[i] = (p[i] + parity((root / 2 + 1) / 2) + M) % M;
            }
            let mut t = 1i64;
            while tr(t) <= (i - page * L) as i64 {
                let tr_t = tr(t) as usize;
                p[i] = (p[i] + parity((t - 1) / 2) * p[i - tr_t] % M + M) % M;
                t += 1;
            }
            p[i] = (p[i] % M + M) % M;
        }
    }

    let mut ans = 0i64;
    for i in 1..=N {
        ans = (ans + p[i]) % M;
    }

    println!("{}", ans);
}
