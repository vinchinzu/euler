// Project Euler 781 - Feynman Diagrams
// EGF: f * s = t (s[0] = 1), so f = t * s^{-1}. Inverse via Newton + 3-prime NTT.

const MOD: u64 = 1_000_000_007;
const P1: u64 = 998_244_353;
const P2: u64 = 985_661_441;
const P3: u64 = 754_974_721;
const MAX_LOG: usize = 16;
const MAX_NTT: usize = 1 << MAX_LOG; // 65536; product of two deg-25000 polys
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
                    // SAFETY: i + jj + half < i + len <= n; tw[half + jj] is precomputed
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
        let ai = a[i] as u128;
        let jmax = (nout - i).min(nb);
        for j in 0..jmax {
            out[i + j] = ((out[i + j] as u128 + ai * b[j] as u128) % MOD as u128) as u64;
        }
    }
}

fn poly_mul(
    a: &[u64],
    b: &[u64],
    ntt1: &NttPrime<P1>,
    ntt2: &NttPrime<P2>,
    ntt3: &NttPrime<P3>,
    crt: &CrtCtx,
    ws: &mut Workspace,
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
        let mut tmp = vec![0u64; take];
        poly_mul_naive(a, b, &mut tmp);
        out[..take].copy_from_slice(&tmp);
        if take < out.len() {
            out[take..].fill(0);
        }
        return;
    }

    let ntt_len = need.next_power_of_two();
    debug_assert!(ntt_len <= MAX_NTT);

    {
        let (fa, fb) = ws.get_pair(0, ntt_len);
        ntt_one::<P1>(ntt1, a, b, fa, fb);
    }
    {
        let (fa, fb) = ws.get_pair(1, ntt_len);
        ntt_one::<P2>(ntt2, a, b, fa, fb);
    }
    {
        let (fa, fb) = ws.get_pair(2, ntt_len);
        ntt_one::<P3>(ntt3, a, b, fa, fb);
    }

    for i in 0..take {
        // SAFETY: inverse NTT results sit at the start of each prime's first buffer
        unsafe {
            let r1 = *ws.buf.get_unchecked(i);
            let r2 = *ws.buf.get_unchecked(2 * MAX_NTT + i);
            let r3 = *ws.buf.get_unchecked(4 * MAX_NTT + i);
            *out.get_unchecked_mut(i) = crt.crt3_mod(r1, r2, r3);
        }
    }
    if take < out.len() {
        out[take..].fill(0);
    }
}

fn poly_inv(
    f: &[u64],
    n: usize,
    ntt1: &NttPrime<P1>,
    ntt2: &NttPrime<P2>,
    ntt3: &NttPrime<P3>,
    crt: &CrtCtx,
    ws: &mut Workspace,
) -> Vec<u64> {
    let mut g = vec![0u64; n];
    g[0] = pow_mod_m(f[0], MOD - 2, MOD);
    let mut t = vec![0u64; n];
    let mut r = vec![0u64; n];
    let mut ng = vec![0u64; n];
    let mut m = 1usize;
    while m < n {
        let want = (2 * m).min(n);
        poly_mul(
            &f[..want],
            &g[..m],
            ntt1,
            ntt2,
            ntt3,
            crt,
            ws,
            &mut t[..want],
        );
        r[0] = (2 + MOD - t[0]) % MOD;
        for i in 1..want {
            r[i] = if t[i] == 0 { 0 } else { MOD - t[i] };
        }
        poly_mul(
            &g[..m],
            &r[..want],
            ntt1,
            ntt2,
            ntt3,
            crt,
            ws,
            &mut ng[..want],
        );
        g[..want].copy_from_slice(&ng[..want]);
        m = want;
    }
    g
}

fn main() {
    const N: usize = 50_000;
    let m = N / 2;

    let ntt1 = NttPrime::<P1>::new(3);
    let ntt2 = NttPrime::<P2>::new(3);
    let ntt3 = NttPrime::<P3>::new(11);
    let crt = CrtCtx::new();
    let mut ws = Workspace::new();

    let mut d = vec![0u64; N + 1];
    d[0] = 1;
    d[1] = 0;
    for k in 2..=N {
        let sign = if k % 2 == 0 { 1u64 } else { MOD - 1 };
        d[k] = (k as u64 * d[k - 1] % MOD + sign) % MOD;
    }

    let mut fact = vec![1u64; m + 1];
    for i in 1..=m {
        fact[i] = fact[i - 1] * i as u64 % MOD;
    }
    let mut inv_fact = vec![1u64; m + 1];
    inv_fact[m] = pow_mod_m(fact[m], MOD - 2, MOD);
    for i in (0..m).rev() {
        inv_fact[i] = inv_fact[i + 1] * (i as u64 + 1) % MOD;
    }

    let inv2 = pow_mod_m(2, MOD - 2, MOD);
    let mut inv_pow2 = vec![1u64; m + 1];
    for i in 1..=m {
        inv_pow2[i] = inv_pow2[i - 1] * inv2 % MOD;
    }

    let mut t = vec![0u64; m + 1];
    let mut s = vec![0u64; m + 1];
    for j in 0..=m {
        let idx = 2 * j;
        let b_val = if idx == 0 {
            1
        } else {
            ((idx as u64 + 1) * d[idx] + idx as u64 * d[idx - 1]) % MOD
        };
        let coeff = inv_pow2[j] * inv_fact[j] % MOD;
        t[j] = b_val * coeff % MOD;
        s[j] = d[idx] * coeff % MOD;
    }

    // f * s = t  (s[0] = 1), so f = t * s^{-1}  (mod x^{m+1})
    let inv_s = poly_inv(&s, m + 1, &ntt1, &ntt2, &ntt3, &crt, &mut ws);
    let mut f = vec![0u64; m + 1];
    poly_mul(&t, &inv_s, &ntt1, &ntt2, &ntt3, &crt, &mut ws, &mut f);

    println!("{}", f[m]);
}
