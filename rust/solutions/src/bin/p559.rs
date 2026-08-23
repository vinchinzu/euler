// Project Euler 559 - Permutation Matrices
//
// Compute Q(50000) mod 1000000123.
// Per-k generating function is a power-series inverse of (1/(jk)!)^n.
// Small series: O(m^2) DP. Large: Newton inverse via 3-prime NTT + CRT.
// k values are independent; rayon over k with per-thread NTT workspace.

use rayon::prelude::*;

const NVAL: usize = 50_000;
const MOD: u64 = 1_000_000_123;

const P1: u64 = 998_244_353;
const P2: u64 = 985_661_441;
const P3: u64 = 754_974_721;

const MAX_LOG: usize = 17;
const MAX_NTT: usize = 1 << MAX_LOG; // 131072
const DIRECT_THRESHOLD: usize = 256;

#[inline(always)]
fn mul_mod_u(a: u64, b: u64, m: u64) -> u64 {
    a * b % m
}

fn pow_mod_u(mut base: u64, mut exp: u64, m: u64) -> u64 {
    let mut result = 1u64;
    base %= m;
    while exp > 0 {
        if exp & 1 == 1 {
            result = mul_mod_u(result, base, m);
        }
        base = mul_mod_u(base, base, m);
        exp >>= 1;
    }
    result
}

struct NttPrime {
    p: u64,
    fwd_root: [u64; MAX_LOG + 1],
    inv_root: [u64; MAX_LOG + 1],
    inv_pow2: [u64; MAX_LOG + 1],
}

impl NttPrime {
    fn new(p: u64, g: u64) -> Self {
        let mut fwd_root = [0u64; MAX_LOG + 1];
        let mut inv_root = [0u64; MAX_LOG + 1];
        let mut inv_pow2 = [0u64; MAX_LOG + 1];

        let mut pm1 = p - 1;
        let mut v2 = 0usize;
        while pm1 % 2 == 0 {
            pm1 /= 2;
            v2 += 1;
        }

        let base = pow_mod_u(g, pm1, p);
        for k in 0..=v2.min(MAX_LOG) {
            fwd_root[k] = pow_mod_u(base, 1u64 << (v2 - k), p);
            inv_root[k] = pow_mod_u(fwd_root[k], p - 2, p);
        }

        let inv2 = pow_mod_u(2, p - 2, p);
        inv_pow2[0] = 1;
        for k in 1..=MAX_LOG {
            inv_pow2[k] = mul_mod_u(inv_pow2[k - 1], inv2, p);
        }

        NttPrime {
            p,
            fwd_root,
            inv_root,
            inv_pow2,
        }
    }

    fn ntt(&self, a: &mut [u64], invert: bool) {
        let n = a.len();
        if n == 1 {
            return;
        }
        let log_n = n.trailing_zeros() as usize;
        let p = self.p;

        let mut j = 0usize;
        for i in 1..n {
            let mut bit = n >> 1;
            while j & bit != 0 {
                j ^= bit;
                bit >>= 1;
            }
            j ^= bit;
            if i < j {
                a.swap(i, j);
            }
        }

        let roots = if invert { &self.inv_root } else { &self.fwd_root };

        let mut half = 1usize;
        for level in 1..=log_n {
            let len = half << 1;
            let w_base = roots[level];
            let mut i = 0;
            while i < n {
                let mut wn = 1u64;
                for jj in 0..half {
                    unsafe {
                        let u = *a.get_unchecked(i + jj);
                        let v = mul_mod_u(*a.get_unchecked(i + jj + half), wn, p);
                        *a.get_unchecked_mut(i + jj) = if u + v >= p { u + v - p } else { u + v };
                        *a.get_unchecked_mut(i + jj + half) =
                            if u >= v { u - v } else { u + p - v };
                    }
                    wn = mul_mod_u(wn, w_base, p);
                }
                i += len;
            }
            half = len;
        }

        if invert {
            let inv_n = self.inv_pow2[log_n];
            for x in a.iter_mut() {
                *x = mul_mod_u(*x, inv_n, p);
            }
        }
    }
}

struct CrtCtx {
    inv_p1_mod_p2: u64,
    inv_m12_mod_p3: u64,
    m12: u128,
}

impl CrtCtx {
    fn new() -> Self {
        let inv_p1_mod_p2 = pow_mod_u(P1 % P2, P2 - 2, P2);
        let m12 = P1 as u128 * P2 as u128;
        let m12_mod_p3 = (m12 % P3 as u128) as u64;
        let inv_m12_mod_p3 = pow_mod_u(m12_mod_p3, P3 - 2, P3);
        CrtCtx {
            inv_p1_mod_p2,
            inv_m12_mod_p3,
            m12,
        }
    }

    #[inline]
    fn crt3_mod(&self, r1: u64, r2: u64, r3: u64) -> u64 {
        let r1p2 = r1 % P2;
        let diff = if r2 >= r1p2 { r2 - r1p2 } else { r2 + P2 - r1p2 };
        let k = mul_mod_u(diff, self.inv_p1_mod_p2, P2);
        let x12 = r1 as u128 + k as u128 * P1 as u128;
        let x12_mod_p3 = (x12 % P3 as u128) as u64;
        let diff2 = if r3 >= x12_mod_p3 {
            r3 - x12_mod_p3
        } else {
            r3 + P3 - x12_mod_p3
        };
        let k2 = mul_mod_u(diff2, self.inv_m12_mod_p3, P3);
        ((x12 + k2 as u128 * self.m12) % MOD as u128) as u64
    }
}

struct Workspace {
    buf: Vec<u64>,
}

impl Workspace {
    fn new() -> Self {
        Workspace {
            buf: vec![0u64; 6 * MAX_NTT],
        }
    }

    fn get_pair(&mut self, prime_idx: usize, ntt_len: usize) -> (&mut [u64], &mut [u64]) {
        let base = prime_idx * 2 * MAX_NTT;
        let (left, right) = self.buf[base..base + 2 * MAX_NTT].split_at_mut(MAX_NTT);
        (&mut left[..ntt_len], &mut right[..ntt_len])
    }
}

fn poly_mul(
    a: &[u64],
    b: &[u64],
    out_len: usize,
    ntts: &[NttPrime; 3],
    crt: &CrtCtx,
    ws: &mut Workspace,
    out: &mut [u64],
) {
    let na = a.len();
    let nb = b.len();
    let need = na + nb - 1;
    let ntt_len = need.next_power_of_two();

    for pi in 0..3 {
        let p = ntts[pi].p;
        let (fa, fb) = ws.get_pair(pi, ntt_len);
        fa[..na].copy_from_slice(a);
        fa[na..].fill(0);
        fb[..nb].copy_from_slice(b);
        fb[nb..].fill(0);
        ntts[pi].ntt(fa, false);
        ntts[pi].ntt(fb, false);
        for i in 0..ntt_len {
            unsafe {
                *fa.get_unchecked_mut(i) =
                    mul_mod_u(*fa.get_unchecked(i), *fb.get_unchecked(i), p);
            }
        }
        ntts[pi].ntt(fa, true);
    }

    let take = out_len.min(need);
    for i in 0..take {
        unsafe {
            let r1 = *ws.buf.get_unchecked(i);
            let r2 = *ws.buf.get_unchecked(2 * MAX_NTT + i);
            let r3 = *ws.buf.get_unchecked(4 * MAX_NTT + i);
            *out.get_unchecked_mut(i) = crt.crt3_mod(r1, r2, r3);
        }
    }
    if take < out_len {
        out[take..].fill(0);
    }
}

fn poly_inv(
    f: &[u64],
    n: usize,
    ntts: &[NttPrime; 3],
    crt: &CrtCtx,
    ws: &mut Workspace,
) -> Vec<u64> {
    let mut g = vec![0u64; n];
    g[0] = 1;
    let mut t = vec![0u64; n];
    let mut r = vec![0u64; n];
    let mut ng = vec![0u64; n];
    let mut m = 1usize;
    while m < n {
        let want = (2 * m).min(n);
        poly_mul(&f[..want], &g[..m], want, ntts, crt, ws, &mut t[..want]);
        r[0] = (2 + MOD - t[0]) % MOD;
        for i in 1..want {
            r[i] = if t[i] == 0 { 0 } else { MOD - t[i] };
        }
        poly_mul(&g[..m], &r[..want], want, ntts, crt, ws, &mut ng[..want]);
        g[..want].copy_from_slice(&ng[..want]);
        m = want;
    }
    g
}

fn naive_inverse(bk: &[u64], q: usize) -> Vec<u64> {
    let mut dp = vec![0u64; q + 1];
    dp[0] = 1;
    unsafe {
        let bk_p = bk.as_ptr();
        let dp_p = dp.as_mut_ptr();
        for i in 1..=q {
            let mut acc = 0u128;
            let mut j = 1usize;
            while j + 3 <= i {
                acc += *bk_p.add(j) as u128 * *dp_p.add(i - j) as u128;
                acc += *bk_p.add(j + 1) as u128 * *dp_p.add(i - j - 1) as u128;
                acc += *bk_p.add(j + 2) as u128 * *dp_p.add(i - j - 2) as u128;
                acc += *bk_p.add(j + 3) as u128 * *dp_p.add(i - j - 3) as u128;
                j += 4;
            }
            while j <= i {
                acc += *bk_p.add(j) as u128 * *dp_p.add(i - j) as u128;
                j += 1;
            }
            let v = (acc % MOD as u128) as u64;
            *dp_p.add(i) = if v == 0 { 0 } else { MOD - v };
        }
    }
    dp
}

fn compute_pk(
    k: usize,
    n: usize,
    pif: &[u64],
    ntts: &[NttPrime; 3],
    crt: &CrtCtx,
    ws: &mut Option<Workspace>,
) -> u64 {
    let q = n / k;
    let r = n % k;

    let mut bk = vec![0u64; q + 1];
    unsafe {
        *bk.get_unchecked_mut(0) = 1;
        for j in 1..=q {
            *bk.get_unchecked_mut(j) = *pif.get_unchecked(j * k);
        }
    }

    let dp = if q <= DIRECT_THRESHOLD {
        naive_inverse(&bk, q)
    } else {
        let ws = ws.get_or_insert_with(Workspace::new);
        poly_inv(&bk, q + 1, ntts, crt, ws)
    };

    let last = if r == 0 {
        dp[q]
    } else {
        let mut acc = 0u128;
        unsafe {
            for s in 0..=q {
                acc += *pif.get_unchecked(n - s * k) as u128 * *dp.get_unchecked(s) as u128;
            }
        }
        let v = (acc % MOD as u128) as u64;
        if v == 0 { 0 } else { MOD - v }
    };

    let np = if r == 0 { q + 1 } else { q + 2 };
    if (np + 1) % 2 == 0 {
        last
    } else if last == 0 {
        0
    } else {
        MOD - last
    }
}

fn main() {
    let n = NVAL;

    let mut factorials = vec![0u64; n + 1];
    factorials[0] = 1;
    for i in 1..=n {
        factorials[i] = factorials[i - 1] * i as u64 % MOD;
    }

    let mut inv_factorials = vec![0u64; n + 1];
    inv_factorials[n] = pow_mod_u(factorials[n], MOD - 2, MOD);
    for i in (0..n).rev() {
        inv_factorials[i] = inv_factorials[i + 1] * (i as u64 + 1) % MOD;
    }

    let mut pow_inv_fact = vec![0u64; n + 1];
    pow_inv_fact
        .par_iter_mut()
        .enumerate()
        .for_each(|(i, slot)| {
            *slot = pow_mod_u(inv_factorials[i], n as u64, MOD);
        });

    let ntts = [
        NttPrime::new(P1, 3),
        NttPrime::new(P2, 3),
        NttPrime::new(P3, 11),
    ];
    let crt = CrtCtx::new();
    let pif = &pow_inv_fact[..];

    let pk_sum: u64 = (1..n + 1)
        .into_par_iter()
        .map_init(
            || None::<Workspace>,
            |ws, k| compute_pk(k, n, pif, &ntts, &crt, ws),
        )
        .sum();

    let ans = (pk_sum % MOD) * pow_mod_u(factorials[n], n as u64, MOD) % MOD;
    println!("{ans}");
}
