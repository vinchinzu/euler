// Project Euler 939 - Partisan Nim
//
// Game theory analysis shows position is positive (A wins regardless of who moves first) iff:
//   diff >= 2, or (diff == 1 and total star parity is even)
// where diff = sum(a_i - 1 for A piles >= 2) - sum(b_j - 1 for B piles >= 2),
// star parity = (count of odd-sized piles on both sides) mod 2.
//
// Counting uses partition DP p(w,k), parity-split pe/po, and moment-based summation.
// Optimized: compute pd = pe-po directly (skip pe/po), precompute ts_val,
// anti-diagonal prefix sums to collapse E_main from O(N^3) to O(N^2),
// on-the-fly parity prefix sums in B_diff to avoid 3 large arrays.
// Total memory: 7 large arrays (pw, pd, ps0, ps1, ps2, ts_val, ad) = 1.4GB.

const MOD: u64 = 1234567891;
const N: usize = 5000;

fn mod_pow(mut base: u64, mut exp: u64, modulus: u64) -> u64 {
    let mut result = 1u64;
    base %= modulus;
    while exp > 0 {
        if exp & 1 == 1 {
            result = result * base % modulus;
        }
        base = base * base % modulus;
        exp >>= 1;
    }
    result
}

fn main() {
    let inv2 = mod_pow(2, MOD - 2, MOD);
    let inv4 = inv2 * inv2 % MOD;

    let max_w = N;
    let s = max_w + 1; // stride

    // p(w, k) and pd(w, k) = pe(w,k) - po(w,k) mod MOD
    let mut pw = vec![0u64; s * s];
    let mut pd = vec![0u64; s * s];

    pw[0] = 1;
    pd[0] = 1;

    for w in 1..=max_w {
        for k in 1..=w {
            let idx = w * s + k;
            let prev = (w - 1) * s + (k - 1);
            pw[idx] = pw[prev];
            pd[idx] = pd[prev];
            if w >= 2 * k {
                let si = (w - k) * s + k;
                pw[idx] = (pw[idx] + pw[si]) % MOD;
                if k % 2 == 0 {
                    pd[idx] = (pd[idx] + pd[si]) % MOD;
                } else {
                    pd[idx] = (pd[idx] + MOD - pd[si]) % MOD;
                }
            }
        }
    }

    // Prefix sums ps0, ps1, ps2 for partition function moments
    let mut ps0 = vec![0u64; s * s];
    let mut ps1 = vec![0u64; s * s];
    let mut ps2 = vec![0u64; s * s];

    for w in 0..=max_w {
        for k in 1..=w {
            let idx = w * s + k;
            let prev = w * s + (k - 1);
            let p_wk = pw[idx];
            let km = (k as u64) % MOD;
            ps0[idx] = (ps0[prev] + p_wk) % MOD;
            ps1[idx] = (ps1[prev] + km * p_wk % MOD) % MOD;
            ps2[idx] = (ps2[prev] + km * km % MOD * p_wk % MOD) % MOD;
        }
    }

    // Precompute ts_val[wb][c] = triangle evaluation at (wb, c)
    let mut ts_val = vec![0u64; s * s];
    for wb in 1..=max_w {
        for c in 1..=wb {
            let ib = wb * s + c;
            let s0b = ps0[ib];
            let s1b = ps1[ib];
            let s2b = ps2[ib];
            let c1 = ((c + 1) as u64) % MOD;
            let c2 = ((c + 2) as u64) % MOD;
            let c3 = ((2 * c + 3) as u64) % MOD;
            ts_val[wb * s + c] =
                (c1 * c2 % MOD * s0b % MOD + s2b + MOD * 2 - c3 * s1b % MOD) % MOD * inv2 % MOD;
        }
    }

    // Anti-diagonal prefix sums for E_main residual.
    // Only stores entries where wb is in the valid range (no fill extension).
    let mut ad = vec![0u64; s * s];
    for j in 2..N {
        let wb_min = j / 2 + 1;
        let wb_max_valid = (j - 1).min(max_w);
        let mut running = 0u64;
        for wb in wb_min..=wb_max_valid {
            let c = j - wb;
            running = (running + ts_val[wb * s + c]) % MOD;
            ad[j * s + wb] = running;
        }
    }

    // E_main: diff >= 2
    let mut e_main = 0u64;

    for wa in 2..N {
        // (a) wb = 0: closed-form
        {
            let r = N - wa;
            let km = wa.min(r);
            if km >= 1 {
                let i = wa * s + km;
                let (s0, s1, s2) = (ps0[i], ps1[i], ps2[i]);
                let r1 = ((r + 1) as u64) % MOD;
                let r2 = ((r + 2) as u64) % MOD;
                let r3 = ((2 * r + 3) as u64) % MOD;
                let v =
                    (r1 * r2 % MOD * s0 % MOD + MOD * 3 - r3 * s1 % MOD + s2) % MOD * inv2 % MOD;
                e_main = (e_main + v) % MOD;
            }
        }

        // (b) wb >= 1: bulk O(1) part
        for wb in 1..=(wa - 2) {
            if wa + wb >= N {
                break;
            }
            let r = N - wa - wb;
            if r < 2 {
                continue;
            }
            let km = wa.min(r - 1);
            if km == 0 {
                continue;
            }
            let k1 = if r > wb { km.min(r - wb) } else { 0 };

            if k1 >= 1 {
                let ia = wa * s + k1;
                let ib = wb * s + wb;
                // SAFETY: ia, ib < s*s
                let (s0a, s1a, s2a) = unsafe {
                    (
                        *ps0.get_unchecked(ia),
                        *ps1.get_unchecked(ia),
                        *ps2.get_unchecked(ia),
                    )
                };
                let (s0b, s1b, s2b) = unsafe {
                    (
                        *ps0.get_unchecked(ib),
                        *ps1.get_unchecked(ib),
                        *ps2.get_unchecked(ib),
                    )
                };
                let r1 = ((r + 1) as u64) % MOD;
                let r2 = ((r + 2) as u64) % MOD;
                let r3 = ((2 * r + 3) as u64) % MOD;
                let ct = (s0b * r1 % MOD * r2 % MOD + s2b + MOD * 2 - r3 * s1b % MOD) % MOD;
                let lt = (2 * s1b % MOD + MOD * 2 - s0b * r3 % MOD) % MOD;
                let qt = s0b;
                let v = (ct * s0a % MOD + lt * s1a % MOD + qt * s2a % MOD) % MOD;
                e_main = (e_main + v * inv2 % MOD) % MOD;
            }
        }

        // (c) Residual part via anti-diagonal prefix sums.
        let ka_max_res = wa.min(N.saturating_sub(wa + 3));
        let wb_hi_base = (wa - 2).min(N.saturating_sub(wa + 2));
        if wb_hi_base >= 1 && ka_max_res >= 1 {
            let wa_row = wa * s;
            let mut acc = 0u64;
            for ka in 1..=ka_max_res {
                let j = N - wa - ka;
                if j < 3 {
                    break;
                }
                let wb_lo = j / 2 + 1;
                let wb_hi = wb_hi_base.min(j - 1);
                if wb_lo > wb_hi {
                    continue;
                }
                // SAFETY: indices in bounds
                let pw_val = unsafe { *pw.get_unchecked(wa_row + ka) };
                let ad_hi = unsafe { *ad.get_unchecked(j * s + wb_hi) };
                // wb_lo = j/2 + 1 is the start of the prefix sum, so no subtraction needed
                acc += pw_val * ad_hi % MOD;
            }
            e_main = (e_main + acc % MOD) % MOD;
        }
    }

    // B_total: diff = 1, total triangle part
    let mut b_total = 0u64;
    for wa in 1..N {
        let wb = wa - 1;
        if wa + wb >= N {
            continue;
        }
        let r = N - wa - wb;
        b_total = (b_total + get_ts(r, wa, wb, s, inv2, &ps0, &ps1, &ps2, &pw, &ts_val)) % MOD;
    }

    // B_diff: alternating parity sum for diff = 1
    // Compute parity prefix sums (pd0v, qd0v, qd1v) on the fly for needed rows.
    let mut b_diff = 0u64;
    let mut row_pd0 = vec![0u64; s];
    let mut row_qd0 = vec![0u64; s];
    let mut row_qd1 = vec![0u64; s];

    for wa in 1..N {
        let wb = wa - 1;
        let rv = N as i64 - wa as i64 - wb as i64;
        if rv < 1 {
            continue;
        }
        let r = rv as usize;
        let t = r + 2;
        let sign_r: u64 = if r % 2 == 0 { 1 } else { MOD - 1 };
        let sign_t: u64 = if t % 2 == 0 { 1 } else { MOD - 1 };

        if wb == 0 {
            let km = wa.min(r);
            let mut val = 0u64;
            for ka in 1..=km {
                let mv = r - ka;
                let f = ((mv + 2) / 2) as u64 % MOD;
                let pd_ka = pd[wa * s + ka];
                let qd_ka = if ka % 2 == 1 {
                    (MOD - pd_ka) % MOD
                } else {
                    pd_ka
                };
                val = (val + qd_ka * f) % MOD;
            }
            b_diff = (b_diff + sign_r * val % MOD) % MOD;
        } else {
            let km = wa.min(r.saturating_sub(1));
            if km == 0 {
                continue;
            }
            let k1 = if r > wb { km.min(r - wb) } else { 0 };

            if k1 >= 1 {
                compute_row_prefix(&pd, wa, k1, s, &mut row_pd0, &mut row_qd0, &mut row_qd1);
                let (q0a, q1a, p0a) = (row_qd0[k1], row_qd1[k1], row_pd0[k1]);

                compute_row_prefix(&pd, wb, wb, s, &mut row_pd0, &mut row_qd0, &mut row_qd1);
                let (q0b, q1b, p0b) = (row_qd0[wb], row_qd1[wb], row_pd0[wb]);
                let _ = p0b;

                let tm = (t as u64) % MOD;
                let t1 = (tm * q0a % MOD * q0b % MOD + MOD * 2 - q0a * q1b % MOD
                    - q1a * q0b % MOD)
                    % MOD
                    * inv2
                    % MOD;
                let t2 = (MOD - q0a * q0b % MOD * inv4 % MOD) % MOD;
                let t3 = sign_t * p0a % MOD * p0b % MOD * inv4 % MOD;
                let val = (t1 + t2 + t3) % MOD;
                b_diff = (b_diff + sign_r * val % MOD) % MOD;
            }

            let start = if k1 >= 1 { k1 + 1 } else { 1 };
            if start <= km {
                let max_c = r - start;
                if max_c >= 1 {
                    compute_row_prefix(
                        &pd,
                        wb,
                        max_c,
                        s,
                        &mut row_pd0,
                        &mut row_qd0,
                        &mut row_qd1,
                    );
                    for ka in start..=km {
                        let c = r - ka;
                        if c < 1 {
                            continue;
                        }
                        let pd_ka = pd[wa * s + ka];
                        let qd_ka = if ka % 2 == 1 {
                            (MOD - pd_ka) % MOD
                        } else {
                            pd_ka
                        };
                        let stka: u64 = if (t - ka) % 2 == 0 { 1 } else { MOD - 1 };
                        let (q0b, q1b, p0b) = (row_qd0[c], row_qd1[c], row_pd0[c]);
                        let tka = ((t - ka) as u64) % MOD;
                        let inner = (tka * q0b % MOD + MOD - q1b) % MOD * inv2 % MOD;
                        let inner = (inner + MOD - q0b * inv4 % MOD) % MOD;
                        let inner = (inner + stka * p0b % MOD * inv4 % MOD) % MOD;
                        b_diff = (b_diff + sign_r * qd_ka % MOD * inner % MOD) % MOD;
                    }
                }
            }
        }
    }

    let e_boundary = (b_total + b_diff) % MOD * inv2 % MOD;
    let e_total = (e_main + e_boundary) % MOD;
    println!("{}", e_total);
}

#[inline]
fn compute_row_prefix(
    pd: &[u64],
    w: usize,
    k_max: usize,
    s: usize,
    row_pd0: &mut [u64],
    row_qd0: &mut [u64],
    row_qd1: &mut [u64],
) {
    row_pd0[0] = 0;
    row_qd0[0] = 0;
    row_qd1[0] = 0;
    let base = w * s;
    for k in 1..=k_max {
        let pd_wk = pd[base + k];
        let qd_wk = if k % 2 == 1 {
            (MOD - pd_wk) % MOD
        } else {
            pd_wk
        };
        let km = (k as u64) % MOD;
        row_pd0[k] = (row_pd0[k - 1] + pd_wk) % MOD;
        row_qd0[k] = (row_qd0[k - 1] + qd_wk) % MOD;
        row_qd1[k] = (row_qd1[k - 1] + km * qd_wk % MOD) % MOD;
    }
}

fn get_ts(
    r: usize,
    wa: usize,
    wb: usize,
    s: usize,
    inv2: u64,
    ps0: &[u64],
    ps1: &[u64],
    ps2: &[u64],
    pw: &[u64],
    ts_val: &[u64],
) -> u64 {
    if wb == 0 {
        let km = wa.min(r);
        if km == 0 {
            return 0;
        }
        let i = wa * s + km;
        let (s0, s1, s2) = (ps0[i], ps1[i], ps2[i]);
        let r1 = ((r + 1) as u64) % MOD;
        let r2 = ((r + 2) as u64) % MOD;
        let r3 = ((2 * r + 3) as u64) % MOD;
        return (r1 * r2 % MOD * s0 % MOD + MOD * 3 - r3 * s1 % MOD + s2) % MOD * inv2 % MOD;
    }
    if r < 2 {
        return 0;
    }
    let km = wa.min(r - 1);
    if km == 0 {
        return 0;
    }
    let k1 = if r > wb { km.min(r - wb) } else { 0 };
    let mut res = 0u64;

    if k1 >= 1 {
        let ia = wa * s + k1;
        let ib = wb * s + wb;
        let (s0a, s1a, s2a) = (ps0[ia], ps1[ia], ps2[ia]);
        let (s0b, s1b, s2b) = (ps0[ib], ps1[ib], ps2[ib]);
        let r1 = ((r + 1) as u64) % MOD;
        let r2 = ((r + 2) as u64) % MOD;
        let r3 = ((2 * r + 3) as u64) % MOD;
        let ct = (s0b * r1 % MOD * r2 % MOD + s2b + MOD * 2 - r3 * s1b % MOD) % MOD;
        let lt = (2 * s1b % MOD + MOD * 2 - s0b * r3 % MOD) % MOD;
        let qt = s0b;
        let v = (ct * s0a % MOD + lt * s1a % MOD + qt * s2a % MOD) % MOD;
        res = v * inv2 % MOD;
    }

    let start = if k1 >= 1 { k1 + 1 } else { 1 };
    let wa_row = wa * s;
    let wb_row = wb * s;
    let mut acc = 0u64;
    for ka in start..=km {
        let pw_val = unsafe { *pw.get_unchecked(wa_row + ka) };
        let tv = unsafe { *ts_val.get_unchecked(wb_row + r - ka) };
        acc += pw_val * tv % MOD;
    }
    res = (res + acc % MOD) % MOD;
    res
}
