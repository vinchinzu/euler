use rayon::prelude::*;

// Project Euler 415 - Titanic sets
// Lucy DP for summatory totient moments, then closed-form 2^g sums.

const N_VAL: i64 = 100_000_000_000; // 10^11
const MOD: i64 = 100_000_000;       // 10^8
const M5: i64 = 390_625;            // 5^8
const PHI5: u64 = 312_500;          // φ(5^8)
const INV256: i64 = 340_271;        // 256^{-1} mod 5^8

#[inline(always)]
fn mod_norm(x: i64) -> i64 {
    let r = x % MOD;
    if r < 0 { r + MOD } else { r }
}

#[inline(always)]
fn sub_mod(a: i64, b: i64) -> i64 {
    let r = a - b;
    if r < 0 { r + MOD } else { r }
}

#[inline(always)]
fn s0(n: i64) -> i64 {
    n % MOD
}

#[inline(always)]
fn s1(n: i64) -> i64 {
    if n & 1 == 0 {
        (n / 2 % MOD) * ((n + 1) % MOD) % MOD
    } else {
        (n % MOD) * ((n + 1) / 2 % MOD) % MOD
    }
}

#[inline(always)]
fn s2(n: i64) -> i64 {
    let (mut a, mut b, mut c) = (n, n + 1, 2 * n + 1);
    if a & 1 == 0 { a >>= 1; } else { b >>= 1; }
    if a % 3 == 0 { a /= 3; } else if b % 3 == 0 { b /= 3; } else { c /= 3; }
    (a % MOD) * (b % MOD) % MOD * (c % MOD) % MOD
}

#[inline(always)]
fn s3(n: i64) -> i64 {
    let t = s1(n);
    t * t % MOD
}

#[inline(always)]
fn pow_mod(mut base: i64, mut exp: u64) -> i64 {
    let mut r = 1i64;
    base %= MOD;
    while exp > 0 {
        if exp & 1 == 1 { r = r * base % MOD; }
        base = base * base % MOD;
        exp >>= 1;
    }
    r
}

/// 2^n mod 10^8 for n >= 8, via CRT (0 mod 256, 2^{n mod φ(5^8)} mod 5^8).
#[inline(always)]
fn pow2_mod(n: u64, tab: &[i32]) -> i64 {
    // SAFETY: n % PHI5 < tab.len() == PHI5
    let a = unsafe { *tab.get_unchecked((n % PHI5) as usize) } as i64;
    256 * (a * INV256 % M5)
}

#[inline(always)]
fn sum_ag_all(l: i64, p2: i64) -> (i64, i64, i64) {
    let lm = l % MOD;
    let s_ag0 = sub_mod(p2, 2);
    let lm1 = if lm == 0 { MOD - 1 } else { lm - 1 };
    let s_ag1 = (lm1 * p2 + 2) % MOD;
    let t = mod_norm(lm * lm % MOD - 2 * lm % MOD + 3);
    let s_ag2 = sub_mod(t * p2 % MOD, 6);
    (s_ag0, s_ag1, s_ag2)
}

fn isqrt_ll(n: i64) -> i64 {
    let mut r = (n as f64).sqrt() as i64;
    while r * r > n { r -= 1; }
    while (r + 1) * (r + 1) <= n { r += 1; }
    r
}

#[inline(always)]
fn compute_lg(g: usize, n: i64, u: i64, pref: &[[i32; 3]], lg: &[[i32; 3]]) -> [i32; 3] {
    let nv = n / g as i64;
    let mut r0 = s1(nv);
    let mut r1 = s2(nv);
    let mut r2 = s3(nv);

    let k = isqrt_ll(nv);
    let d_cut = (nv / (u + 1)).min(k);
    let mut d = 2i64;

    // Range 1a: d <= d_cut (where q = nv/d > u, T(q) lives in lg)
    while d <= d_cut {
        let q = nv / d;
        let idx = (n / q) as usize;
        let t = unsafe { *lg.get_unchecked(idx) };
        let dm = d;
        let d2m = dm * dm;
        r0 = sub_mod(r0, t[0] as i64);
        r1 = sub_mod(r1, dm * t[1] as i64 % MOD);
        r2 = sub_mod(r2, d2m * t[2] as i64 % MOD);
        d += 1;
    }

    // Range 1b: d_cut < d <= k (where q = nv/d <= u, T(q) lives in pref)
    let mut sum0 = 0u64;
    let mut sum1 = 0u64;
    let mut sum2 = 0u128;

    if nv <= u32::MAX as i64 {
        let nv32 = nv as u32;
        while d <= k {
            let q = (nv32 / (d as u32)) as usize;
            let t = unsafe { *pref.get_unchecked(q) };
            let d64 = d as u64;
            sum0 += t[0] as u64;
            sum1 += d64 * (t[1] as u64);
            sum2 += (d64 * d64) as u128 * (t[2] as u128);
            d += 1;
        }
    } else {
        while d <= k {
            let q = (nv / d) as usize;
            let t = unsafe { *pref.get_unchecked(q) };
            let d64 = d as u64;
            sum0 += t[0] as u64;
            sum1 += d64 * (t[1] as u64);
            sum2 += (d64 * d64) as u128 * (t[2] as u128);
            d += 1;
        }
    }

    r0 = sub_mod(r0, (sum0 % MOD as u64) as i64);
    r1 = sub_mod(r1, (sum1 % MOD as u64) as i64);
    r2 = sub_mod(r2, (sum2 % MOD as u128) as i64);

    // Range 2: q from q_max down to 1 (T(q) lives in pref)
    let q_max = nv / (k + 1);
    let mut prev0 = k % MOD;
    let mut prev1 = s1(k);
    let mut prev2 = s2(k);

    let mut r2_0 = 0u128;
    let mut r2_1 = 0u128;
    let mut r2_2 = 0u128;

    if nv <= u32::MAX as i64 {
        let nv32 = nv as u32;
        let mut q = q_max;
        while q >= 1 {
            let d_max = (nv32 / (q as u32)) as i64;
            let cur0 = d_max % MOD;
            let cur1 = s1(d_max);
            let cur2 = s2(d_max);
            let c0 = sub_mod(cur0, prev0);
            let c1 = sub_mod(cur1, prev1);
            let c2 = sub_mod(cur2, prev2);
            prev0 = cur0;
            prev1 = cur1;
            prev2 = cur2;
            let t = unsafe { *pref.get_unchecked(q as usize) };
            r2_0 += (c0 as u128) * (t[0] as u128);
            r2_1 += (c1 as u128) * (t[1] as u128);
            r2_2 += (c2 as u128) * (t[2] as u128);
            q -= 1;
        }
    } else {
        let mut q = q_max;
        while q >= 1 {
            let d_max = nv / q;
            let cur0 = d_max % MOD;
            let cur1 = s1(d_max);
            let cur2 = s2(d_max);
            let c0 = sub_mod(cur0, prev0);
            let c1 = sub_mod(cur1, prev1);
            let c2 = sub_mod(cur2, prev2);
            prev0 = cur0;
            prev1 = cur1;
            prev2 = cur2;
            let t = unsafe { *pref.get_unchecked(q as usize) };
            r2_0 += (c0 as u128) * (t[0] as u128);
            r2_1 += (c1 as u128) * (t[1] as u128);
            r2_2 += (c2 as u128) * (t[2] as u128);
            q -= 1;
        }
    }

    r0 = sub_mod(r0, (r2_0 % MOD as u128) as i64);
    r1 = sub_mod(r1, (r2_1 % MOD as u128) as i64);
    r2 = sub_mod(r2, (r2_2 % MOD as u128) as i64);

    [r0 as i32, r1 as i32, r2 as i32]
}

fn main() {
    let n = N_VAL;
    let l = isqrt_ll(n);
    let lu = l as usize;

    let mut u = (n as f64).powf(2.0 / 3.0).round() as i64;
    if u < l + 1 { u = l + 1; }
    let uu = u as usize;

    // Linear sieve for phi
    let mut phi_arr = vec![0i32; uu + 1];
    let mut primes = Vec::with_capacity(1_400_000);
    phi_arr[1] = 1;
    for i in 2..=uu {
        if phi_arr[i] == 0 {
            phi_arr[i] = (i - 1) as i32;
            primes.push(i as u32);
        }
        let pi = phi_arr[i];
        for &p in &primes {
            let pu = p as usize;
            let ip = i * pu;
            if ip > uu { break; }
            if i % pu == 0 {
                phi_arr[ip] = pi * (p as i32);
                break;
            } else {
                phi_arr[ip] = pi * (p as i32 - 1);
            }
        }
    }
    drop(primes);

    // Parallel chunked prefix scan for pref: [sum phi, sum i*phi, sum i^2*phi] mod MOD
    let mut pref = vec![[0i32; 3]; uu + 1];
    let num_chunks = 128;
    let chunk_size = uu.div_ceil(num_chunks);

    let chunk_totals: Vec<[i64; 3]> = (0..num_chunks).into_par_iter().map(|c| {
        let start = 1 + c * chunk_size;
        let end = (start + chunk_size).min(uu + 1);
        let mut r0 = 0u64;
        let mut r1 = 0u128;
        let mut r2 = 0u128;
        for x in start..end {
            let p = unsafe { *phi_arr.get_unchecked(x) } as u64;
            let xi = x as u64;
            let x2m = (xi * xi) % (MOD as u64);
            r0 += p;
            r1 += (xi * p) as u128;
            r2 += (x2m * p) as u128;
        }
        [(r0 % MOD as u64) as i64, (r1 % MOD as u128) as i64, (r2 % MOD as u128) as i64]
    }).collect();

    let mut chunk_offsets = vec![[0i64; 3]; num_chunks];
    for c in 1..num_chunks {
        let prev = chunk_offsets[c - 1];
        let tot = chunk_totals[c - 1];
        chunk_offsets[c] = [
            (prev[0] + tot[0]) % MOD,
            (prev[1] + tot[1]) % MOD,
            (prev[2] + tot[2]) % MOD,
        ];
    }

    pref[1..].par_chunks_mut(chunk_size).enumerate().for_each(|(c, chunk)| {
        let start = 1 + c * chunk_size;
        let off = chunk_offsets[c];
        let mut r0 = off[0];
        let mut r1 = off[1];
        let mut r2 = off[2];
        for (i, item) in chunk.iter_mut().enumerate() {
            let x = (start + i) as i64;
            let p = unsafe { *phi_arr.get_unchecked(x as usize) } as i64;
            r0 += p;
            if r0 >= MOD { r0 -= MOD; }
            r1 = (r1 + x * p) % MOD;
            r2 = (r2 + (x * x % MOD) * p) % MOD;
            *item = [r0 as i32, r1 as i32, r2 as i32];
        }
    });

    drop(phi_arr);

    // lg[g] = [T0, T1, T2](n/g). sm[q] == pref[q] for q <= L < U, so sm is omitted.
    let mut lg = vec![[0i32; 3]; lu + 2];

    let g_cut = (n / (u + 1)) as usize;
    for g in (g_cut + 1)..=lu {
        let nv = (n / g as i64) as usize;
        unsafe {
            *lg.get_unchecked_mut(g) = *pref.get_unchecked(nv);
        }
    }

    let mut top = g_cut;
    while top >= 1 {
        let bot = (top / 2) + 1;
        let chunk: Vec<[i32; 3]> = (bot..=top)
            .into_par_iter()
            .map(|g| compute_lg(g, n, u, &pref, &lg))
            .collect();
        for (i, &val) in chunk.iter().enumerate() {
            unsafe {
                *lg.get_unchecked_mut(bot + i) = val;
            }
        }
        top = bot - 1;
    }

    // 2^k mod 5^8 for k < φ(5^8)
    let mut p2tab = vec![0i32; PHI5 as usize];
    p2tab[0] = 1;
    for i in 1..PHI5 as usize {
        p2tab[i] = ((p2tab[i - 1] as i64 * 2) % M5) as i32;
    }

    let p2n1 = pow2_mod((n + 1) as u64, &p2tab);
    let mut ans = pow_mod(p2n1, (n + 1) as u64);
    let n1 = (n + 1) % MOD;
    let n1sq = n1 * n1 % MOD;
    ans = sub_mod(ans, 1);
    ans = sub_mod(ans, n1sq);

    let term = mod_norm(p2n1 - 1 - n1 - s1(n));
    ans = sub_mod(ans, 2 * n1 % MOD * term % MOD);

    // g = 1..L: incremental 2^g. N/g > L so T_e(N/g) = lg[g].
    let mut p2g_arr = vec![0i32; lu + 1];
    let mut cur_p2g = 1i64;
    for item in p2g_arr[1..=lu].iter_mut() {
        cur_p2g <<= 1;
        if cur_p2g >= MOD { cur_p2g -= MOD; }
        *item = cur_p2g as i32;
    }

    let loop1_sum: i64 = (1..=lu)
        .into_par_iter()
        .map(|g| {
            let gm = g as i64;
            let t = unsafe { *lg.get_unchecked(g) };
            let val = mod_norm(
                gm * gm % MOD * t[2] as i64 % MOD
                - 3 * n1 % MOD * gm % MOD * t[1] as i64 % MOD
                + 2 * n1sq % MOD * t[0] as i64 % MOD
                - ((n + 1 - gm) % MOD) * n1 % MOD
            );
            let p2g = p2g_arr[g] as i64;
            sub_mod(p2g, 2) * val % MOD
        })
        .sum();
    ans = sub_mod(ans, loop1_sum % MOD);

    // q = 1..N/L-1: q <= L so T_e(q) = pref[q].
    let q_lim = n / l;
    let loop2_sum: i64 = (1..q_lim)
        .into_par_iter()
        .map(|q| {
            let nq = n / q;
            let nq1 = n / (q + 1);
            let p2_nq = pow2_mod((nq + 1) as u64, &p2tab);
            let p2_nq1 = pow2_mod((nq1 + 1) as u64, &p2tab);

            let t = unsafe { *pref.get_unchecked(q as usize) };
            let t0 = t[0] as i64;
            let t1 = t[1] as i64;
            let t2 = t[2] as i64;

            let (ag0, ag1, ag2) = sum_ag_all(nq, p2_nq);
            let (bg0, bg1, bg2) = sum_ag_all(nq1, p2_nq1);
            let sag0 = sub_mod(ag0, bg0);
            let sag1 = sub_mod(ag1, bg1);
            let sag2 = sub_mod(ag2, bg2);

            let sp0 = sub_mod(s0(nq), s0(nq1));
            let sp1 = sub_mod(s1(nq), s1(nq1));
            let sp2 = sub_mod(s2(nq), s2(nq1));

            let v3t1m1 = mod_norm(3 * t1 - 1);
            let v2t0m1 = mod_norm(2 * t0 - 1);

            mod_norm(
                - t2 * sag2 % MOD
                + n1 * v3t1m1 % MOD * sag1 % MOD
                - n1sq * v2t0m1 % MOD * sag0 % MOD
                + 2 * t2 % MOD * sp2 % MOD
                - 2 * n1 % MOD * v3t1m1 % MOD * sp1 % MOD
                + 2 * n1sq % MOD * v2t0m1 % MOD * sp0 % MOD
            )
        })
        .sum();
    ans = mod_norm(ans + loop2_sum % MOD);

    println!("{}", mod_norm(ans));
}
