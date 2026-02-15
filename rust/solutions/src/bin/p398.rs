const NN: usize = 10_000_000;
const K: usize = 100;

fn main() {
    let array_size = NN + 10_000;
    let mut f = vec![0.0f64; array_size];

    for i in 0..=NN {
        if i % 64 == 0 {
            f[i] = 1.0;
            for j in 0..(K - 1) {
                f[i] *= (NN - i + K - 1 - j) as f64 / (NN - 1 - j) as f64;
            }
        } else {
            f[i] = f[i - 1] * (NN - i) as f64 / (NN - i + K - 1) as f64;
        }
    }

    let mut ans = 0.0f64;

    for b in 1..NN {
        if b * (K - 1) >= NN { break; }

        let idx1 = b * K;
        let idx2 = b + (b + 1) * (K - 1);
        let idx3 = (b + 1) * K;

        if idx1 >= array_size || idx2 >= array_size || idx3 >= array_size { break; }

        let mut prob = f[idx1] - K as f64 * f[idx2] + (K - 1) as f64 * f[idx3];

        for a in 1..b {
            let i1 = a + b * (K - 1);
            let i2 = a + (b + 1) * (K - 1);
            let i3 = (a + 1) + b * (K - 1);
            let i4 = (a + 1) + (b + 1) * (K - 1);

            if i1 >= array_size || i2 >= array_size || i3 >= array_size || i4 >= array_size { break; }

            let sub_prob = K as f64 * (f[i1] - f[i2] - f[i3] + f[i4]);
            if (b as f64 * sub_prob).abs() < 1e-15 { break; }
            prob += sub_prob;
        }

        ans += b as f64 * prob;
    }

    println!("{:.5}", ans);
}
