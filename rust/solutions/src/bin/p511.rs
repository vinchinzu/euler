// Project Euler 511 - Sequences with Divisibility Constraints
// Cyclic convolution mod K using NTT-based fast multiplication with binary exponentiation.
// Uses 3 NTT-friendly primes + CRT for exact convolution, then reduces mod MOD.

const K: usize = 4321;
const MOD: u64 = 1_000_000_000;

// NTT-friendly primes: P = c * 2^k + 1
const P1: u64 = 998244353;  // 119 * 2^23 + 1, primitive root 3
const P2: u64 = 985661441;  // 235 * 2^22 + 1, primitive root 3
const P3: u64 = 754974721;  //  45 * 2^24 + 1, primitive root 11

// NTT size: next power of 2 >= 2*K - 1 = 8641
const NTT_LEN: usize = 16384; // 2^14
const LOG_NTT: u32 = 14;

#[inline(always)]
fn mul_mod(a: u64, b: u64, m: u64) -> u64 {
    (a as u128 * b as u128 % m as u128) as u64
}

fn pow_mod(mut base: u64, mut exp: u64, m: u64) -> u64 {
    let mut result = 1u64;
    base %= m;
    while exp > 0 {
        if exp & 1 == 1 {
            result = mul_mod(result, base, m);
        }
        base = mul_mod(base, base, m);
        exp >>= 1;
    }
    result
}

/// Precomputed NTT context for a given prime
struct NttCtx {
    p: u64,
    // Forward twiddle factors for each level: roots[level][j] for j in 0..half
    // Level l has half = 2^l, len = 2^(l+1)
    fwd_roots: Vec<Vec<u64>>,
    inv_roots: Vec<Vec<u64>>,
    inv_n: u64,
}

impl NttCtx {
    fn new(p: u64, g: u64) -> Self {
        let n = NTT_LEN;
        let mut fwd_roots = Vec::with_capacity(LOG_NTT as usize);
        let mut inv_roots = Vec::with_capacity(LOG_NTT as usize);

        for level in 0..LOG_NTT {
            let half = 1usize << level;
            let len = half << 1;
            let w_fwd = pow_mod(g, (p - 1) / len as u64, p);
            let w_inv = pow_mod(w_fwd, p - 2, p);

            let mut fwd = Vec::with_capacity(half);
            let mut inv = Vec::with_capacity(half);
            let mut wf = 1u64;
            let mut wi = 1u64;
            for _ in 0..half {
                fwd.push(wf);
                inv.push(wi);
                wf = mul_mod(wf, w_fwd, p);
                wi = mul_mod(wi, w_inv, p);
            }
            fwd_roots.push(fwd);
            inv_roots.push(inv);
        }

        let inv_n = pow_mod(n as u64, p - 2, p);
        NttCtx { p, fwd_roots, inv_roots, inv_n }
    }

    fn ntt(&self, a: &mut [u64; NTT_LEN], invert: bool) {
        let n = NTT_LEN;
        let p = self.p;

        // Bit-reversal permutation
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

        let roots = if invert { &self.inv_roots } else { &self.fwd_roots };

        for level in 0..LOG_NTT as usize {
            let half = 1usize << level;
            let len = half << 1;
            let ref w_table = roots[level];

            let mut i = 0;
            while i < n {
                for jj in 0..half {
                    // SAFETY: i + jj < n and i + jj + half < n guaranteed by loop structure
                    unsafe {
                        let wn = *w_table.get_unchecked(jj);
                        let u = *a.get_unchecked(i + jj);
                        let v = mul_mod(*a.get_unchecked(i + jj + half), wn, p);
                        *a.get_unchecked_mut(i + jj) = if u + v >= p { u + v - p } else { u + v };
                        *a.get_unchecked_mut(i + jj + half) = if u >= v { u - v } else { u + p - v };
                    }
                }
                i += len;
            }
        }

        if invert {
            let inv_n = self.inv_n;
            for x in a.iter_mut() {
                *x = mul_mod(*x, inv_n, p);
            }
        }
    }
}

/// Precomputed CRT constants
struct CrtCtx {
    inv_p1_mod_p2: u64,
    inv_m12_mod_p3: u64,
    m12: u128,
}

impl CrtCtx {
    fn new() -> Self {
        let inv_p1_mod_p2 = pow_mod(P1 % P2, P2 - 2, P2);
        let m12 = P1 as u128 * P2 as u128;
        let m12_mod_p3 = (m12 % P3 as u128) as u64;
        let inv_m12_mod_p3 = pow_mod(m12_mod_p3, P3 - 2, P3);
        CrtCtx { inv_p1_mod_p2, inv_m12_mod_p3, m12 }
    }

    #[inline(always)]
    fn crt3(&self, r1: u64, r2: u64, r3: u64) -> u128 {
        let diff = if r2 >= r1 % P2 { r2 - r1 % P2 } else { r2 + P2 - r1 % P2 };
        let k = mul_mod(diff, self.inv_p1_mod_p2, P2);
        let x12 = r1 as u128 + k as u128 * P1 as u128;

        let x12_mod_p3 = (x12 % P3 as u128) as u64;
        let diff2 = if r3 >= x12_mod_p3 { r3 - x12_mod_p3 } else { r3 + P3 - x12_mod_p3 };
        let k2 = mul_mod(diff2, self.inv_m12_mod_p3, P3);
        x12 + k2 as u128 * self.m12
    }
}

struct ConvCtx {
    ntt1: NttCtx,
    ntt2: NttCtx,
    ntt3: NttCtx,
    crt: CrtCtx,
}

impl ConvCtx {
    fn new() -> Self {
        ConvCtx {
            ntt1: NttCtx::new(P1, 3),
            ntt2: NttCtx::new(P2, 3),
            ntt3: NttCtx::new(P3, 11),
            crt: CrtCtx::new(),
        }
    }

    /// Cyclic convolution of two length-K arrays modulo MOD.
    fn poly_multiply_cyclic(&self, a: &[u64; K], b: &[u64; K]) -> Box<[u64; K]> {
        // Linear convolution mod each prime
        let c1 = self.linear_conv(&self.ntt1, a, b);
        let c2 = self.linear_conv(&self.ntt2, a, b);
        let c3 = self.linear_conv(&self.ntt3, a, b);

        // Fold linear convolution into cyclic and apply CRT + mod MOD
        let mut result = Box::new([0u64; K]);
        for m in 0..K {
            let mut val = self.crt.crt3(c1[m], c2[m], c3[m]);
            if m + K < 2 * K - 1 {
                val += self.crt.crt3(c1[m + K], c2[m + K], c3[m + K]);
            }
            result[m] = (val % MOD as u128) as u64;
        }
        result
    }

    fn linear_conv(&self, ctx: &NttCtx, a: &[u64; K], b: &[u64; K]) -> Box<[u64; NTT_LEN]> {
        let mut fa = Box::new([0u64; NTT_LEN]);
        let mut fb = Box::new([0u64; NTT_LEN]);
        fa[..K].copy_from_slice(a);
        fb[..K].copy_from_slice(b);
        ctx.ntt(&mut fa, false);
        ctx.ntt(&mut fb, false);
        let p = ctx.p;
        for i in 0..NTT_LEN {
            fa[i] = mul_mod(fa[i], fb[i], p);
        }
        ctx.ntt(&mut fa, true);
        fa
    }
}

fn find_divisors(n: i64) -> Vec<i64> {
    let mut divs = Vec::new();
    let mut i = 1i64;
    while i * i <= n {
        if n % i == 0 {
            divs.push(i);
            if i * i != n { divs.push(n / i); }
        }
        i += 1;
    }
    divs
}

fn imod(a: i64, m: i64) -> usize {
    ((a % m + m) % m) as usize
}

fn num_transitions(n: i64, divisors: &[i64], ctx: &ConvCtx) -> Box<[u64; K]> {
    if n == 1 {
        let mut result = Box::new([0u64; K]);
        for &d in divisors {
            let idx = imod(d, K as i64);
            result[idx] = (result[idx] + 1) % MOD;
        }
        return result;
    }

    let half = num_transitions(n / 2, divisors, ctx);
    let mut result = ctx.poly_multiply_cyclic(&half, &half);

    if n % 2 == 1 {
        let one = num_transitions(1, divisors, ctx);
        result = ctx.poly_multiply_cyclic(&result, &one);
    }

    result
}

fn main() {
    let n: i64 = 1234567898765;
    let divisors = find_divisors(n);
    let ctx = ConvCtx::new();

    let transitions = num_transitions(n, &divisors, &ctx);

    let idx = imod(-n, K as i64);
    println!("{}", transitions[idx]);
}
