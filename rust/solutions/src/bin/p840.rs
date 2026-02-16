// Project Euler 840 - Arithmetic Derivative Partition Sums
// D(n) arithmetic derivative, B[k], g[n] convolution, S(N) = sum g[1..N]
// Uses divide-and-conquer NTT for the online convolution: O(N log^2 N)
// 3-prime NTT with CRT since MOD=999676999 is not NTT-friendly.

const NMAX: usize = 50000;
const MOD: i64 = 999676999;
const UMOD: u64 = MOD as u64;

const P1: u64 = 998244353;
const P2: u64 = 985661441;
const P3: u64 = 754974721;

const MAX_LOG: usize = 17;
const MAX_NTT: usize = 1 << MAX_LOG; // 131072

const DIRECT_THRESHOLD: usize = 512;

#[inline(always)]
fn mul_mod_u(a: u64, b: u64, m: u64) -> u64 {
    (a as u128 * b as u128 % m as u128) as u64
}

fn pow_mod_u(mut base: u64, mut exp: u64, m: u64) -> u64 {
    let mut result = 1u64;
    base %= m;
    while exp > 0 {
        if exp & 1 == 1 { result = mul_mod_u(result, base, m); }
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
        while pm1 % 2 == 0 { pm1 /= 2; v2 += 1; }

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

        NttPrime { p, fwd_root, inv_root, inv_pow2 }
    }

    fn ntt(&self, a: &mut [u64], invert: bool) {
        let n = a.len();
        if n == 1 { return; }
        let log_n = n.trailing_zeros() as usize;
        let p = self.p;

        let mut j = 0usize;
        for i in 1..n {
            let mut bit = n >> 1;
            while j & bit != 0 { j ^= bit; bit >>= 1; }
            j ^= bit;
            if i < j { a.swap(i, j); }
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
                        *a.get_unchecked_mut(i + jj + half) = if u >= v { u - v } else { u + p - v };
                    }
                    wn = mul_mod_u(wn, w_base, p);
                }
                i += len;
            }
            half = len;
        }

        if invert {
            let inv_n = self.inv_pow2[log_n];
            for x in a.iter_mut() { *x = mul_mod_u(*x, inv_n, p); }
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
        CrtCtx { inv_p1_mod_p2, inv_m12_mod_p3, m12 }
    }

    #[inline]
    fn crt3_mod(&self, r1: u64, r2: u64, r3: u64) -> u64 {
        let diff = if r2 >= r1 % P2 { r2 - r1 % P2 } else { r2 + P2 - r1 % P2 };
        let k = mul_mod_u(diff, self.inv_p1_mod_p2, P2);
        let x12 = r1 as u128 + k as u128 * P1 as u128;
        let x12_mod_p3 = (x12 % P3 as u128) as u64;
        let diff2 = if r3 >= x12_mod_p3 { r3 - x12_mod_p3 } else { r3 + P3 - x12_mod_p3 };
        let k2 = mul_mod_u(diff2, self.inv_m12_mod_p3, P3);
        ((x12 + k2 as u128 * self.m12) % UMOD as u128) as u64
    }
}

/// Shared workspace to avoid repeated allocation
struct Workspace {
    // 2 buffers per prime (fa and fb), 3 primes = 6 buffers
    buf: Vec<u64>, // flat: 6 * MAX_NTT
}

impl Workspace {
    fn new() -> Self {
        Workspace { buf: vec![0u64; 6 * MAX_NTT] }
    }

    fn get_pair(&mut self, prime_idx: usize, ntt_len: usize) -> (&mut [u64], &mut [u64]) {
        let base = prime_idx * 2 * MAX_NTT;
        let (left, right) = self.buf[base..base + 2 * MAX_NTT].split_at_mut(MAX_NTT);
        (&mut left[..ntt_len], &mut right[..ntt_len])
    }

}

fn contribute_ntt(
    lo: usize, mid: usize, hi: usize,
    b_arr: &[u64], g: &mut [i64],
    ntts: &[NttPrime; 3], crt: &CrtCtx, ws: &mut Workspace,
) {
    let len_a = mid - lo;
    let len_b = hi - lo;
    let out_len = len_a + len_b - 1;
    let ntt_len = out_len.next_power_of_two();

    for pi in 0..3 {
        let (fa, fb) = ws.get_pair(pi, ntt_len);
        // Fill fa with g[lo..mid], zero-padded
        for i in 0..len_a { fa[i] = g[lo + i] as u64; }
        fa[len_a..].fill(0);
        // Fill fb with b[0..len_b], zero-padded
        fb[..len_b].copy_from_slice(&b_arr[..len_b]);
        fb[len_b..].fill(0);
        // Forward NTT both
        ntts[pi].ntt(fa, false);
        ntts[pi].ntt(fb, false);
        // Pointwise multiply into fa
        let p = ntts[pi].p;
        for i in 0..ntt_len {
            unsafe {
                *fa.get_unchecked_mut(i) = mul_mod_u(*fa.get_unchecked(i), *fb.get_unchecked(i), p);
            }
        }
        // Inverse NTT
        ntts[pi].ntt(fa, true);
    }

    // CRT and accumulate
    for n in mid..hi {
        let m = n - lo;
        unsafe {
            let r1 = *ws.buf.get_unchecked(0 * 2 * MAX_NTT + m);
            let r2 = *ws.buf.get_unchecked(1 * 2 * MAX_NTT + m);
            let r3 = *ws.buf.get_unchecked(2 * 2 * MAX_NTT + m);
            let val = crt.crt3_mod(r1, r2, r3);
            g[n] = (g[n] + val as i64) % MOD;
        }
    }
}

fn solve(
    lo: usize, hi: usize,
    b_arr: &[u64], g: &mut [i64], inv_arr: &[i64],
    ntts: &[NttPrime; 3], crt: &CrtCtx, ws: &mut Workspace,
) {
    if hi - lo <= 1 {
        if lo > 0 {
            g[lo] = ((g[lo] as i128 * inv_arr[lo] as i128) % MOD as i128) as i64;
            if g[lo] < 0 { g[lo] += MOD; }
        }
        return;
    }

    if hi - lo <= DIRECT_THRESHOLD {
        for n in lo..hi {
            if n > 0 {
                g[n] = ((g[n] as i128 * inv_arr[n] as i128) % MOD as i128) as i64;
                if g[n] < 0 { g[n] += MOD; }
            }
            let gn = g[n] as i128;
            for m in (n + 1)..hi {
                unsafe {
                    g[m] = ((g[m] as i128 + *b_arr.get_unchecked(m - n) as i128 * gn) % MOD as i128) as i64;
                }
            }
        }
        return;
    }

    let mid = (lo + hi) / 2;
    solve(lo, mid, b_arr, g, inv_arr, ntts, crt, ws);
    contribute_ntt(lo, mid, hi, b_arr, g, ntts, crt, ws);
    solve(mid, hi, b_arr, g, inv_arr, ntts, crt, ws);
}

fn main() {
    // SPF sieve
    let mut spf = vec![0u32; NMAX + 1];
    for i in 2..=NMAX {
        if spf[i] == 0 { spf[i] = i as u32; }
        if (i as u64) * (i as u64) <= NMAX as u64 {
            if spf[i] == i as u32 {
                let mut j = i * i;
                while j <= NMAX {
                    if spf[j] == 0 { spf[j] = i as u32; }
                    j += i;
                }
            }
        }
    }

    // Arithmetic derivative
    let mut d_arr = vec![0i64; NMAX + 1];
    d_arr[1] = 1;
    for n in 2..=NMAX {
        let mut m = n;
        let mut deriv: i64 = 0;
        while m > 1 {
            let p = spf[m] as usize;
            let mut e = 0;
            while m % p == 0 { m /= p; e += 1; }
            deriv = (deriv + e as i64 * (n / p) as i64) % MOD;
        }
        d_arr[n] = deriv;
    }

    // B[k] = sum_{d|k} d * D(d)^(k/d)
    let mut b_arr = vec![0u64; NMAX + 1];
    for d in 1..=NMAX {
        let y = d_arr[d] as u64 % UMOD;
        let mut pow_y = y;
        let mut k = d;
        while k <= NMAX {
            b_arr[k] = ((b_arr[k] as u128 + d as u128 * pow_y as u128) % UMOD as u128) as u64;
            pow_y = (pow_y as u128 * y as u128 % UMOD as u128) as u64;
            k += d;
        }
    }

    // Modular inverses
    let mut inv_arr = vec![0i64; NMAX + 1];
    inv_arr[1] = 1;
    for n in 2..=NMAX {
        inv_arr[n] = (MOD - (MOD / n as i64) * inv_arr[(MOD % n as i64) as usize] % MOD) % MOD;
    }

    let ntts = [
        NttPrime::new(P1, 3),
        NttPrime::new(P2, 3),
        NttPrime::new(P3, 11),
    ];
    let crt = CrtCtx::new();
    let mut ws = Workspace::new();

    let mut g = vec![0i64; NMAX + 1];
    g[0] = 1;

    solve(0, NMAX + 1, &b_arr, &mut g, &inv_arr, &ntts, &crt, &mut ws);

    let mut total: i64 = 0;
    for n in 1..=NMAX {
        total = (total + g[n]) % MOD;
    }

    println!("{}", total);
}
