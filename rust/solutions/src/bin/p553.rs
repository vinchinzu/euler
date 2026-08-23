// Project Euler 553 - Power Sets of Power Sets
//
// C(10^4, 10) mod 10^9+7 via EGF polynomial ops.
// Inverse/exp by Newton; multiply by 3-prime NTT + CRT (MOD is not NTT-friendly).

const MOD: u64 = 1_000_000_007;
const P1: u64 = 998_244_353;
const P2: u64 = 985_661_441;
const P3: u64 = 754_974_721;
const MAX_LOG: usize = 15;
const MAX_NTT: usize = 1 << MAX_LOG; // 32768; two deg-10000 products fit
const NAIVE_LIMIT: usize = 32;

#[inline(always)]
fn mul_mod<const P: u64>(a: u64, b: u64) -> u64 {
    a * b % P
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

        // Twiddles for level k (half = 2^{k-1}) stored at [half .. 2*half).
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

    fn ntt(&self, a: &mut [u64], invert: bool) {
        let n = a.len();
        if n <= 1 {
            return;
        }
        let log_n = n.trailing_zeros() as usize;

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

        let tw = if invert { &self.inv_tw } else { &self.fwd_tw };

        let mut half = 1usize;
        for _level in 1..=log_n {
            let len = half << 1;
            let mut i = 0;
            while i < n {
                for jj in 0..half {
                    // SAFETY: i + jj + half < i + len <= n; tw[half + jj] precomputed
                    unsafe {
                        let wn = *tw.get_unchecked(half + jj);
                        let u = *a.get_unchecked(i + jj);
                        let v = mul_mod::<P>(*a.get_unchecked(i + jj + half), wn);
                        *a.get_unchecked_mut(i + jj) = if u + v >= P { u + v - P } else { u + v };
                        *a.get_unchecked_mut(i + jj + half) =
                            if u >= v { u - v } else { u + P - v };
                    }
                }
                i += len;
            }
            half = len;
        }

        if invert {
            let inv_n = self.inv_pow2[log_n];
            for x in a.iter_mut() {
                *x = mul_mod::<P>(*x, inv_n);
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
        CrtCtx {
            inv_p1_mod_p2: pow_mod::<P2>(P1 % P2, P2 - 2),
            inv_m12_mod_p3: pow_mod::<P3>((P1 as u128 * P2 as u128 % P3 as u128) as u64, P3 - 2),
            m12: P1 as u128 * P2 as u128,
        }
    }

    #[inline(always)]
    fn crt3_mod(&self, r1: u64, r2: u64, r3: u64) -> u64 {
        let r1p2 = r1 % P2;
        let diff = if r2 >= r1p2 { r2 - r1p2 } else { r2 + P2 - r1p2 };
        let k = mul_mod::<P2>(diff, self.inv_p1_mod_p2);
        let x12 = r1 as u128 + k as u128 * P1 as u128;
        let x12_mod_p3 = (x12 % P3 as u128) as u64;
        let diff2 = if r3 >= x12_mod_p3 {
            r3 - x12_mod_p3
        } else {
            r3 + P3 - x12_mod_p3
        };
        let k2 = mul_mod::<P3>(diff2, self.inv_m12_mod_p3);
        ((x12 + k2 as u128 * self.m12) % MOD as u128) as u64
    }
}

fn ntt_one<const P: u64>(ntt: &NttPrime<P>, a: &[u64], b: &[u64], fa: &mut [u64], fb: &mut [u64]) {
    let n = fa.len();
    let na = a.len();
    let nb = b.len();
    for i in 0..na {
        fa[i] = a[i] % P;
    }
    fa[na..].fill(0);
    for i in 0..nb {
        fb[i] = b[i] % P;
    }
    fb[nb..].fill(0);
    ntt.ntt(fa, false);
    ntt.ntt(fb, false);
    for i in 0..n {
        // SAFETY: fa, fb length n
        unsafe {
            *fa.get_unchecked_mut(i) = mul_mod::<P>(*fa.get_unchecked(i), *fb.get_unchecked(i));
        }
    }
    ntt.ntt(fa, true);
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
            out[i + j] = (out[i + j] + ai * b[j]) % MOD;
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
            ntt3: NttPrime::<P3>::new(11),
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
                let mut tmp = vec![0u64; take];
                poly_mul_naive(a, b, &mut tmp);
                out[..take].copy_from_slice(&tmp);
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

        // Three independent NTTs; only pay rayon overhead on large transforms.
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

        for i in 0..take {
            // SAFETY: inverse NTT results sit at the start of each prime's first buffer
            unsafe {
                let r1 = *self.buf.get_unchecked(i);
                let r2 = *self.buf.get_unchecked(2 * MAX_NTT + i);
                let r3 = *self.buf.get_unchecked(4 * MAX_NTT + i);
                *out.get_unchecked_mut(i) = self.crt.crt3_mod(r1, r2, r3);
            }
        }
        if take < out.len() {
            out[take..].fill(0);
        }
    }

    fn mul_vec(&mut self, a: &[u64], b: &[u64], len: usize) -> Vec<u64> {
        let mut out = vec![0u64; len];
        self.mul(a, b, &mut out);
        out
    }

    /// Inverse of f mod x^n (n coefficients).
    fn inv(&mut self, f: &[u64], n: usize) -> Vec<u64> {
        let mut g = vec![0u64; n];
        g[0] = pow_mod_m(f[0], MOD - 2, MOD);
        let mut t = vec![0u64; n];
        let mut r = vec![0u64; n];
        let mut ng = vec![0u64; n];
        let mut m = 1usize;
        while m < n {
            let want = (2 * m).min(n);
            let f_len = want.min(f.len());
            self.mul(&f[..f_len], &g[..m], &mut t[..want]);
            r[0] = (2 + MOD - t[0]) % MOD;
            for i in 1..want {
                r[i] = if t[i] == 0 { 0 } else { MOD - t[i] };
            }
            self.mul(&g[..m], &r[..want], &mut ng[..want]);
            g[..want].copy_from_slice(&ng[..want]);
            m = want;
        }
        g
    }

    /// log(f) mod x^n. f[0] must be invertible.
    fn log(&mut self, f: &[u64], n: usize, inv_table: &[u64]) -> Vec<u64> {
        let mut df = vec![0u64; n];
        for i in 0..n - 1 {
            df[i] = f[i + 1] * (i as u64 + 1) % MOD;
        }
        let finv = self.inv(f, n);
        let mut quot = vec![0u64; n];
        self.mul(&df, &finv, &mut quot);
        let mut res = vec![0u64; n];
        for i in 0..n - 1 {
            res[i + 1] = quot[i] * inv_table[i + 1] % MOD;
        }
        res
    }

    /// exp(f) mod x^n. Requires f[0] == 0.
    fn exp(&mut self, f: &[u64], n: usize, inv_table: &[u64]) -> Vec<u64> {
        let mut g = vec![0u64; n];
        g[0] = 1;
        let mut m = 1usize;
        while m < n {
            let m2 = (2 * m).min(n);
            let ln_g = self.log(&g[..m2], m2, inv_table);
            let mut diff = vec![0u64; m2];
            for i in 0..m2 {
                let fi = if i < f.len() { f[i] } else { 0 };
                diff[i] = (fi + MOD - ln_g[i]) % MOD;
            }
            diff[0] = (diff[0] + 1) % MOD;
            let mut ng = vec![0u64; m2];
            self.mul(&g[..m], &diff, &mut ng);
            g[..m2].copy_from_slice(&ng);
            m = m2;
        }
        g
    }
}

fn main() {
    let n = 10_000usize;
    let k = 10usize;
    let mut ctx = PolyCtx::new();

    let mut fact = vec![0u64; n + 1];
    let mut inv_fact = vec![0u64; n + 1];
    fact[0] = 1;
    for i in 1..=n {
        fact[i] = fact[i - 1] * i as u64 % MOD;
    }
    inv_fact[n] = pow_mod_m(fact[n], MOD - 2, MOD);
    for i in (0..n).rev() {
        inv_fact[i] = inv_fact[i + 1] * (i as u64 + 1) % MOD;
    }

    let mut inv_table = vec![0u64; n + 1];
    inv_table[1] = 1;
    for i in 2..=n {
        inv_table[i] = (MOD - MOD / i as u64) * inv_table[(MOD % i as u64) as usize] % MOD;
    }

    // a_coeff[i] = 2^{2^i - 1} / i!
    let mut a_coeff = vec![0u64; n + 1];
    let mut t = 1u64;
    for i in 0..=n {
        a_coeff[i] = t * inv_fact[i] % MOD;
        t = t * t % MOD * 2 % MOD;
    }

    let mut e_neg = vec![0u64; n + 1];
    let mut e_pos = vec![0u64; n + 1];
    for i in 0..=n {
        e_neg[i] = if i % 2 == 0 {
            inv_fact[i]
        } else {
            (MOD - inv_fact[i]) % MOD
        };
        e_pos[i] = inv_fact[i];
    }

    let p = ctx.mul_vec(&a_coeff, &e_neg, n + 1);
    let logp = ctx.log(&p, n + 1, &inv_table);

    // h = logp shifted down by 1
    let mut h = vec![0u64; n + 1];
    for i in 0..n {
        h[i] = logp[i + 1];
    }

    let h0 = h[0];
    let h0_inv = pow_mod_m(h0, MOD - 2, MOD);
    let nk = n - k;
    let mut h_norm = vec![0u64; nk + 1];
    for i in 0..=nk {
        h_norm[i] = h[i] * h0_inv % MOD;
    }

    let log_h = ctx.log(&h_norm, nk + 1, &inv_table);

    let mut log_h_k = vec![0u64; nk + 1];
    for i in 0..=nk {
        log_h_k[i] = log_h[i] * k as u64 % MOD;
    }

    let h_pow = ctx.exp(&log_h_k, nk + 1, &inv_table);

    let h0k = pow_mod_m(h0, k as u64, MOD);
    let mut h_pow_scaled = vec![0u64; nk + 1];
    for i in 0..=nk {
        h_pow_scaled[i] = h_pow[i] * h0k % MOD;
    }

    let mut logpk = vec![0u64; n + 1];
    for i in k..=n {
        logpk[i] = h_pow_scaled[i - k];
    }

    let result = ctx.mul_vec(&logpk, &e_pos, n + 1);

    let ans = result[n] * fact[n] % MOD * inv_fact[k] % MOD;
    println!("{ans}");
}
