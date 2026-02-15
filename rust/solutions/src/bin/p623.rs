// Project Euler 623 - Lambda Terms
// DP: T[c][b] = count of closed lambda terms with c chars and b bound vars

const NN: usize = 2000;
const MOD: i64 = 1_000_000_007;
const MAXB: usize = 401;

fn main() {
    // Use flat array: T[c * MAXB + b]
    let mut t = vec![0i64; (NN + 1) * MAXB];

    for b in (0..MAXB).rev() {
        t[1 * MAXB + b] = b as i64 % MOD;

        for c in 2..=NN {
            let mut val = 0i64;

            // Abstraction
            if c >= 6 && b + 1 < MAXB {
                val = t[(c - 5) * MAXB + b + 1];
            }

            // Application
            if c >= 4 {
                let rem = c - 2;
                let mut conv = 0i64;
                for l in 1..rem {
                    conv = (conv + t[l * MAXB + b] % MOD * (t[(rem - l) * MAXB + b] % MOD)) % MOD;
                }
                val = (val + conv) % MOD;
            }

            t[c * MAXB + b] = val;
        }
    }

    let mut ans = 0i64;
    for c in 1..=NN {
        ans = (ans + t[c * MAXB]) % MOD;
    }
    println!("{}", ans);
}
