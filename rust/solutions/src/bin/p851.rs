// Project Euler 851 - R_6(10000!) mod 10^9+7
// Quasimodular forms: tau, sigma_k, linear system solve

const MOD: i64 = 1_000_000_007;
const BIG_N: usize = 10000;
const K: usize = 6;

fn mod_pow(mut base: i64, mut exp: i64, m: i64) -> i64 {
    let mut r = 1i64;
    base = base.rem_euclid(m);
    while exp > 0 {
        if exp & 1 == 1 { r = r * base % m; }
        base = base * base % m;
        exp >>= 1;
    }
    r
}

fn mod_inv(a: i64) -> i64 { mod_pow(a.rem_euclid(MOD), MOD - 2, MOD) }

fn main() {
    // Sieve primes up to BIG_N
    let mut is_prime = vec![true; BIG_N + 1];
    is_prime[0] = false;
    if BIG_N > 0 { is_prime[1] = false; }
    for i in 2..=((BIG_N as f64).sqrt() as usize) {
        if is_prime[i] {
            let mut j = i * i;
            while j <= BIG_N { is_prime[j] = false; j += i; }
        }
    }
    let primes: Vec<usize> = (2..=BIG_N).filter(|&i| is_prime[i]).collect();

    // Basis terms
    let mut basis: Vec<(usize, usize)> = Vec::new(); // (k, l)
    let mut k_val = 1;
    while k_val < 2 * K {
        let mut l = 0;
        while k_val + 2 * l < 2 * K {
            basis.push((k_val, l));
            l += 1;
        }
        k_val += 2;
    }
    let nbasis = basis.len();
    let big_l = 1 + nbasis;

    // Compute tau(n) for n=1..BIG_N via (1-q^n)^24
    let mut a = vec![0i64; BIG_N + 1];
    a[0] = 1;
    for n in 1..=BIG_N {
        for j in (n..=BIG_N).rev() {
            a[j] = (a[j] - a[j - n] + MOD) % MOD;
        }
    }

    // a^24 using binary exponentiation on truncated polynomials
    let poly_mul = |a: &[i64], b: &[i64]| -> Vec<i64> {
        let mut out = vec![0i64; BIG_N + 1];
        for i in 0..=BIG_N {
            if a[i] == 0 { continue; }
            let ai = a[i];
            for j in 0..=BIG_N - i {
                if b[j] != 0 {
                    out[i + j] = (out[i + j] + ai * b[j]) % MOD;
                }
            }
        }
        out
    };

    let mut base_p = a.clone();
    let mut result = vec![0i64; BIG_N + 1];
    result[0] = 1;
    let mut exp = 24;
    while exp > 0 {
        if exp & 1 == 1 {
            result = poly_mul(&result, &base_p);
        }
        if exp > 1 {
            let bp = base_p.clone();
            base_p = poly_mul(&bp, &bp);
        }
        exp >>= 1;
    }

    let mut tau_arr = vec![0i64; BIG_N + 1];
    for n in 1..=BIG_N {
        tau_arr[n] = result[n - 1];
    }

    // Compute R_6(n) for n=1..big_l via convolution
    let mut sigma1 = vec![0i64; big_l + 1];
    for d in 1..=big_l {
        let mut m = d;
        while m <= big_l { sigma1[m] += d as i64; m += d; }
    }

    let mut r1 = vec![0i64; big_l + 1];
    for i in 0..=big_l { r1[i] = (2 * sigma1[i]) % MOD; }

    let mut r_cur = r1.clone();
    for _ in 0..5 {
        let mut new_r = vec![0i64; big_l + 1];
        for i in 1..=big_l {
            if r_cur[i] == 0 { continue; }
            for j in 1..=big_l - i {
                if r1[j] != 0 {
                    new_r[i + j] = (new_r[i + j] + r_cur[i] * r1[j]) % MOD;
                }
            }
        }
        r_cur = new_r;
    }
    let r6_vals = r_cur;

    // Compute sigma_k values for each unique k
    let needed_ks: Vec<usize> = {
        let mut ks: Vec<usize> = basis.iter().map(|&(k, _)| k).collect();
        ks.sort(); ks.dedup(); ks
    };

    let mut sigma_k_vals: Vec<Vec<i64>> = Vec::new();
    for &kv in &needed_ks {
        let mut sk = vec![0i64; big_l + 1];
        for d in 1..=big_l {
            let dk = mod_pow(d as i64, kv as i64, MOD);
            let mut m = d;
            while m <= big_l { sk[m] = (sk[m] + dk) % MOD; m += d; }
        }
        sigma_k_vals.push(sk);
    }

    // Build and solve linear system
    let mut a_mat = vec![vec![0i64; big_l]; big_l];
    let mut b_vec = vec![0i64; big_l];

    for i in 0..big_l {
        let n = i + 1;
        a_mat[i][0] = tau_arr[n];
        for j in 0..nbasis {
            let ki_idx = needed_ks.iter().position(|&k| k == basis[j].0).unwrap();
            a_mat[i][j + 1] = sigma_k_vals[ki_idx][n] * mod_pow(n as i64, basis[j].1 as i64, MOD) % MOD;
        }
        b_vec[i] = r6_vals[n];
    }

    // Gaussian elimination
    let nn = big_l;
    let mut aug = vec![vec![0i64; nn + 1]; nn];
    for i in 0..nn {
        for j in 0..nn { aug[i][j] = a_mat[i][j]; }
        aug[i][nn] = b_vec[i];
    }

    for col in 0..nn {
        let mut pivot = nn;
        for row in col..nn {
            if aug[row][col] % MOD != 0 { pivot = row; break; }
        }
        if pivot != col {
            aug.swap(col, pivot);
        }
        let inv = mod_inv(aug[col][col]);
        for j in col..=nn { aug[col][j] = aug[col][j] * inv % MOD; }
        for r in 0..nn {
            if r == col { continue; }
            let factor = aug[r][col] % MOD;
            if factor == 0 { continue; }
            for j in col..=nn {
                aug[r][j] = (aug[r][j] - factor * aug[col][j] % MOD + MOD) % MOD;
            }
        }
    }

    let x_sol: Vec<i64> = (0..nn).map(|i| aug[i][nn] % MOD).collect();

    // Evaluate at n = 10000!
    let mut fac_exp = vec![0i32; BIG_N + 1];
    for &p in &primes {
        let mut count = 0;
        let mut m = BIG_N;
        while m > 0 { m /= p; count += m as i32; }
        fac_exp[p] = count;
    }

    // tau(N!)
    let mut big_tau = 1i64;
    for &p in &primes {
        let e = fac_exp[p] as usize;
        if e == 0 { continue; }
        let mut tp = vec![0i64; e + 1];
        tp[0] = 1;
        tp[1] = tau_arr[p];
        let p11 = mod_pow(p as i64, 11, MOD);
        for r in 1..e {
            tp[r + 1] = (tp[1] * tp[r] % MOD - p11 * tp[r - 1] % MOD + MOD) % MOD;
        }
        big_tau = big_tau * tp[e] % MOD;
    }

    // sigma_k(N!)
    let mut big_sigmas: Vec<i64> = Vec::new();
    for &kv in &needed_ks {
        let mut val = 1i64;
        for &p in &primes {
            let e = fac_exp[p] as i64;
            if e == 0 { continue; }
            let pk = mod_pow(p as i64, kv as i64, MOD);
            let num = (mod_pow(p as i64, kv as i64 * (e + 1), MOD) - 1 + MOD) % MOD;
            let den = (pk - 1 + MOD) % MOD;
            val = val * (num * mod_inv(den) % MOD) % MOD;
        }
        big_sigmas.push(val);
    }

    // N! mod MOD
    let mut factorial_mod = 1i64;
    for x in 1..=BIG_N as i64 {
        factorial_mod = factorial_mod * x % MOD;
    }

    let max_l = basis.iter().map(|&(_, l)| l).max().unwrap_or(0);
    let mut fac_powers = vec![0i64; max_l + 1];
    fac_powers[0] = 1;
    for l in 1..=max_l {
        fac_powers[l] = fac_powers[l - 1] * factorial_mod % MOD;
    }

    let mut ans = big_tau * x_sol[0] % MOD;
    for i in 0..nbasis {
        let ki_idx = needed_ks.iter().position(|&k| k == basis[i].0).unwrap();
        ans = (ans + big_sigmas[ki_idx] * fac_powers[basis[i].1] % MOD * x_sol[i + 1]) % MOD;
    }
    ans = (ans % MOD + MOD) % MOD;

    println!("{}", ans);
}
