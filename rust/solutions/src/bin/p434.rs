// Project Euler 434: Rigid graphs

const NMAX: usize = 100;
const M: u64 = 1_000_000_033;

fn main() {
    // Precompute binomial coefficients
    let mut ncr = [[0u64; NMAX + 1]; NMAX + 1];
    for i in 0..=NMAX {
        ncr[i][0] = 1;
        for j in 1..=i {
            ncr[i][j] = (ncr[i - 1][j - 1] + ncr[i - 1][j]) % M;
        }
    }

    // Precompute powers of 2
    let max_pow = NMAX * NMAX;
    let mut pow2s = vec![0u64; max_pow + 1];
    pow2s[0] = 1;
    for i in 1..=max_pow {
        pow2s[i] = pow2s[i - 1] * 2 % M;
    }

    // Compute R[m][n]
    let mut r = vec![vec![0u64; NMAX + 1]; NMAX + 1];

    for m in 1..=NMAX {
        for n in 0..=NMAX {
            r[m][n] = pow2s[m * n];
            for a in 1..=m {
                for b in 0..=n {
                    if a < m || b < n {
                        let sub = ncr[m - 1][a - 1] as u128
                            * ncr[n][b] as u128 % M as u128
                            * r[a][b] as u128 % M as u128
                            * pow2s[(m - a) * (n - b)] as u128 % M as u128;
                        r[m][n] = (r[m][n] + M - (sub % M as u128) as u64) % M;
                    }
                }
            }
        }
    }

    let mut ans: u64 = 0;
    for m in 1..=NMAX {
        for n in 1..=NMAX {
            ans = (ans + r[m][n]) % M;
        }
    }

    ans = (ans + M) % M;
    println!("{}", ans);
}
