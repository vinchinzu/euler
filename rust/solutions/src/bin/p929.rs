// Project Euler Problem 929 - Compositions with Odd-Length Runs
// F(10^5) mod 1111124111
// 3-prime NTT + CRT, Newton inverse of (1 - H).
// h[m] = sum_{d|m} (-1)^{d-1} F_d

use rayon::prelude::*;

const FINAL_MOD: u64 = 1_111_124_111;
const P1: u64 = 998_244_353;
const P2: u64 = 1_004_535_809;
const P3: u64 = 469_762_049;
const MAX_LOG: usize = 18;
const MAX_NTT: usize = 1 << MAX_LOG;
const NAIVE_LIMIT: usize = 64;
const PAR_N: usize = 4096;
const GRAIN: usize = 512;

#[inline(always)]
fn mul_mod<const P: u64>(a: u64, b: u64) -> u64 {
    a * b % P
}

#[inline(always)]
fn add_mod<const P: u64>(a: u64, b: u64) -> u64 {
    let s = a + b;
    if s >= P { s - P } else { s }
}

#[inline(always)]
fn sub_mod<const P: u64>(a: u64, b: u64) -> u64 {
    if a >= b { a - b } else { a + P - b }
}

fn pow_mod<const P: u64>(mut base: u64, mut exp: u64) -> u64 {
    let mut r = 1u64;
    base %= P;
    while exp > 0 {
        if exp & 1 == 1 {
            r = mul_mod::<P>(r, base);
        }
        base = mul_mod::<P>(base, base);
        exp >>= 1;
    }
    r
}

fn pow_mod_m(mut base: u64, mut exp: u64, m: u64) -> u64 {
    let mut r = 1u64;
    base %= m;
    while exp > 0 {
        if exp & 1 == 1 {
            r = r * base % m;
        }
        base = base * base % m;
        exp >>= 1;
    }
    r
}

struct NttPrime<const P: u64> {
    fwd_tw: Vec<u64>,
    inv_tw: Vec<u64>,
    inv_pow2: [u64; MAX_LOG + 1],
}

#[inline(always)]
fn dif_leaf<const P: u64>(left: &mut [u64], right: &mut [u64], tw: &[u64]) {
    for i in 0..left.len() {
        // SAFETY: left, right, tw are the same length; indices from split NTT block
        unsafe {
            let u = *left.get_unchecked(i);
            let v = *right.get_unchecked(i);
            let wn = *tw.get_unchecked(i);
            *left.get_unchecked_mut(i) = add_mod::<P>(u, v);
            *right.get_unchecked_mut(i) = mul_mod::<P>(sub_mod::<P>(u, v), wn);
        }
    }
}

fn dif_range_par<const P: u64>(left: &mut [u64], right: &mut [u64], tw: &[u64]) {
    let n = left.len();
    if n <= GRAIN {
        dif_leaf::<P>(left, right, tw);
        return;
    }
    let mid = n >> 1;
    let (l0, l1) = left.split_at_mut(mid);
    let (r0, r1) = right.split_at_mut(mid);
    let (t0, t1) = tw.split_at(mid);
    rayon::join(
        || dif_range_par::<P>(l0, r0, t0),
        || dif_range_par::<P>(l1, r1, t1),
    );
}

#[inline(always)]
fn dit_leaf<const P: u64>(left: &mut [u64], right: &mut [u64], tw: &[u64]) {
    for i in 0..left.len() {
        // SAFETY: left, right, tw are the same length; indices from split NTT block
        unsafe {
            let u = *left.get_unchecked(i);
            let v = mul_mod::<P>(*right.get_unchecked(i), *tw.get_unchecked(i));
            *left.get_unchecked_mut(i) = add_mod::<P>(u, v);
            *right.get_unchecked_mut(i) = sub_mod::<P>(u, v);
        }
    }
}

fn dit_range_par<const P: u64>(left: &mut [u64], right: &mut [u64], tw: &[u64]) {
    let n = left.len();
    if n <= GRAIN {
        dit_leaf::<P>(left, right, tw);
        return;
    }
    let mid = n >> 1;
    let (l0, l1) = left.split_at_mut(mid);
    let (r0, r1) = right.split_at_mut(mid);
    let (t0, t1) = tw.split_at(mid);
    rayon::join(
        || dit_range_par::<P>(l0, r0, t0),
        || dit_range_par::<P>(l1, r1, t1),
    );
}

fn dif_block<const P: u64>(block: &mut [u64], half: usize, tw: &[u64]) {
    let (left, right) = block.split_at_mut(half);
    dif_leaf::<P>(left, right, &tw[half..half + half]);
}

fn dit_block<const P: u64>(block: &mut [u64], half: usize, tw: &[u64]) {
    let (left, right) = block.split_at_mut(half);
    dit_leaf::<P>(left, right, &tw[half..half + half]);
}

impl<const P: u64> NttPrime<P> {
    fn new(g: u64) -> Self {
        let mut fwd_root = [0u64; MAX_LOG + 1];
        let mut inv_root = [0u64; MAX_LOG + 1];
        let mut inv_pow2 = [0u64; MAX_LOG + 1];

        let mut pm1 = P - 1;
        let mut v2 = 0usize;
        while pm1 % 2 == 0 {
            pm1 /= 2;
            v2 += 1;
        }

        let base = pow_mod::<P>(g, pm1);
        for k in 0..=v2.min(MAX_LOG) {
            fwd_root[k] = pow_mod::<P>(base, 1u64 << (v2 - k));
            inv_root[k] = pow_mod::<P>(fwd_root[k], P - 2);
        }

        let inv2 = pow_mod::<P>(2, P - 2);
        inv_pow2[0] = 1;
        for k in 1..=MAX_LOG {
            inv_pow2[k] = mul_mod::<P>(inv_pow2[k - 1], inv2);
        }

        let mut fwd_tw = vec![0u64; MAX_NTT];
        let mut inv_tw = vec![0u64; MAX_NTT];
        for level in 1..=MAX_LOG {
            let half = 1usize << (level - 1);
            let mut wn_f = 1u64;
            let mut wn_i = 1u64;
            for jj in 0..half {
                fwd_tw[half + jj] = wn_f;
                inv_tw[half + jj] = wn_i;
                wn_f = mul_mod::<P>(wn_f, fwd_root[level]);
                wn_i = mul_mod::<P>(wn_i, inv_root[level]);
            }
        }

        NttPrime {
            fwd_tw,
            inv_tw,
            inv_pow2,
        }
    }

    /// Decimation-in-frequency; normal order in, bit-reversed out.
    fn ntt_dif(&self, a: &mut [u64]) {
        let n = a.len();
        if n <= 1 {
            return;
        }
        let tw = &self.fwd_tw;
        let use_par = n >= PAR_N;
        let mut len = n;
        while len >= 2 {
            let half = len >> 1;
            let nblocks = n / len;
            if use_par && nblocks < 8 && half >= GRAIN {
                a.chunks_mut(len).for_each(|block| {
                    let (left, right) = block.split_at_mut(half);
                    dif_range_par::<P>(left, right, &tw[half..half + half]);
                });
            } else if use_par && nblocks >= 8 {
                a.par_chunks_mut(len).for_each(|block| {
                    dif_block::<P>(block, half, tw);
                });
            } else {
                let mut i = 0;
                while i < n {
                    dif_block::<P>(&mut a[i..i + len], half, tw);
                    i += len;
                }
            }
            len = half;
        }
    }

    /// Decimation-in-time inverse; bit-reversed in, normal order out.
    fn ntt_dit_inv(&self, a: &mut [u64]) {
        let n = a.len();
        if n <= 1 {
            return;
        }
        let log_n = n.trailing_zeros() as usize;
        let tw = &self.inv_tw;
        let use_par = n >= PAR_N;
        let mut half = 1usize;
        for _level in 1..=log_n {
            let len = half << 1;
            let nblocks = n / len;
            if use_par && nblocks < 8 && half >= GRAIN {
                a.chunks_mut(len).for_each(|block| {
                    let (left, right) = block.split_at_mut(half);
                    dit_range_par::<P>(left, right, &tw[half..half + half]);
                });
            } else if use_par && nblocks >= 8 {
                a.par_chunks_mut(len).for_each(|block| {
                    dit_block::<P>(block, half, tw);
                });
            } else {
                let mut i = 0;
                while i < n {
                    dit_block::<P>(&mut a[i..i + len], half, tw);
                    i += len;
                }
            }
            half = len;
        }
        let inv_n = self.inv_pow2[log_n];
        if n >= PAR_N {
            a.par_iter_mut().for_each(|x| *x = mul_mod::<P>(*x, inv_n));
        } else {
            for x in a.iter_mut() {
                *x = mul_mod::<P>(*x, inv_n);
            }
        }
    }
}

struct CrtCtx {
    inv_p1_mod_p2: u64,
    inv_m12_mod_p3: u64,
    m12_mod: u64,
}

impl CrtCtx {
    fn new() -> Self {
        CrtCtx {
            inv_p1_mod_p2: pow_mod::<P2>(P1 % P2, P2 - 2),
            inv_m12_mod_p3: pow_mod::<P3>((P1 as u128 * P2 as u128 % P3 as u128) as u64, P3 - 2),
            m12_mod: ((P1 as u128 * P2 as u128) % FINAL_MOD as u128) as u64,
        }
    }

    // r1 < P1 < P2, so r1 % P2 = r1. P1*P2 fits u64.
    #[inline(always)]
    fn crt3_mod(&self, r1: u64, r2: u64, r3: u64) -> u64 {
        let diff = if r2 >= r1 { r2 - r1 } else { r2 + P2 - r1 };
        let k = mul_mod::<P2>(diff, self.inv_p1_mod_p2);
        let x12 = r1 + k * P1;
        let x12_mod_p3 = x12 % P3;
        let diff2 = if r3 >= x12_mod_p3 {
            r3 - x12_mod_p3
        } else {
            r3 + P3 - x12_mod_p3
        };
        let k2 = mul_mod::<P3>(diff2, self.inv_m12_mod_p3);
        let t1 = x12 % FINAL_MOD;
        let t2 = k2 * self.m12_mod % FINAL_MOD;
        let s = t1 + t2;
        if s >= FINAL_MOD { s - FINAL_MOD } else { s }
    }
}

fn ntt_one<const P: u64>(ntt: &NttPrime<P>, a: &[u64], b: &[u64], fa: &mut [u64], fb: &mut [u64]) {
    let n = fa.len();
    let na = a.len().min(n);
    let nb = b.len().min(n);
    for i in 0..na {
        fa[i] = a[i] % P;
    }
    fa[na..].fill(0);
    for i in 0..nb {
        fb[i] = b[i] % P;
    }
    fb[nb..].fill(0);
    if n >= PAR_N {
        rayon::join(|| ntt.ntt_dif(fa), || ntt.ntt_dif(fb));
    } else {
        ntt.ntt_dif(fa);
        ntt.ntt_dif(fb);
    }
    for i in 0..n {
        // SAFETY: fa, fb length n
        unsafe {
            *fa.get_unchecked_mut(i) = mul_mod::<P>(*fa.get_unchecked(i), *fb.get_unchecked(i));
        }
    }
    ntt.ntt_dit_inv(fa);
}

fn poly_mul_naive(a: &[u64], b: &[u64], out: &mut [u64]) {
    out.fill(0);
    let na = a.len();
    let nb = b.len();
    let nout = out.len();
    for i in 0..na {
        let ai = a[i];
        if ai == 0 {
            continue;
        }
        let jmax = (nout - i).min(nb);
        for j in 0..jmax {
            out[i + j] = (out[i + j] + ai * b[j]) % FINAL_MOD;
        }
    }
}

struct PolyCtx {
    ntt1: NttPrime<P1>,
    ntt2: NttPrime<P2>,
    ntt3: NttPrime<P3>,
    crt: CrtCtx,
    buf: Vec<u64>,
}

impl PolyCtx {
    fn new() -> Self {
        PolyCtx {
            ntt1: NttPrime::<P1>::new(3),
            ntt2: NttPrime::<P2>::new(3),
            ntt3: NttPrime::<P3>::new(3),
            crt: CrtCtx::new(),
            buf: vec![0u64; 6 * MAX_NTT],
        }
    }

    fn mul(&mut self, a: &[u64], b: &[u64], out: &mut [u64]) {
        let na = a.len();
        let nb = b.len();
        if na == 0 || nb == 0 {
            out.fill(0);
            return;
        }
        let need = na + nb - 1;
        let take = out.len().min(need);

        if take <= NAIVE_LIMIT || na.min(nb) <= 8 {
            if take == out.len() {
                poly_mul_naive(a, b, out);
            } else {
                poly_mul_naive(a, b, &mut out[..take]);
                out[take..].fill(0);
            }
            return;
        }

        let ntt_len = need.next_power_of_two();
        debug_assert!(ntt_len <= MAX_NTT);

        let ntt1 = &self.ntt1;
        let ntt2 = &self.ntt2;
        let ntt3 = &self.ntt3;
        let (s0, rest) = self.buf.split_at_mut(2 * MAX_NTT);
        let (s1, s2) = rest.split_at_mut(2 * MAX_NTT);
        let (fa0, fb0) = s0.split_at_mut(MAX_NTT);
        let (fa1, fb1) = s1.split_at_mut(MAX_NTT);
        let (fa2, fb2) = s2.split_at_mut(MAX_NTT);
        let fa0 = &mut fa0[..ntt_len];
        let fb0 = &mut fb0[..ntt_len];
        let fa1 = &mut fa1[..ntt_len];
        let fb1 = &mut fb1[..ntt_len];
        let fa2 = &mut fa2[..ntt_len];
        let fb2 = &mut fb2[..ntt_len];

        if ntt_len >= 2048 {
            rayon::join(
                || ntt_one::<P1>(ntt1, a, b, fa0, fb0),
                || {
                    rayon::join(
                        || ntt_one::<P2>(ntt2, a, b, fa1, fb1),
                        || ntt_one::<P3>(ntt3, a, b, fa2, fb2),
                    )
                },
            );
        } else {
            ntt_one::<P1>(ntt1, a, b, fa0, fb0);
            ntt_one::<P2>(ntt2, a, b, fa1, fb1);
            ntt_one::<P3>(ntt3, a, b, fa2, fb2);
        }

        if take >= PAR_N {
            let buf = &self.buf;
            let crt = &self.crt;
            out[..take].par_iter_mut().enumerate().for_each(|(i, slot)| {
                // SAFETY: inverse NTT results sit at the start of each prime's first buffer
                unsafe {
                    let r1 = *buf.get_unchecked(i);
                    let r2 = *buf.get_unchecked(2 * MAX_NTT + i);
                    let r3 = *buf.get_unchecked(4 * MAX_NTT + i);
                    *slot = crt.crt3_mod(r1, r2, r3);
                }
            });
        } else {
            for i in 0..take {
                // SAFETY: inverse NTT results sit at the start of each prime's first buffer
                unsafe {
                    let r1 = *self.buf.get_unchecked(i);
                    let r2 = *self.buf.get_unchecked(2 * MAX_NTT + i);
                    let r3 = *self.buf.get_unchecked(4 * MAX_NTT + i);
                    *out.get_unchecked_mut(i) = self.crt.crt3_mod(r1, r2, r3);
                }
            }
        }
        if take < out.len() {
            out[take..].fill(0);
        }
    }

    fn inv(&mut self, f: &[u64], n: usize) -> Vec<u64> {
        let mut g = vec![0u64; n];
        g[0] = pow_mod_m(f[0], FINAL_MOD - 2, FINAL_MOD);
        let mut t = vec![0u64; n];
        let mut r = vec![0u64; n];
        let mut ng = vec![0u64; n];
        let mut m = 1usize;
        while m < n {
            let want = (2 * m).min(n);
            let f_len = want.min(f.len());
            self.mul(&f[..f_len], &g[..m], &mut t[..want]);
            r[0] = (2 + FINAL_MOD - t[0]) % FINAL_MOD;
            for i in 1..want {
                r[i] = if t[i] == 0 { 0 } else { FINAL_MOD - t[i] };
            }
            self.mul(&g[..m], &r[..want], &mut ng[..want]);
            g[..want].copy_from_slice(&ng[..want]);
            m = want;
        }
        g
    }
}

fn main() {
    let n = 100_000usize;

    let mut f = vec![0u64; n + 1];
    f[1] = 1;
    if n >= 2 {
        f[2] = 1;
    }
    for i in 3..=n {
        let s = f[i - 1] + f[i - 2];
        f[i] = if s >= FINAL_MOD { s - FINAL_MOD } else { s };
    }

    let mut h = vec![0u64; n + 1];
    for d in 1..=n {
        let val = if d & 1 == 0 {
            (FINAL_MOD - f[d]) % FINAL_MOD
        } else {
            f[d]
        };
        let mut m = d;
        while m <= n {
            let s = h[m] + val;
            h[m] = if s >= FINAL_MOD { s - FINAL_MOD } else { s };
            m += d;
        }
    }

    let mut p = vec![0u64; n + 1];
    p[0] = 1;
    for i in 1..=n {
        p[i] = if h[i] == 0 { 0 } else { FINAL_MOD - h[i] };
    }

    let mut ctx = PolyCtx::new();
    let q = ctx.inv(&p, n + 1);
    println!("{}", q[n]);
}
