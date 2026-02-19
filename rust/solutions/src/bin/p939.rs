// Project Euler 939 - Partisan Nim
//
// Game theory analysis shows position is positive (A wins regardless of who moves first) iff:
//   diff >= 2, or (diff == 1 and total star parity is even)
// where diff = sum(a_i - 1 for A piles >= 2) - sum(b_j - 1 for B piles >= 2),
// star parity = (count of odd-sized piles on both sides) mod 2.
//
// Counting uses partition DP p(w,k), parity-split pe/po, and moment-based summation.

fn main() {
    const MOD: u64 = 1234567891;
    const N: usize = 5000;

    let inv2 = mod_pow(2, MOD - 2, MOD);
    let inv4 = inv2 * inv2 % MOD;

    let max_w = N;
    let s = max_w + 1; // stride

    // p(w, k), pe(w, k), po(w, k) as flat 2D arrays
    let mut pw = vec![0u64; s * s];
    let mut pe = vec![0u64; s * s];
    let mut po = vec![0u64; s * s];

    pw[0] = 1;
    pe[0] = 1;

    for w in 1..=max_w {
        for k in 1..=w {
            let idx = w * s + k;
            let prev = (w - 1) * s + (k - 1);
            pw[idx] = pw[prev];
            pe[idx] = pe[prev];
            po[idx] = po[prev];
            if w >= 2 * k {
                let si = (w - k) * s + k;
                pw[idx] = (pw[idx] + pw[si]) % MOD;
                if k % 2 == 0 {
                    pe[idx] = (pe[idx] + pe[si]) % MOD;
                    po[idx] = (po[idx] + po[si]) % MOD;
                } else {
                    let pe_si = pe[si];
                    let po_si = po[si];
                    pe[idx] = (pe[idx] + po_si) % MOD;
                    po[idx] = (po[idx] + pe_si) % MOD;
                }
            }
        }
    }

    // Prefix sums for each w
    let mut ps0 = vec![0u64; s * s];
    let mut ps1 = vec![0u64; s * s];
    let mut ps2 = vec![0u64; s * s];
    let mut pd0v = vec![0u64; s * s];
    let mut qd0v = vec![0u64; s * s];
    let mut qd1v = vec![0u64; s * s];

    for w in 0..=max_w {
        for k in 1..=w {
            let idx = w * s + k;
            let prev = w * s + (k - 1);
            let p_wk = pw[idx];
            let pd_wk = (pe[idx] + MOD - po[idx]) % MOD;
            let qd_wk = if k % 2 == 1 { (MOD - pd_wk) % MOD } else { pd_wk };
            let km = (k as u64) % MOD;

            ps0[idx] = (ps0[prev] + p_wk) % MOD;
            ps1[idx] = (ps1[prev] + km * p_wk % MOD) % MOD;
            ps2[idx] = (ps2[prev] + km * km % MOD * p_wk % MOD) % MOD;
            pd0v[idx] = (pd0v[prev] + pd_wk) % MOD;
            qd0v[idx] = (qd0v[prev] + qd_wk) % MOD;
            qd1v[idx] = (qd1v[prev] + km * qd_wk % MOD) % MOD;
        }
    }

    // get_triangle_sum(r, w_a, w_b): sum_{kA,kB} p(wA,kA)*p(wB,kB)*triangle(r-kA-kB)
    let get_ts = |r: usize, wa: usize, wb: usize| -> u64 {
        if wb == 0 {
            let km = wa.min(r);
            if km == 0 { return 0; }
            let i = wa * s + km;
            let (s0, s1, s2) = (ps0[i], ps1[i], ps2[i]);
            let r1 = ((r + 1) as u64) % MOD;
            let r2 = ((r + 2) as u64) % MOD;
            let r3 = ((2 * r + 3) as u64) % MOD;
            return (r1 * r2 % MOD * s0 % MOD + MOD * 3 - r3 * s1 % MOD + s2) % MOD * inv2 % MOD;
        }
        if r < 2 { return 0; }
        let km = wa.min(r - 1);
        if km == 0 { return 0; }
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
        for ka in start..=km {
            let c = r - ka;
            if c < 1 { continue; }
            let ib = wb * s + c;
            let (s0b, s1b, s2b) = (ps0[ib], ps1[ib], ps2[ib]);
            let c1 = ((c + 1) as u64) % MOD;
            let c2 = ((c + 2) as u64) % MOD;
            let c3 = ((2 * c + 3) as u64) % MOD;
            let iv = (c1 * c2 % MOD * s0b % MOD + s2b + MOD * 2 - c3 * s1b % MOD) % MOD * inv2 % MOD;
            res = (res + pw[wa * s + ka] * iv % MOD) % MOD;
        }
        res
    };

    // E_main: diff >= 2
    let mut e_main = 0u64;
    for wa in 2..N {
        for wb in 0..=(wa - 2) {
            if wa + wb >= N { break; }
            let r = N - wa - wb;
            e_main = (e_main + get_ts(r, wa, wb)) % MOD;
        }
    }

    // B_total: diff = 1, total triangle part
    let mut b_total = 0u64;
    for wa in 1..N {
        let wb = wa - 1;
        if wa + wb >= N { continue; }
        let r = N - wa - wb;
        b_total = (b_total + get_ts(r, wa, wb)) % MOD;
    }

    // B_diff: alternating parity sum for diff = 1
    let mut b_diff = 0u64;
    for wa in 1..N {
        let wb = wa - 1;
        let rv = N as i64 - wa as i64 - wb as i64;
        if rv < 1 { continue; }
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
                let pd_ka = (pe[wa * s + ka] + MOD - po[wa * s + ka]) % MOD;
                let qd_ka = if ka % 2 == 1 { (MOD - pd_ka) % MOD } else { pd_ka };
                val = (val + qd_ka * f) % MOD;
            }
            b_diff = (b_diff + sign_r * val % MOD) % MOD;
        } else {
            let km = wa.min(r.saturating_sub(1));
            if km == 0 { continue; }
            let k1 = if r > wb { km.min(r - wb) } else { 0 };

            if k1 >= 1 {
                let ia = wa * s + k1;
                let ib = wb * s + wb;
                let (q0a, q1a, p0a) = (qd0v[ia], qd1v[ia], pd0v[ia]);
                let (q0b, q1b, p0b) = (qd0v[ib], qd1v[ib], pd0v[ib]);
                let tm = (t as u64) % MOD;
                let t1 = (tm * q0a % MOD * q0b % MOD + MOD * 2
                    - q0a * q1b % MOD - q1a * q0b % MOD) % MOD * inv2 % MOD;
                let t2 = (MOD - q0a * q0b % MOD * inv4 % MOD) % MOD;
                let t3 = sign_t * p0a % MOD * p0b % MOD * inv4 % MOD;
                let val = (t1 + t2 + t3) % MOD;
                b_diff = (b_diff + sign_r * val % MOD) % MOD;
            }

            let start = if k1 >= 1 { k1 + 1 } else { 1 };
            for ka in start..=km {
                let c = r - ka;
                if c < 1 { continue; }
                let pd_ka = (pe[wa * s + ka] + MOD - po[wa * s + ka]) % MOD;
                let qd_ka = if ka % 2 == 1 { (MOD - pd_ka) % MOD } else { pd_ka };
                let stka: u64 = if (t - ka) % 2 == 0 { 1 } else { MOD - 1 };
                let ib = wb * s + c;
                let (q0b, q1b, p0b) = (qd0v[ib], qd1v[ib], pd0v[ib]);
                let tka = ((t - ka) as u64) % MOD;
                let inner = (tka * q0b % MOD + MOD - q1b) % MOD * inv2 % MOD;
                let inner = (inner + MOD - q0b * inv4 % MOD) % MOD;
                let inner = (inner + stka * p0b % MOD * inv4 % MOD) % MOD;
                b_diff = (b_diff + sign_r * qd_ka % MOD * inner % MOD) % MOD;
            }
        }
    }

    let e_boundary = (b_total + b_diff) % MOD * inv2 % MOD;
    let e_total = (e_main + e_boundary) % MOD;
    println!("{}", e_total);
}

fn mod_pow(mut base: u64, mut exp: u64, modulus: u64) -> u64 {
    let mut result = 1u64;
    base %= modulus;
    while exp > 0 {
        if exp & 1 == 1 { result = result * base % modulus; }
        base = base * base % modulus;
        exp >>= 1;
    }
    result
}
