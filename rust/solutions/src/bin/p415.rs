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

fn main() {
    let n = N_VAL;
    let l = isqrt_ll(n);
    let lu = l as usize;

    let mut u = (n as f64).powf(2.0 / 3.0).round() as i64;
    if u < l + 1 { u = l + 1; }
    let uu = u as usize;

    // Euler phi sieve
    let mut phi_arr: Vec<i32> = (0..=uu).map(|i| i as i32).collect();
    for i in 2..=uu {
        if phi_arr[i] == i as i32 {
            let ii = i as i32;
            let mut j = i;
            while j <= uu {
                // SAFETY: j <= uu, phi_arr.len() == uu+1
                unsafe {
                    let p = phi_arr.get_unchecked_mut(j);
                    *p -= *p / ii;
                }
                j += i;
            }
        }
    }

    // Interleaved prefixes: [sum phi, sum i*phi, sum i^2*phi] mod MOD
    let mut pref = vec![[0i32; 3]; uu + 1];
    {
        let mut r0 = 0i64;
        let mut r1 = 0i64;
        let mut r2 = 0i64;
        for x in 1..=uu {
            let p = unsafe { *phi_arr.get_unchecked(x) } as i64;
            let xi = x as i64;
            r0 += p;
            if r0 >= MOD { r0 -= MOD; }
            r1 += xi * p;
            r1 %= MOD;
            r2 += xi * xi % MOD * p;
            r2 %= MOD;
            unsafe {
                *pref.get_unchecked_mut(x) = [r0 as i32, r1 as i32, r2 as i32];
            }
        }
    }
    drop(phi_arr);

    // lg[g] = [T0, T1, T2](n/g). sm[q] == pref[q] for q <= L < U, so sm is omitted.
    let mut lg = vec![[0i32; 3]; lu + 2];

    // Fused Lucy DP for e = 0,1,2 (same floor-block structure).
    let mut g = lu;
    while g >= 1 {
        let nv = n / g as i64;
        if nv > u { break; }
        // SAFETY: nv <= u, g <= l
        unsafe {
            *lg.get_unchecked_mut(g) = *pref.get_unchecked(nv as usize);
        }
        g -= 1;
    }
    while g >= 1 {
        let nv = n / g as i64;
        let mut r0 = s1(nv);
        let mut r1 = s2(nv);
        let mut r2 = s3(nv);
        let mut prev0 = 1i64;
        let mut prev1 = 1i64;
        let mut prev2 = 1i64;
        let mut d = 2i64;

        // q = nv/d > u → T(q) lives in lg
        while d <= nv {
            let q = nv / d;
            if q <= u { break; }
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
            let idx = (n / q) as usize;
            // SAFETY: q > u ⇒ n/q < n/u < l
            let t = unsafe { *lg.get_unchecked(idx) };
            r0 = sub_mod(r0, c0 * t[0] as i64 % MOD);
            r1 = sub_mod(r1, c1 * t[1] as i64 % MOD);
            r2 = sub_mod(r2, c2 * t[2] as i64 % MOD);
            d = d_max + 1;
        }

        // q <= u → T(q) lives in pref
        while d <= nv {
            let q = nv / d;
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
            // SAFETY: q <= u (d only grows)
            let t = unsafe { *pref.get_unchecked(q as usize) };
            r0 = sub_mod(r0, c0 * t[0] as i64 % MOD);
            r1 = sub_mod(r1, c1 * t[1] as i64 % MOD);
            r2 = sub_mod(r2, c2 * t[2] as i64 % MOD);
            d = d_max + 1;
        }

        unsafe {
            *lg.get_unchecked_mut(g) = [r0 as i32, r1 as i32, r2 as i32];
        }
        g -= 1;
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
    let mut p2g = 1i64;
    for g in 1..=lu {
        p2g <<= 1;
        if p2g >= MOD { p2g -= MOD; }
        let gm = g as i64;
        let t = unsafe { *lg.get_unchecked(g) };
        let val = mod_norm(
            gm * gm % MOD * t[2] as i64 % MOD
            - 3 * n1 % MOD * gm % MOD * t[1] as i64 % MOD
            + 2 * n1sq % MOD * t[0] as i64 % MOD
            - ((n + 1 - gm) % MOD) * n1 % MOD
        );
        ans = sub_mod(ans, sub_mod(p2g, 2) * val % MOD);
    }

    // q = 1..N/L-1: q <= L so T_e(q) = pref[q]. Reuse N/(q+1) and 2^{nq+1}.
    let q_lim = n / l;
    let mut nq = n;
    let mut p2_nq = pow2_mod((nq + 1) as u64, &p2tab);
    for q in 1..q_lim {
        let nq1 = n / (q + 1);
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

        ans = mod_norm(
            ans
            - t2 * sag2 % MOD
            + n1 * v3t1m1 % MOD * sag1 % MOD
            - n1sq * v2t0m1 % MOD * sag0 % MOD
            + 2 * t2 % MOD * sp2 % MOD
            - 2 * n1 % MOD * v3t1m1 % MOD * sp1 % MOD
            + 2 * n1sq % MOD * v2t0m1 % MOD * sp0 % MOD
        );

        nq = nq1;
        p2_nq = p2_nq1;
    }

    println!("{}", mod_norm(ans));
}
