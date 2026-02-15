// Project Euler 882
// Compute G[i] values using bit manipulation and floor-based search.

fn main() {
    let n = 100_000usize;
    let mut g = vec![0.0f64; n + 1];
    let mut total = 0.0f64;

    for i in 1..=n {
        let mut low = 0.0f64;
        let mut high = f64::MAX;
        let mut j = 0u32;
        while (1u64 << j) <= i as u64 {
            let remaining = (i >> (j + 1) << j) + i % (1 << j);
            if (i & (1 << j)) > 0 {
                if g[remaining] > low { low = g[remaining]; }
            } else {
                if g[remaining] < high { high = g[remaining]; }
            }
            j += 1;
        }
        let mut d = 1.0f64;
        g[i] = 0.0;
        while g[i] <= low || g[i] >= high {
            g[i] = (low / d + 1.0).floor() * d;
            d /= 2.0;
        }
        total += i as f64 * g[i];
    }

    println!("{}", total.ceil() as i64);
}
