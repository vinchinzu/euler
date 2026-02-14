// Project Euler 465 - Polar polygons
// Lucy DP for summatory Euler totient, modular arithmetic.

const MOD: u64 = 1_000_000_007;

fn pow_mod(mut base: u64, mut exp: u64, m: u64) -> u64 {
    let mut result = 1u64;
    base %= m;
    while exp > 0 {
        if exp & 1 == 1 { result = (result as u128 * base as u128 % m as u128) as u64; }
        base = (base as u128 * base as u128 % m as u128) as u64;
        exp >>= 1;
    }
    result
}

fn main() {
    let n: u64 = {
        let mut v = 1u64;
        for _ in 0..13 { v *= 7; }
        v
    }; // 7^13

    let l = {
        let mut s = (n as f64).sqrt() as u64;
        while s * s > n { s -= 1; }
        while (s + 1) * (s + 1) <= n { s += 1; }
        s
    };

    let cbrt_n = (n as f64).cbrt();
    let mut slimit = (cbrt_n * cbrt_n) as usize + 100;
    let ndl = (n / l + 2) as usize;
    if slimit < ndl { slimit = ndl; }
    if slimit < l as usize + 2 { slimit = l as usize + 2; }

    // Sieve phi
    let mut phi_arr = vec![0u64; slimit + 1];
    for i in 0..=slimit { phi_arr[i] = i as u64; }
    for i in 2..=slimit {
        if phi_arr[i] == i as u64 {
            for j in (i..=slimit).step_by(i) {
                phi_arr[j] = phi_arr[j] / i as u64 * (i as u64 - 1);
            }
        }
    }

    let mut prefix_phi = vec![0i64; slimit + 1];
    for i in 1..=slimit {
        prefix_phi[i] = prefix_phi[i - 1] + phi_arr[i] as i64;
    }

    // Compute sum_phi for both mod M-1 and mod M
    fn compute_sum_phi(
        n: u64, l: u64, slimit: usize, prefix_phi: &[i64], mod_val: u64,
    ) -> (Vec<u64>, Vec<u64>) {
        let l = l as usize;
        let mut small = vec![0u64; l + 2];
        let mut large = vec![0u64; l + 2];

        for v in 0..=l.min(slimit) {
            small[v] = ((prefix_phi[v] % mod_val as i64 + mod_val as i64) % mod_val as i64) as u64;
        }

        for x in (1..=l).rev() {
            let nv = n / x as u64;
            if nv <= l as u64 {
                large[x] = small[nv as usize];
                continue;
            }
            if nv <= slimit as u64 {
                large[x] = ((prefix_phi[nv as usize] % mod_val as i64 + mod_val as i64) % mod_val as i64) as u64;
                continue;
            }

            let (half_n, other) = if nv % 2 == 0 {
                (nv / 2, nv + 1)
            } else {
                ((nv + 1) / 2, nv)
            };
            let mut val = (half_n % mod_val) * (other % mod_val) % mod_val;

            let mut d = 2u64;
            while d <= nv {
                let q = nv / d;
                let d_max = nv / q;

                let s = if q <= l as u64 {
                    small[q as usize]
                } else {
                    let xd = x as u64 * d;
                    if xd <= l as u64 {
                        large[xd as usize]
                    } else if q <= slimit as u64 {
                        ((prefix_phi[q as usize] % mod_val as i64 + mod_val as i64) % mod_val as i64) as u64
                    } else {
                        small[(n / xd) as usize]
                    }
                };

                let count = (d_max - d + 1) % mod_val;
                val = (val + mod_val - count * s % mod_val) % mod_val;

                d = d_max + 1;
            }

            large[x] = val;
        }

        (small, large)
    }

    fn get_sp(n: u64, l: u64, small: &[u64], large: &[u64], v: u64) -> u64 {
        if v <= l { small[v as usize] } else { large[(n / v) as usize] }
    }

    let m1 = MOD - 1;
    let (small1, large1) = compute_sum_phi(n, l, slimit, &prefix_phi, m1);
    let (small2, large2) = compute_sum_phi(n, l, slimit, &prefix_phi, MOD);

    let ndiv_l = (n / l) as usize;

    let mut big_t = 1u64;
    for x in 1..=ndiv_l {
        big_t = (big_t as u128 * pow_mod(n / x as u64 + 1, phi_arr[x], MOD) as u128 % MOD as u128) as u64;
    }
    for q in 1..l as usize {
        let spq = get_sp(n, l, &small1, &large1, n / q as u64);
        let spq1 = get_sp(n, l, &small1, &large1, n / (q as u64 + 1));
        let diff = (spq + m1 - spq1) % m1;
        big_t = (big_t as u128 * pow_mod(q as u64 + 1, diff, MOD) as u128 % MOD as u128) as u64;
    }

    let sq_2n1 = {
        let v = (2 * (n % MOD) + 1) % MOD;
        (v as u128 * v as u128 % MOD as u128) as u64
    };

    let t8 = pow_mod(big_t, 8, MOD);
    let t4 = pow_mod(big_t, 4, MOD);

    let mut ans = (t8 + MOD - 1) % MOD;
    let sub = ((sq_2n1 + MOD - 1) as u128 * t4 as u128 % MOD as u128) as u64;
    ans = (ans + MOD - sub) % MOD;

    for x in 1..=ndiv_l {
        let sq = (n / x as u64) % MOD;
        let sq2 = (sq as u128 * sq as u128 % MOD as u128) as u64;
        let term = (4u128 * (phi_arr[x] % MOD) as u128 % MOD as u128 * sq2 as u128 % MOD as u128) as u64;
        ans = (ans + term) % MOD;
    }
    for q in 1..l as usize {
        let spq = get_sp(n, l, &small2, &large2, n / q as u64);
        let spq1 = get_sp(n, l, &small2, &large2, n / (q as u64 + 1));
        let diff = (spq + MOD - spq1) % MOD;
        let sq = (q as u64 * q as u64) % MOD;
        let term = (4u128 * diff as u128 % MOD as u128 * sq as u128 % MOD as u128) as u64;
        ans = (ans + term) % MOD;
    }

    ans %= MOD;
    println!("{}", ans);
}
