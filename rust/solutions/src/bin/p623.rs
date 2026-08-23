// Project Euler 623 - Lambda Terms
//
// F_b(x) = b x + x^5 F_{b+1}(x) + x^2 F_b(x)^2
// F_b = (1 - sqrt(1 - 4 x^2 C)) / (2 x^2),  C = b x + x^5 F_{b+1}
// Sqrt/Newton via 3-prime NTT + CRT (MOD is not NTT-friendly).

const NN: usize = 2000;
const MOD: u64 = 1_000_000_007;
const MAXB: usize = 401;
const P1: u64 = 998_244_353;
const P2: u64 = 985_661_441;
const P3: u64 = 754_974_721;
const MAX_LOG: usize = 12;
const MAX_NTT: usize = 1 << MAX_LOG;
const POLY: usize = 2048;
const NAIVE_LIMIT: usize = 64;
const INV2: u64 = (MOD + 1) / 2;

#[inline(always)]
fn mul_mod<const P: u64>(a: u64, b: u64) -> u64 {
    a * b % P
}

#[inline(always)]
fn add_mod<const P: u64>(a: u64, b: u64) -> u64 {
    let s = a + b;
    if s >= P {
        s - P
    } else {
        s
    }
}

#[inline(always)]
fn sub_mod<const P: u64>(a: u64, b: u64) -> u64 {
    if a >= b {
        a - b
    } else {
        a + P - b
    }
}

#[inline(always)]
fn add_m(a: u64, b: u64) -> u64 {
    add_mod::<MOD>(a, b)
}

#[inline(always)]
fn sub_m(a: u64, b: u64) -> u64 {
    sub_mod::<MOD>(a, b)
}

#[inline(always)]
fn mul_m(a: u64, b: u64) -> u64 {
    a * b % MOD
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
        if invert {
            self.ntt_dit_inv(a);
        } else {
            self.ntt_dif(a);
        }
    }

    /// Decimation-in-frequency; normal order in, bit-reversed out.
    fn ntt_dif(&self, a: &mut [u64]) {
        let n = a.len();
        let tw = &self.fwd_tw;
        let mut len = n;
        while len >= 2 {
            let half = len >> 1;
            let mut i = 0;
            while i < n {
                for jj in 0..half {
                    // SAFETY: i + jj + half < i + len <= n; tw[half + jj] precomputed
                    unsafe {
                        let wn = *tw.get_unchecked(half + jj);
                        let u = *a.get_unchecked(i + jj);
                        let v = *a.get_unchecked(i + jj + half);
                        *a.get_unchecked_mut(i + jj) = add_mod::<P>(u, v);
                        *a.get_unchecked_mut(i + jj + half) =
                            mul_mod::<P>(sub_mod::<P>(u, v), wn);
                    }
                }
                i += len;
            }
            len = half;
        }
    }

    /// Decimation-in-time inverse; bit-reversed in, normal order out.
    fn ntt_dit_inv(&self, a: &mut [u64]) {
        let n = a.len();
        let log_n = n.trailing_zeros() as usize;
        let tw = &self.inv_tw;
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
                        *a.get_unchecked_mut(i + jj) = add_mod::<P>(u, v);
                        *a.get_unchecked_mut(i + jj + half) = sub_mod::<P>(u, v);
                    }
                }
                i += len;
            }
            half = len;
        }
        let inv_n = self.inv_pow2[log_n];
        for x in a.iter_mut() {
            *x = mul_mod::<P>(*x, inv_n);
        }
    }
}

struct CrtCtx {
    inv_p1_mod_p2: u64,
    inv_m12_mod_p3: u64,
    m12_mod: u64,
    p1_mod_p3: u64,
}

impl CrtCtx {
    fn new() -> Self {
        CrtCtx {
            inv_p1_mod_p2: pow_mod::<P2>(P1 % P2, P2 - 2),
            inv_m12_mod_p3: pow_mod::<P3>((P1 as u128 * P2 as u128 % P3 as u128) as u64, P3 - 2),
            m12_mod: (P1 as u128 * P2 as u128 % MOD as u128) as u64,
            p1_mod_p3: P1 % P3,
        }
    }

    #[inline(always)]
    fn crt3_mod(&self, r1: u64, r2: u64, r3: u64) -> u64 {
        let r1p2 = if r1 >= P2 { r1 - P2 } else { r1 };
        let diff = if r2 >= r1p2 { r2 - r1p2 } else { r2 + P2 - r1p2 };
        let k = mul_mod::<P2>(diff, self.inv_p1_mod_p2);
        let x12_mod = add_m(r1, mul_m(k, P1));
        let mut r1p3 = r1;
        if r1p3 >= P3 {
            r1p3 -= P3;
        }
        if r1p3 >= P3 {
            r1p3 -= P3;
        }
        let kp3 = if k >= P3 { k - P3 } else { k };
        let x12_mod_p3 = add_mod::<P3>(r1p3, mul_mod::<P3>(kp3, self.p1_mod_p3));
        let diff2 = if r3 >= x12_mod_p3 {
            r3 - x12_mod_p3
        } else {
            r3 + P3 - x12_mod_p3
        };
        let k2 = mul_mod::<P3>(diff2, self.inv_m12_mod_p3);
        add_m(x12_mod, mul_m(k2, self.m12_mod))
    }
}

#[inline(always)]
fn red_p<const P: u64>(x: u64) -> u64 {
    if x >= P { x - P } else { x }
}

fn ntt_one<const P: u64>(ntt: &NttPrime<P>, a: &[u64], b: &[u64], fa: &mut [u64], fb: &mut [u64]) {
    let n = fa.len();
    let na = a.len();
    let nb = b.len();
    for i in 0..na {
        fa[i] = red_p::<P>(a[i]);
    }
    fa[na..].fill(0);
    for i in 0..nb {
        fb[i] = red_p::<P>(b[i]);
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

fn ntt_sq_one<const P: u64>(ntt: &NttPrime<P>, a: &[u64], fa: &mut [u64]) {
    let n = fa.len();
    let na = a.len();
    for i in 0..na {
        fa[i] = red_p::<P>(a[i]);
    }
    fa[na..].fill(0);
    ntt.ntt(fa, false);
    for i in 0..n {
        // SAFETY: fa length n
        unsafe {
            let x = *fa.get_unchecked(i);
            *fa.get_unchecked_mut(i) = mul_mod::<P>(x, x);
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

fn poly_mul(
    ntt1: &NttPrime<P1>,
    ntt2: &NttPrime<P2>,
    ntt3: &NttPrime<P3>,
    crt: &CrtCtx,
    buf: &mut [u64],
    a: &[u64],
    b: &[u64],
    out: &mut [u64],
) {
    let na = a.len();
    let nb = b.len();
    if na == 0 || nb == 0 {
        out.fill(0);
        return;
    }
    let need = na + nb - 1;
    let take = out.len().min(need);

    if take <= NAIVE_LIMIT || na.min(nb) <= 8 {
        poly_mul_naive(a, b, &mut out[..take]);
        if take < out.len() {
            out[take..].fill(0);
        }
        return;
    }

    let ntt_len = need.next_power_of_two();
    debug_assert!(ntt_len <= MAX_NTT);

    let (s0, rest) = buf.split_at_mut(2 * MAX_NTT);
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

    for i in 0..take {
        // SAFETY: inverse NTT results sit at the start of each prime's first buffer
        unsafe {
            let r1 = *buf.get_unchecked(i);
            let r2 = *buf.get_unchecked(2 * MAX_NTT + i);
            let r3 = *buf.get_unchecked(4 * MAX_NTT + i);
            *out.get_unchecked_mut(i) = crt.crt3_mod(r1, r2, r3);
        }
    }
    if take < out.len() {
        out[take..].fill(0);
    }
}

fn poly_square(
    ntt1: &NttPrime<P1>,
    ntt2: &NttPrime<P2>,
    ntt3: &NttPrime<P3>,
    crt: &CrtCtx,
    buf: &mut [u64],
    a: &[u64],
    out: &mut [u64],
) {
    let na = a.len();
    if na == 0 {
        out.fill(0);
        return;
    }
    let need = 2 * na - 1;
    let take = out.len().min(need);

    if take <= NAIVE_LIMIT || na <= 8 {
        poly_mul_naive(a, a, &mut out[..take]);
        if take < out.len() {
            out[take..].fill(0);
        }
        return;
    }

    let ntt_len = need.next_power_of_two();
    debug_assert!(ntt_len <= MAX_NTT);

    let (s0, rest) = buf.split_at_mut(2 * MAX_NTT);
    let (s1, s2) = rest.split_at_mut(2 * MAX_NTT);
    let fa0 = &mut s0[..ntt_len];
    let fa1 = &mut s1[..ntt_len];
    let fa2 = &mut s2[..ntt_len];

    if ntt_len >= 2048 {
        rayon::join(
            || ntt_sq_one::<P1>(ntt1, a, fa0),
            || {
                rayon::join(
                    || ntt_sq_one::<P2>(ntt2, a, fa1),
                    || ntt_sq_one::<P3>(ntt3, a, fa2),
                )
            },
        );
    } else {
        ntt_sq_one::<P1>(ntt1, a, fa0);
        ntt_sq_one::<P2>(ntt2, a, fa1);
        ntt_sq_one::<P3>(ntt3, a, fa2);
    }

    for i in 0..take {
        // SAFETY: inverse NTT results sit at the start of each prime's first buffer
        unsafe {
            let r1 = *buf.get_unchecked(i);
            let r2 = *buf.get_unchecked(2 * MAX_NTT + i);
            let r3 = *buf.get_unchecked(4 * MAX_NTT + i);
            *out.get_unchecked_mut(i) = crt.crt3_mod(r1, r2, r3);
        }
    }
    if take < out.len() {
        out[take..].fill(0);
    }
}

struct PolyCtx {
    ntt1: NttPrime<P1>,
    ntt2: NttPrime<P2>,
    ntt3: NttPrime<P3>,
    crt: CrtCtx,
    buf: Vec<u64>,
    t: Vec<u64>,
    r: Vec<u64>,
    ng: Vec<u64>,
    h: Vec<u64>,
}

impl PolyCtx {
    fn new() -> Self {
        PolyCtx {
            ntt1: NttPrime::<P1>::new(3),
            ntt2: NttPrime::<P2>::new(3),
            ntt3: NttPrime::<P3>::new(11),
            crt: CrtCtx::new(),
            buf: vec![0u64; 6 * MAX_NTT],
            t: vec![0u64; POLY],
            r: vec![0u64; POLY],
            ng: vec![0u64; POLY],
            h: vec![0u64; POLY],
        }
    }

    /// sqrt(f) mod x^n. Requires f[0] == 1.
    /// g_{2m} = g_m + ((f - g_m^2)/2) * h,  h = 1/g_m. Last doubling skips the h lift.
    fn sqrt_into(&mut self, f: &[u64], n: usize, out: &mut [u64]) {
        out[..n].fill(0);
        out[0] = 1;
        self.h[..n].fill(0);
        self.h[0] = 1;
        let mut m = 1usize;
        while m < n {
            let want = (2 * m).min(n);
            poly_square(
                &self.ntt1,
                &self.ntt2,
                &self.ntt3,
                &self.crt,
                &mut self.buf,
                &out[..m],
                &mut self.t[..want],
            );
            let dlen = want - m;
            for i in 0..dlen {
                let fi = if m + i < f.len() { f[m + i] } else { 0 };
                self.r[i] = mul_m(sub_m(fi, self.t[m + i]), INV2);
            }
            poly_mul(
                &self.ntt1,
                &self.ntt2,
                &self.ntt3,
                &self.crt,
                &mut self.buf,
                &self.r[..dlen],
                &self.h[..m],
                &mut self.ng[..dlen],
            );
            for i in 0..dlen {
                out[m + i] = self.ng[i];
            }

            m = want;
            if m >= n {
                break;
            }
            poly_mul(
                &self.ntt1,
                &self.ntt2,
                &self.ntt3,
                &self.crt,
                &mut self.buf,
                &out[..m],
                &self.h[..m / 2],
                &mut self.t[..m],
            );
            self.r[0] = sub_m(2, self.t[0]);
            for i in 1..m {
                self.r[i] = sub_m(0, self.t[i]);
            }
            poly_mul(
                &self.ntt1,
                &self.ntt2,
                &self.ntt3,
                &self.crt,
                &mut self.buf,
                &self.h[..m / 2],
                &self.r[..m],
                &mut self.ng[..m],
            );
            self.h[..m].copy_from_slice(&self.ng[..m]);
        }
    }
}

fn deg_for(b: usize) -> usize {
    let raw = NN + 8 - 5 * b;
    if raw < 16 { 16 } else { raw.min(POLY) }
}

fn main() {
    let mut ctx = PolyCtx::new();
    let mut f = vec![0u64; POLY];
    let mut c = vec![0u64; POLY];
    let mut d = vec![0u64; POLY];
    let mut s = vec![0u64; POLY];
    let m4 = MOD - 4;

    for b in (0..MAXB).rev() {
        let n = deg_for(b);
        c[..n].fill(0);
        if n > 1 {
            c[1] = b as u64;
        }
        let lim = n.saturating_sub(5);
        for i in 0..lim {
            c[i + 5] = add_m(c[i + 5], f[i]);
        }
        d[..n].fill(0);
        d[0] = 1;
        let lim2 = n.saturating_sub(2);
        for i in 0..lim2 {
            d[i + 2] = mul_m(m4, c[i]);
        }
        ctx.sqrt_into(&d[..n], n, &mut s);
        f[..n].fill(0);
        for k in 0..lim2 {
            f[k] = mul_m(sub_m(0, s[k + 2]), INV2);
        }
    }

    let mut ans = 0u64;
    for i in 1..=NN {
        ans += f[i];
        if ans >= MOD {
            ans -= MOD;
        }
    }
    println!("{}", ans);
}
