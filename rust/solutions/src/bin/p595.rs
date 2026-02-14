// Project Euler 595 - Incremental Random Sort
//
// Find the expected number of shuffles to sort 1..52.
// Uses inclusion-exclusion to compute g(k).

const MAXN: usize = 53;

fn main() {
    let n = 52;

    // Precompute binomial coefficients
    let mut comb = [[0.0f64; MAXN]; MAXN];
    for nn in 0..MAXN {
        comb[nn][0] = 1.0;
        for k in 1..=nn {
            comb[nn][k] = comb[nn - 1][k - 1] + comb[nn - 1][k];
        }
    }

    // Precompute factorials
    let mut fact = [0.0f64; MAXN];
    fact[0] = 1.0;
    for i in 1..MAXN {
        fact[i] = fact[i - 1] * i as f64;
    }

    let mut f = [0.0f64; MAXN];

    for nn in 2..=n {
        let mut g = [0.0f64; MAXN];

        for k in 1..=nn {
            g[k] = comb[nn - 1][k - 1] * fact[k];
            for i in 1..k {
                g[k] -= comb[nn - i][k - i] * g[i];
            }
        }

        f[nn] = g[nn];
        for j in 2..nn {
            f[nn] += g[j] * (1.0 + f[j]);
        }
        f[nn] /= fact[nn] - g[nn];
    }

    println!("{:.8}", f[n]);
}
