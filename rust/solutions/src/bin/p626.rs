// Project Euler 626 - Counting Binary Matrices
// Burnside over partition pairs of N=20 (p(20)=627).
// M^2 fits u64; M^3 fits u128.

use rayon::prelude::*;

const N: usize = 20;
const M: u64 = 1_001_001_011;
const NP: usize = 627; // p(20)
const MAX_EXP: usize = N * N + 2 * N; // cycles <= N^2, flip exp <= 2N
const _: () = assert!(M <= u64::MAX / M);

const fn gcd_const(mut a: usize, mut b: usize) -> usize {
    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }
    a
}

const GCD: [[u8; N + 1]; N + 1] = {
    let mut t = [[0u8; N + 1]; N + 1];
    let mut a = 1;
    while a <= N {
        let mut b = 1;
        while b <= N {
            t[a][b] = gcd_const(a, b) as u8;
            b += 1;
        }
        a += 1;
    }
    t
};

/// Bit t is set iff (t / gcd(s, t)) is odd: other-size t restricts cycle size s.
const ODD_T: [u32; N + 1] = {
    let mut m = [0u32; N + 1];
    let mut s = 1;
    while s <= N {
        let mut t = 1;
        while t <= N {
            if (t / gcd_const(s, t)) % 2 == 1 {
                m[s] |= 1 << t;
            }
            t += 1;
        }
        s += 1;
    }
    m
};

#[inline(always)]
fn mul_mod(a: u64, b: u64) -> u64 {
    a * b % M
}

fn modinv(a: u64) -> u64 {
    let mut t = 0i64;
    let mut newt = 1i64;
    let mut r = M as i64;
    let mut newr = a as i64;
    while newr != 0 {
        let q = r / newr;
        (t, newt) = (newt, t - q * newt);
        (r, newr) = (newr, r - q * newr);
    }
    if t < 0 {
        t += M as i64;
    }
    t as u64
}

/// At most 5 distinct part sizes for n=20 (1+2+3+4+5=15, 1+…+6=21).
#[derive(Clone, Copy)]
struct Part {
    nd: u8,
    total: u8,
    sizes: [u8; 6],
    counts: [u8; 6],
    restrict_mask: u32,
    arr: u64,
}

fn gen_partitions() -> Vec<Part> {
    let mut result = Vec::with_capacity(NP);
    let mut buf = [0u8; N];
    fn rec(rem: usize, maxv: usize, len: usize, buf: &mut [u8; N], out: &mut Vec<Part>) {
        if rem == 0 {
            let mut p = Part {
                nd: 0,
                total: len as u8,
                sizes: [0; 6],
                counts: [0; 6],
                restrict_mask: 0,
                arr: 0,
            };
            let mut i = 0;
            while i < len {
                let v = buf[i];
                let mut c = 0u8;
                while i < len && buf[i] == v {
                    c += 1;
                    i += 1;
                }
                let k = p.nd as usize;
                p.sizes[k] = v;
                p.counts[k] = c;
                p.nd += 1;
            }
            out.push(p);
            return;
        }
        let top = if maxv < rem { maxv } else { rem };
        for i in (1..=top).rev() {
            buf[len] = i as u8;
            rec(rem - i, i, len + 1, buf, out);
        }
    }
    rec(N, N, 0, &mut buf, &mut result);
    result
}

fn main() {
    let mut fact = [1u64; N + 1];
    for i in 1..=N {
        fact[i] = mul_mod(fact[i - 1], i as u64);
    }
    let mut inv = [0u64; N + 1];
    let mut inv_fact = [1u64; N + 1];
    for i in 1..=N {
        inv[i] = modinv(i as u64);
        inv_fact[i] = mul_mod(inv_fact[i - 1], inv[i]);
    }

    let mut parts = gen_partitions();
    for p in &mut parts {
        let mut sm = 0u32;
        let nd = p.nd as usize;
        let mut arr = fact[N];
        for k in 0..nd {
            let s = p.sizes[k] as usize;
            let c = p.counts[k] as usize;
            sm |= 1u32 << s;
            for _ in 0..c {
                arr = mul_mod(arr, inv[s]);
            }
            arr = mul_mod(arr, inv_fact[c]);
        }
        p.arr = arr;
        let mut rm = 0u32;
        for s in 1..=N {
            if ODD_T[s] & sm != 0 {
                rm |= 1u32 << s;
            }
        }
        p.restrict_mask = rm;
    }

    let mut pow2 = [0u64; MAX_EXP + 1];
    pow2[0] = 1;
    for i in 1..=MAX_EXP {
        pow2[i] = mul_mod(pow2[i - 1], 2);
    }

    let ans: u64 = parts
        .par_iter()
        .map(|p1| {
            let n1 = p1.nd as usize;
            let mut local = 0u64;
            for p2 in &parts {
                let n2 = p2.nd as usize;
                let mut cycles = 0u32;
                let mut nr1 = 0u32;
                let rm2 = p2.restrict_mask;
                // SAFETY: nd <= 5, arrays length 6; sizes in 1..=N index GCD.
                unsafe {
                    for a in 0..n1 {
                        let sa = *p1.sizes.get_unchecked(a);
                        let ca = *p1.counts.get_unchecked(a) as u32;
                        if rm2 & (1u32 << sa) != 0 {
                            nr1 += ca;
                        }
                        let sa = sa as usize;
                        for b in 0..n2 {
                            let sb = *p2.sizes.get_unchecked(b) as usize;
                            let cb = *p2.counts.get_unchecked(b) as u32;
                            cycles += *GCD.get_unchecked(sa).get_unchecked(sb) as u32 * ca * cb;
                        }
                    }
                }
                let mut nr2 = 0u32;
                let rm1 = p1.restrict_mask;
                unsafe {
                    for b in 0..n2 {
                        let sb = *p2.sizes.get_unchecked(b);
                        if rm1 & (1u32 << sb) != 0 {
                            nr2 += *p2.counts.get_unchecked(b) as u32;
                        }
                    }
                }
                let all_restricted = nr1 == p1.total as u32 && nr2 == p2.total as u32;
                let exp2 = (2 * N as u32) - nr1 - nr2 - u32::from(!all_restricted);
                let e = (cycles + exp2) as usize;
                // arr1*arr2*2^e < M^3 < 2^128
                let term = (p1.arr as u128 * p2.arr as u128 * pow2[e] as u128 % M as u128) as u64;
                local += term;
            }
            local % M
        })
        .sum();

    let inv_fact_n = inv_fact[N];
    let inv_2_pow = modinv(pow2[2 * N - 1]);
    let mut ans = ans % M;
    ans = mul_mod(ans, inv_fact_n);
    ans = mul_mod(ans, inv_fact_n);
    ans = mul_mod(ans, inv_2_pow);
    println!("{}", ans);
}
