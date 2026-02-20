// Project Euler 677 - Coloured Graphs
// DP for counting colored trees with degree/color constraints. N=10000, K=4.

const MOD: i64 = 1_000_000_007;
const N: usize = 10_000;
const K: usize = 4;

fn mod_inv(a: i64) -> i64 {
    let mut result = 1i64; let mut exp = MOD - 2; let mut base = ((a % MOD) + MOD) % MOD;
    while exp > 0 {
        if exp & 1 == 1 { result = result * base % MOD; }
        base = base * base % MOD;
        exp >>= 1;
    }
    result
}

fn main() {
    let mut fact = [0i64; K + 1]; fact[0] = 1;
    for i in 1..=K { fact[i] = fact[i-1] * i as i64 % MOD; }
    let mut inv_fact = [0i64; K + 1];
    for i in 0..=K { inv_fact[i] = mod_inv(fact[i]); }
    let mut ncr_table = [[0i64; K + 1]; K + 1];
    for nn in 0..=K { ncr_table[nn][0] = 1; for rr in 1..=nn { ncr_table[nn][rr] = (ncr_table[nn-1][rr-1] + ncr_table[nn-1][rr]) % MOD; } }

    // Allocate arrays: f[nc][yr][size], g[nc][yr][size]
    let mut f = vec![vec![vec![0i64; N + 1]; 2]; K + 1];
    let mut g = vec![vec![vec![0i64; N + 1]; 2]; K + 1];
    let mut h_r = vec![0i64; N + 1];
    let mut h_b = vec![0i64; N + 1];
    let mut h_y = vec![0i64; N + 1];
    let mut h_all = vec![vec![0i64; N + 1]; 2];

    for size in 0..=N {
        for yr in 0..2 {
            for nc in 0..=K {
                if nc == K && size != N { continue; }
                if nc == 0 {
                    f[nc][yr][size] = if size == 1 { 1 } else { 0 };
                } else {
                    let mut count = 0i64;
                    for cs in 1..size {
                        if 2 * cs <= N {
                            count = (count + h_all[yr][cs] * f[nc-1][yr][size - cs] % MOD) % MOD;
                        }
                    }
                    f[nc][yr][size] = count;
                }
                if size > N / 2 && size != N { continue; }
                let mut count = f[nc][yr][size];
                for kk in 2..=nc {
                    let multiplier = fact[kk-1] * ncr_table[nc][kk] % MOD;
                    for cs in 1..size {
                        if (kk * cs) < size && 2 * cs <= N {
                            count = (count + multiplier * h_all[yr][cs] % MOD * f[nc - kk][yr][size - kk * cs] % MOD) % MOD;
                        }
                    }
                }
                g[nc][yr][size] = count * inv_fact[nc] % MOD;
            }
        }
        h_r[size] = (0..K).map(|nc| g[nc][0][size]).sum::<i64>() % MOD;
        h_b[size] = (0..K-1).map(|nc| g[nc][0][size]).sum::<i64>() % MOD;
        h_y[size] = (0..K-1).map(|nc| g[nc][1][size]).sum::<i64>() % MOD;
        h_all[0][size] = (h_r[size] + h_b[size] + h_y[size]) % MOD;
        h_all[1][size] = (h_r[size] + h_b[size]) % MOD;
    }

    let mut ans = 0i64;
    for nc in 0..=K { ans = (ans + g[nc][0][N]) % MOD; }
    for nc in 0..K { ans = (ans + g[nc][0][N]) % MOD; }
    for nc in 0..K { ans = (ans + g[nc][1][N]) % MOD; }
    if N % 2 == 0 {
        ans = ans * 2 % MOD;
        let h_arr = [h_r[N/2], h_b[N/2], h_y[N/2]];
        for i in 0..3 { for j in 0..3 {
            if i != 2 || j != 2 { ans = (ans - h_arr[i] * h_arr[j] % MOD + MOD) % MOD; }
        } }
        ans = (ans + h_all[1][N/2]) % MOD;
        ans = ans * mod_inv(2) % MOD;
    }
    println!("{}", (ans % MOD + MOD) % MOD);
}
