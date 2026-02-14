// Project Euler 227: The Chase
fn main() {
    const N: usize = 100;
    const K: f64 = 6.0;
    const L: usize = 100_000;

    let k2 = K * K;
    let mut table = [0.0f64; N / 2 + 1];
    table[N / 2] = 1.0;
    let mut ans = 0.0;

    for rounds in 0..L {
        ans += rounds as f64 * table[0];
        let mut new_table = [0.0f64; N / 2 + 1];

        for dist in 1..=N / 2 {
            let t = table[dist];
            if t == 0.0 { continue; }

            // Both towards
            if dist == 1 {
                new_table[dist] += t / k2;
            } else {
                new_table[dist - 2] += t / k2;
            }

            // One towards
            new_table[dist - 1] += t * 2.0 * (K - 2.0) / k2;

            // Stay same
            new_table[dist] += t * (2.0 + (K - 2.0) * (K - 2.0)) / k2;

            // One away
            if dist == N / 2 {
                new_table[dist - 1] += t * 2.0 * (K - 2.0) / k2;
            } else {
                new_table[dist + 1] += t * 2.0 * (K - 2.0) / k2;
            }

            // Both away
            if dist == N / 2 {
                new_table[dist - 2] += t / k2;
            } else if dist == N / 2 - 1 {
                new_table[dist] += t / k2;
            } else {
                new_table[dist + 2] += t / k2;
            }
        }

        table = new_table;
    }

    println!("{:.6}", ans);
}
