// Project Euler 529 - 10-substring-friendly Numbers
//
// DP with states (mask, s), BFS to discover states, sparse matrix-vector multiply
// to generate sequence terms, Berlekamp-Massey to find linear recurrence,
// polynomial exponentiation to evaluate at n = 10^18.
// Uses 3-prime NTT + Barrett reduction for O(l log l) poly_mult_mod.
// Optimized: u64 arithmetic, precomputed NTT transforms.

const MOD: u64 = 1_000_000_007;
const P1: u64 = 998_244_353;
const P2: u64 = 985_661_441;
const P3: u64 = 754_974_721;
const B_DIGIT: usize = 10;
const MAX_STATES: usize = 6000;
const NUM_TERMS: usize = 5600;

#[inline(always)]
fn mulmod(a: u64, b: u64, m: u64) -> u64 {
    (a * b) % m
}

fn pw(mut base: u64, mut exp: u64, m: u64) -> u64 {
    let mut r = 1u64;
    base %= m;
    while exp > 0 {
        if exp & 1 == 1 { r = mulmod(r, base, m); }
        base = mulmod(base, base, m);
        exp >>= 1;
    }
    r
}

fn pack(mask: usize, s: usize) -> usize { mask * 11 + s }

fn ntt<const M: u64, const G: u64>(a: &mut [u64], inv: bool) {
    let n = a.len();
    let shift = usize::BITS - n.trailing_zeros();
    for i in 1..n {
        let j = i.reverse_bits() >> shift;
        if i < j { a.swap(i, j); }
    }
    let mut len = 2;
    while len <= n {
        let w = if inv { pw(G, M - 1 - (M - 1) / len as u64, M) } else { pw(G, (M - 1) / len as u64, M) };
        let half = len / 2;
        if half == 1 {
            for i in (0..n).step_by(2) {
                let u = a[i];
                let v = a[i + 1];
                a[i] = if u + v >= M { u + v - M } else { u + v };
                a[i + 1] = if u >= v { u - v } else { u + M - v };
            }
        } else {
            let mut twiddles = [0u64; 8192];
            let mut wn = 1u64;
            for jj in 0..half {
                twiddles[jj] = wn;
                wn = mulmod(wn, w, M);
            }
            for i in (0..n).step_by(len) {
                let u_slice = &mut a[i..i + len];
                let (left, right) = u_slice.split_at_mut(half);
                for jj in 0..half {
                    let u = left[jj];
                    let v = mulmod(right[jj], twiddles[jj], M);
                    left[jj] = if u + v >= M { u + v - M } else { u + v };
                    right[jj] = if u >= v { u - v } else { u + M - v };
                }
            }
        }
        len <<= 1;
    }
    if inv {
        let inv_n = pw(n as u64, M - 2, M);
        for v in a.iter_mut() { *v = mulmod(*v, inv_n, M); }
    }
}

fn ntt_size(nc: usize) -> usize {
    let mut n = 1;
    while n < nc { n <<= 1; }
    n
}

struct CrtHelper {
    inv12: u64,
    inv13: u64,
    p1_mod: u64,
    p1p2_mod: u64,
}

impl CrtHelper {
    fn new() -> Self {
        CrtHelper {
            inv12: pw(P1, P2 - 2, P2),
            inv13: pw(mulmod(P1, P2, P3), P3 - 2, P3),
            p1_mod: P1 % MOD,
            p1p2_mod: mulmod(P1 % MOD, P2 % MOD, MOD),
        }
    }

    #[inline(always)]
    fn reconstruct(&self, r1: u64, r2: u64, r3: u64) -> u64 {
        let x1 = r1;
        let x2 = mulmod((r2 + P2 - x1 % P2) % P2, self.inv12, P2);
        let val = (x1 + mulmod(x2, P1 % P3, P3)) % P3;
        let x3 = mulmod((r3 + P3 - val) % P3, self.inv13, P3);
        let term2 = mulmod(x2, self.p1_mod, MOD);
        let term3 = mulmod(x3, self.p1p2_mod, MOD);
        (x1 + term2 + term3) % MOD
    }
}

struct NttPre { t1: Vec<u64>, t2: Vec<u64>, t3: Vec<u64> }

impl NttPre {
    fn new(poly: &[u64], ntt_sz: usize) -> Self {
        let mut t1 = vec![0u64; ntt_sz];
        let mut t2 = vec![0u64; ntt_sz];
        let mut t3 = vec![0u64; ntt_sz];
        for i in 0..poly.len() {
            t1[i] = poly[i] % P1;
            t2[i] = poly[i] % P2;
            t3[i] = poly[i] % P3;
        }
        rayon::join(
            || ntt::<P1, 3>(&mut t1, false),
            || rayon::join(
                || ntt::<P2, 3>(&mut t2, false),
                || ntt::<P3, 11>(&mut t3, false),
            ),
        );
        NttPre { t1, t2, t3 }
    }
}

fn poly_mul_with_pre(a: &[u64], b_pre: &NttPre, ntt_sz: usize, nc: usize, trunc: usize, crt: &CrtHelper) -> Vec<u64> {
    let n = ntt_sz;
    let mut a1 = vec![0u64; n];
    let mut a2 = vec![0u64; n];
    let mut a3 = vec![0u64; n];
    for i in 0..a.len().min(n) {
        a1[i] = a[i] % P1;
        a2[i] = a[i] % P2;
        a3[i] = a[i] % P3;
    }
    rayon::join(
        || {
            ntt::<P1, 3>(&mut a1, false);
            for i in 0..n {
                a1[i] = mulmod(a1[i], b_pre.t1[i], P1);
            }
            ntt::<P1, 3>(&mut a1, true);
        },
        || rayon::join(
            || {
                ntt::<P2, 3>(&mut a2, false);
                for i in 0..n {
                    a2[i] = mulmod(a2[i], b_pre.t2[i], P2);
                }
                ntt::<P2, 3>(&mut a2, true);
            },
            || {
                ntt::<P3, 11>(&mut a3, false);
                for i in 0..n {
                    a3[i] = mulmod(a3[i], b_pre.t3[i], P3);
                }
                ntt::<P3, 11>(&mut a3, true);
            },
        ),
    );
    let out_len = nc.min(trunc);
    let mut res = vec![0u64; out_len];
    for i in 0..out_len {
        res[i] = crt.reconstruct(a1[i], a2[i], a3[i]);
    }
    res
}

fn poly_mul(a: &[u64], b: &[u64], crt: &CrtHelper) -> Vec<u64> {
    if a.is_empty() || b.is_empty() { return vec![]; }
    let nc = a.len() + b.len() - 1;
    let n = ntt_size(nc);
    let is_sqr = std::ptr::eq(a, b);
    let mut a1 = vec![0u64; n];
    let mut a2 = vec![0u64; n];
    let mut a3 = vec![0u64; n];
    for i in 0..a.len() { a1[i] = a[i] % P1; a2[i] = a[i] % P2; a3[i] = a[i] % P3; }

    if is_sqr {
        rayon::join(
            || {
                ntt::<P1, 3>(&mut a1, false);
                for i in 0..n { a1[i] = mulmod(a1[i], a1[i], P1); }
                ntt::<P1, 3>(&mut a1, true);
            },
            || rayon::join(
                || {
                    ntt::<P2, 3>(&mut a2, false);
                    for i in 0..n { a2[i] = mulmod(a2[i], a2[i], P2); }
                    ntt::<P2, 3>(&mut a2, true);
                },
                || {
                    ntt::<P3, 11>(&mut a3, false);
                    for i in 0..n { a3[i] = mulmod(a3[i], a3[i], P3); }
                    ntt::<P3, 11>(&mut a3, true);
                },
            ),
        );
    } else {
        let mut b1 = vec![0u64; n];
        let mut b2 = vec![0u64; n];
        let mut b3 = vec![0u64; n];
        for i in 0..b.len() { b1[i] = b[i] % P1; b2[i] = b[i] % P2; b3[i] = b[i] % P3; }
        rayon::join(
            || {
                ntt::<P1, 3>(&mut a1, false);
                ntt::<P1, 3>(&mut b1, false);
                for i in 0..n { a1[i] = mulmod(a1[i], b1[i], P1); }
                ntt::<P1, 3>(&mut a1, true);
            },
            || rayon::join(
                || {
                    ntt::<P2, 3>(&mut a2, false);
                    ntt::<P2, 3>(&mut b2, false);
                    for i in 0..n { a2[i] = mulmod(a2[i], b2[i], P2); }
                    ntt::<P2, 3>(&mut a2, true);
                },
                || {
                    ntt::<P3, 11>(&mut a3, false);
                    ntt::<P3, 11>(&mut b3, false);
                    for i in 0..n { a3[i] = mulmod(a3[i], b3[i], P3); }
                    ntt::<P3, 11>(&mut a3, true);
                },
            ),
        );
    }
    let mut res = vec![0u64; nc];
    for i in 0..nc {
        res[i] = crt.reconstruct(a1[i], a2[i], a3[i]);
    }
    res
}

fn poly_mul_trunc(a: &[u64], b: &[u64], trunc: usize, crt: &CrtHelper) -> Vec<u64> {
    let mut r = poly_mul(a, b, crt);
    r.truncate(trunc);
    r
}

fn poly_inv(f: &[u64], n: usize, crt: &CrtHelper) -> Vec<u64> {
    let mut g = vec![pw(f[0], MOD - 2, MOD)];
    let mut cur_len = 1;
    while cur_len < n {
        let next_len = std::cmp::min(cur_len * 2, n);
        let f_trunc: Vec<u64> = f.iter().take(next_len).copied().collect();
        let fg = poly_mul_trunc(&f_trunc, &g, next_len, crt);
        let mut h = vec![0u64; next_len];
        h[0] = (2 + MOD - fg[0]) % MOD;
        for i in 1..fg.len().min(next_len) {
            h[i] = if fg[i] == 0 { 0 } else { MOD - fg[i] };
        }
        g = poly_mul_trunc(&g, &h, next_len, crt);
        cur_len = next_len;
    }
    g.truncate(n);
    g
}

fn poly_mod_barrett(
    a: &[u64], d: usize,
    inv_rev_f_pre: &NttPre, inv_rev_f_ntt_sz: usize,
    cp_pre: &NttPre, cp_ntt_sz: usize,
    crt: &CrtHelper,
) -> Vec<u64> {
    if a.len() <= d {
        let mut res = vec![0u64; d];
        for i in 0..a.len() { res[i] = a[i]; }
        return res;
    }
    let deg_a = a.len() - 1;

    let mut rev_a = a.to_vec();
    rev_a.reverse();

    let q_len = deg_a - d + 1;
    let nc1 = rev_a.len() + (d - 1) - 1 + 1;
    let q_rev = poly_mul_with_pre(&rev_a, inv_rev_f_pre, inv_rev_f_ntt_sz, nc1, q_len, crt);

    let mut q = q_rev;
    while q.len() < q_len { q.push(0); }
    q.reverse();

    let nc2 = q.len() + d - 1;
    let qf_low = poly_mul_with_pre(&q, cp_pre, cp_ntt_sz, nc2, d, crt);

    let mut r = vec![0u64; d];
    for i in 0..d {
        let ai = if i < a.len() { a[i] } else { 0 };
        let qfi = if i < qf_low.len() { qf_low[i] } else { 0 };
        r[i] = (ai + MOD - qfi) % MOD;
    }
    r
}

fn berlekamp_massey(s: &[u64]) -> Vec<u64> {
    let n = s.len();
    let mut c_arr = vec![0u64; n + 1];
    let mut b_arr = vec![0u64; n + 1];
    c_arr[0] = 1;
    b_arr[0] = 1;
    let mut l_val = 0usize;
    let mut m = 1usize;
    let mut b = 1u64;

    for i in 0..n {
        let mut d_sum = s[i] as u128;
        for j in 1..=l_val {
            d_sum += c_arr[j] as u128 * s[i - j] as u128;
        }
        let d = (d_sum % MOD as u128) as u64;

        if d == 0 {
            m += 1;
        } else if 2 * l_val <= i {
            let t_arr: Vec<u64> = c_arr.clone();
            let coeff = mulmod(d, pw(b, MOD - 2, MOD), MOD);
            let mut blen = 0;
            for j in (0..b_arr.len()).rev() {
                if b_arr[j] != 0 { blen = j + 1; break; }
            }
            let needed = blen + m;
            if needed > c_arr.len() { c_arr.resize(needed, 0); }
            for j in 0..blen {
                c_arr[j + m] = (c_arr[j + m] + MOD - mulmod(coeff, b_arr[j], MOD)) % MOD;
            }
            l_val = i + 1 - l_val;
            b_arr = t_arr;
            b = d;
            m = 1;
        } else {
            let coeff = mulmod(d, pw(b, MOD - 2, MOD), MOD);
            let mut blen = 0;
            for j in (0..b_arr.len()).rev() {
                if b_arr[j] != 0 { blen = j + 1; break; }
            }
            let needed = blen + m;
            if needed > c_arr.len() { c_arr.resize(needed, 0); }
            for j in 0..blen {
                c_arr[j + m] = (c_arr[j + m] + MOD - mulmod(coeff, b_arr[j], MOD)) % MOD;
            }
            m += 1;
        }
    }

    let mut rec = vec![0u64; l_val];
    for i in 0..l_val {
        rec[i] = if c_arr[i + 1] == 0 { 0 } else { MOD - c_arr[i + 1] };
    }
    rec
}

fn main() {
    let n_target: u64 = 1_000_000_000_000_000_000;

    // Phase 1: Discover states via BFS
    let mut state_map = vec![-1i32; 1024 * 11];
    let mut states_mask = Vec::new();
    let mut states_s = Vec::new();
    let mut queue = Vec::new();

    let init = pack(1, 0);
    state_map[init] = 0;
    states_mask.push(1usize);
    states_s.push(0usize);
    queue.push(init);
    let mut qfront = 0;

    while qfront < queue.len() {
        let p = queue[qfront];
        qfront += 1;
        let mask = p / 11;
        let s = p % 11;

        for d in 0..B_DIGIT {
            if d > B_DIGIT - s { break; }
            let new_suf = ((mask << d) & ((1 << B_DIGIT) - 1)) | 1;
            let new_s = if mask & (1 << (B_DIGIT - d)) != 0 { 0 } else { d + s };

            let np = pack(new_suf, new_s);
            if state_map[np] == -1 {
                state_map[np] = states_mask.len() as i32;
                states_mask.push(new_suf);
                states_s.push(new_s);
                if states_mask.len() >= MAX_STATES {
                    panic!("Too many states");
                }
                queue.push(np);
            }
        }
    }

    let nstates = states_mask.len();

    // Phase 2: Build transition list
    let mut trans_list = Vec::new();
    let mut trans_offset = vec![0usize; nstates + 1];

    for i in 0..nstates {
        trans_offset[i] = trans_list.len();
        let mask = states_mask[i];
        let s = states_s[i];

        for d in 0..B_DIGIT {
            if d > B_DIGIT - s { break; }
            let new_suf = ((mask << d) & ((1 << B_DIGIT) - 1)) | 1;
            let new_s = if mask & (1 << (B_DIGIT - d)) != 0 { 0 } else { d + s };
            let np = pack(new_suf, new_s);
            let j = state_map[np];
            if j >= 0 {
                trans_list.push(j as usize);
            }
        }
    }
    trans_offset[nstates] = trans_list.len();

    // Target: states with s == 0
    let target_indices: Vec<usize> = (0..nstates).filter(|&i| states_s[i] == 0).collect();

    // Phase 3: Generate sequence via sparse matrix-vector multiply
    let mut cur = vec![0u64; nstates];
    let init_idx = state_map[pack(1, 0)] as usize;
    cur[init_idx] = 1;
    let mut seq = vec![0u64; NUM_TERMS];
    let mut nxt = vec![0u64; nstates];

    for t in 0..NUM_TERMS {
        let mut val = 0u64;
        for &i in &target_indices {
            val += cur[i];
        }
        val %= MOD;
        seq[t] = val;

        for v in nxt.iter_mut() { *v = 0; }
        for i in 0..nstates {
            let c = cur[i];
            if c == 0 { continue; }
            for tt in trans_offset[i]..trans_offset[i + 1] {
                let j = trans_list[tt];
                let sum = nxt[j] + c;
                nxt[j] = if sum >= MOD { sum - MOD } else { sum };
            }
        }
        std::mem::swap(&mut cur, &mut nxt);
    }

    // Phase 4: Berlekamp-Massey
    let rec = berlekamp_massey(&seq);
    let l = rec.len();

    let crt = CrtHelper::new();

    // Phase 5: Polynomial exponentiation using NTT-based Barrett reduction
    let mut cp = vec![0u64; l];
    for i in 0..l {
        cp[i] = if rec[l - 1 - i] == 0 { 0 } else { MOD - rec[l - 1 - i] };
    }

    let mut rev_f = vec![0u64; l + 1];
    rev_f[0] = 1;
    for i in 0..l {
        rev_f[i + 1] = if rec[i] == 0 { 0 } else { MOD - rec[i] };
    }
    let inv_rev_f = poly_inv(&rev_f, l, &crt);

    // Precompute NTT transforms
    let nc1 = 3 * l - 3;
    let ntt_sz1 = ntt_size(nc1);
    let inv_rev_f_pre = NttPre::new(&inv_rev_f, ntt_sz1);

    let nc2 = 2 * l - 2;
    let ntt_sz2 = ntt_size(nc2);
    let cp_pre = NttPre::new(&cp, ntt_sz2);

    let mut base_poly = vec![0u64; l];
    let mut result_poly = vec![0u64; l];
    if l > 1 { base_poly[1] = 1; }
    result_poly[0] = 1;

    let mut exp = n_target;
    while exp > 0 {
        if exp & 1 == 1 {
            let prod = poly_mul(&result_poly, &base_poly, &crt);
            result_poly = poly_mod_barrett(&prod, l, &inv_rev_f_pre, ntt_sz1, &cp_pre, ntt_sz2, &crt);
        }
        let prod = poly_mul(&base_poly, &base_poly, &crt);
        base_poly = poly_mod_barrett(&prod, l, &inv_rev_f_pre, ntt_sz1, &cp_pre, ntt_sz2, &crt);
        exp >>= 1;
    }

    let mut ans = 0u64;
    for i in 0..l {
        ans = (ans + mulmod(result_poly[i], seq[i], MOD)) % MOD;
    }

    // T(N) = seq[N] - 1
    ans = (ans + MOD - 1) % MOD;
    println!("{ans}");
}
