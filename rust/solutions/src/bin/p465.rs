fn main() {
    const MOD: u64 = 1_000_000_007;
    const M1: u64 = 1_000_000_006;

    #[inline(always)]
    fn mulmod(a: u64, b: u64, m: u64) -> u64 {
        (a * b) % m
    }

    fn pow_mod(mut base: u64, mut exp: u64, m: u64) -> u64 {
        let mut result = 1u64;
        base %= m;
        while exp > 0 {
            if exp & 1 == 1 {
                result = mulmod(result, base, m);
            }
            base = mulmod(base, base, m);
            exp >>= 1;
        }
        result
    }

    let n: u64 = {
        let mut v = 1u64;
        for _ in 0..13 {
            v *= 7;
        }
        v
    }; // 7^13

    let l = {
        let mut s = (n as f64).sqrt() as u64;
        while s * s > n {
            s -= 1;
        }
        while (s + 1) * (s + 1) <= n {
            s += 1;
        }
        s
    };

    let cbrt_n = (n as f64).cbrt();
    let mut slimit = (cbrt_n * cbrt_n) as usize + 100;
    let ndl = (n / l + 2) as usize;
    if slimit < ndl {
        slimit = ndl;
    }
    if slimit < l as usize + 2 {
        slimit = l as usize + 2;
    }

    let mut phi_arr = vec![0u32; slimit + 1];
    for i in 0..=slimit {
        phi_arr[i] = i as u32;
    }
    for i in 2..=slimit {
        if phi_arr[i] == i as u32 {
            let p = i as u32;
            let pm1 = p - 1;
            let mut j = i;
            while j <= slimit {
                phi_arr[j] = phi_arr[j] / p * pm1;
                j += i;
            }
        }
    }

    // Pre-reduce prefix sums so the hyperbola never does `% M` on i64.
    let mut prefix1 = vec![0u64; slimit + 1];
    let mut prefix2 = vec![0u64; slimit + 1];
    {
        let mut a1 = 0u64;
        let mut a2 = 0u64;
        for i in 1..=slimit {
            let p = phi_arr[i] as u64;
            a1 += p;
            if a1 >= M1 {
                a1 -= M1;
            }
            a2 += p;
            if a2 >= MOD {
                a2 -= MOD;
            }
            prefix1[i] = a1;
            prefix2[i] = a2;
        }
    }

    fn compute_sum_phi<const M: u64>(
        n: u64,
        l: u64,
        slimit: usize,
        prefix_phi: &[u64],
    ) -> (Vec<u64>, Vec<u64>) {
        let l_us = l as usize;
        let mut small = vec![0u64; l_us + 2];
        let mut large = vec![0u64; l_us + 2];
        let slimit_u32 = slimit as u32;

        let fill = l_us.min(slimit);
        small[..=fill].copy_from_slice(&prefix_phi[..=fill]);

        for x in (1..=l_us).rev() {
            let nv = n / x as u64;
            if nv <= l {
                large[x] = small[nv as usize];
                continue;
            }
            if nv <= slimit as u64 {
                large[x] = prefix_phi[nv as usize];
                continue;
            }

            let (half_n, other) = if nv % 2 == 0 {
                (nv / 2, nv + 1)
            } else {
                ((nv + 1) / 2, nv)
            };
            let init_val = (half_n % M) * (other % M) % M;

            let sub = if nv <= u32::MAX as u64 {
                let nv32 = nv as u32;
                let mut sum_sub = 0u64;
                let mut d = 2u32;
                while d <= nv32 {
                    let q = nv32 / d;
                    let d_max = nv32 / q;
                    let s = if (q as u64) <= l {
                        unsafe { *small.get_unchecked(q as usize) }
                    } else {
                        let xd = x as u64 * d as u64;
                        if xd <= l {
                            unsafe { *large.get_unchecked(xd as usize) }
                        } else if q <= slimit_u32 {
                            unsafe { *prefix_phi.get_unchecked(q as usize) }
                        } else {
                            unsafe { *small.get_unchecked((n / xd) as usize) }
                        }
                    };
                    let count = (d_max - d + 1) as u64;
                    sum_sub += count * s;
                    d = d_max.wrapping_add(1);
                    if d == 0 {
                        break;
                    }
                }
                sum_sub % M
            } else {
                let mut sum_sub = 0u128;
                let mut d = 2u64;
                while d <= nv {
                    let q = nv / d;
                    let d_max = nv / q;
                    let s = if q <= l {
                        unsafe { *small.get_unchecked(q as usize) }
                    } else {
                        let xd = x as u64 * d;
                        if xd <= l {
                            unsafe { *large.get_unchecked(xd as usize) }
                        } else if q <= slimit as u64 {
                            unsafe { *prefix_phi.get_unchecked(q as usize) }
                        } else {
                            unsafe { *small.get_unchecked((n / xd) as usize) }
                        }
                    };
                    let count = d_max - d + 1;
                    sum_sub += count as u128 * s as u128;
                    d = d_max + 1;
                }
                (sum_sub % M as u128) as u64
            };

            large[x] = (init_val + M - sub) % M;
        }

        (small, large)
    }

    fn get_sp(n: u64, l: u64, small: &[u64], large: &[u64], v: u64) -> u64 {
        if v <= l {
            small[v as usize]
        } else {
            large[(n / v) as usize]
        }
    }

    let ((small1, large1), (small2, large2)) = rayon::join(
        || compute_sum_phi::<M1>(n, l, slimit, &prefix1),
        || compute_sum_phi::<MOD>(n, l, slimit, &prefix2),
    );

    let ndiv_l = (n / l) as usize;

    let mut big_t = 1u64;
    for x in 1..=ndiv_l {
        big_t = mulmod(
            big_t,
            pow_mod(n / x as u64 + 1, phi_arr[x] as u64, MOD),
            MOD,
        );
    }
    for q in 1..l as usize {
        let spq = get_sp(n, l, &small1, &large1, n / q as u64);
        let spq1 = get_sp(n, l, &small1, &large1, n / (q as u64 + 1));
        let diff = (spq + M1 - spq1) % M1;
        big_t = mulmod(big_t, pow_mod(q as u64 + 1, diff, MOD), MOD);
    }

    let sq_2n1 = {
        let v = (2 * (n % MOD) + 1) % MOD;
        mulmod(v, v, MOD)
    };

    let t8 = pow_mod(big_t, 8, MOD);
    let t4 = pow_mod(big_t, 4, MOD);

    let mut ans = (t8 + MOD - 1) % MOD;
    let sub = mulmod((sq_2n1 + MOD - 1) % MOD, t4, MOD);
    ans = (ans + MOD - sub) % MOD;

    for x in 1..=ndiv_l {
        let sq = (n / x as u64) % MOD;
        let sq2 = mulmod(sq, sq, MOD);
        let term = mulmod(mulmod(4, phi_arr[x] as u64 % MOD, MOD), sq2, MOD);
        ans = (ans + term) % MOD;
    }
    for q in 1..l as usize {
        let spq = get_sp(n, l, &small2, &large2, n / q as u64);
        let spq1 = get_sp(n, l, &small2, &large2, n / (q as u64 + 1));
        let diff = (spq + MOD - spq1) % MOD;
        let sq = (q as u64 * q as u64) % MOD;
        let term = mulmod(mulmod(4, diff, MOD), sq, MOD);
        ans = (ans + term) % MOD;
    }

    ans %= MOD;
    println!("{}", ans);
}
