// Project Euler 445 - Retractions A
// R(n) = σ*(n) - n,  σ*(n) = ∏_{p^e || n} (1 + p^e)
// ∑_{k=1}^{N-1} R(C(N,k)) = ∑ σ*(C(N,k)) - (2^N - 2)  (mod 10^9+7)
//
// Walk C(N,k) via C(N,k) = C(N,k-1)*(N-k+1)/k. Odd-only linear SPF packed
// as (rest, exp, prime-index); (1+p^e) and inverses precomputed in batch.

const NN: usize = 10_000_000;
const MOD: u64 = 1_000_000_007;
const MOD_I64: i64 = 1_000_000_007;

#[inline(always)]
fn mul(a: u64, b: u64) -> u64 {
    a * b % MOD
}

/// Modular inverse via extended Euclidean algorithm (MOD prime).
#[inline(always)]
fn mod_inv(a: u64) -> u64 {
    let mut t: i64 = 0;
    let mut newt: i64 = 1;
    let mut r: i64 = MOD_I64;
    let mut newr: i64 = a as i64;
    while newr != 0 {
        let q = r / newr;
        let tmp_t = t - q * newt;
        t = newt;
        newt = tmp_t;
        let tmp_r = r - q * newr;
        r = newr;
        newr = tmp_r;
    }
    if t < 0 {
        t += MOD_I64;
    }
    t as u64
}

/// Batch invert `a[i]` in place.
fn batch_inv(a: &mut [u64]) {
    let n = a.len();
    if n == 0 {
        return;
    }
    let mut pref = vec![0u64; n];
    let mut acc = 1u64;
    for i in 0..n {
        acc = mul(acc, a[i]);
        pref[i] = acc;
    }
    let mut inv = mod_inv(acc);
    for i in (1..n).rev() {
        let v = a[i];
        a[i] = mul(inv, pref[i - 1]);
        inv = mul(inv, v);
    }
    a[0] = inv;
}

fn pow_mod(mut base: u64, mut exp: u64) -> u64 {
    let mut r = 1u64;
    while exp > 0 {
        if exp & 1 == 1 {
            r = mul(r, base);
        }
        base = mul(base, base);
        exp >>= 1;
    }
    r
}

fn main() {
    // Odd-only linear sieve. fac_odd[n>>1] packs rest (24) | exp (8) | pi (32).
    let n_odd = (NN + 1) / 2;
    let mut fac_odd = vec![0u64; n_odd];
    let mut primes: Vec<u32> = Vec::with_capacity(665_000);
    primes.push(2);

    for i in 1..n_odd {
        let n = 2 * i + 1;
        if n > NN {
            break;
        }
        if unsafe { *fac_odd.get_unchecked(i) } == 0 {
            let pi = primes.len() as u64;
            primes.push(n as u32);
            unsafe {
                *fac_odd.get_unchecked_mut(i) = 1 | (1u64 << 24) | (pi << 32);
            }
        }
        let spf_pi = unsafe { (*fac_odd.get_unchecked(i) >> 32) as usize };
        for pi in 1..primes.len() {
            let p = unsafe { *primes.get_unchecked(pi) } as usize;
            let v = n * p;
            if v > NN {
                break;
            }
            let packed = if pi == spf_pi {
                let prev = unsafe { *fac_odd.get_unchecked(i) };
                let e = ((prev >> 24) & 0xFF) + 1;
                let rest = prev & 0xFFFFFF;
                rest | (e << 24) | ((pi as u64) << 32)
            } else {
                (n as u64) | (1u64 << 24) | ((pi as u64) << 32)
            };
            unsafe {
                *fac_odd.get_unchecked_mut(v >> 1) = packed;
            }
            if pi == spf_pi {
                break;
            }
        }
    }

    let nprimes = primes.len();

    // max_e[p] = # of base-p digits of N ≥ max v_p(C(N,k)) (Kummer carries).
    let mut offset = vec![0u32; nprimes];
    let mut total: u32 = 0;
    for i in 0..nprimes {
        let p = unsafe { *primes.get_unchecked(i) } as u64;
        let mut e = 0u32;
        let mut x = NN as u64;
        while x > 0 {
            x /= p;
            e += 1;
        }
        unsafe {
            *offset.get_unchecked_mut(i) = total;
        }
        total += e;
    }

    let mut val_1pe = vec![0u64; total as usize];
    for i in 0..nprimes {
        let p = unsafe { *primes.get_unchecked(i) } as u64;
        let off = unsafe { *offset.get_unchecked(i) } as usize;
        let mut pe = 1u64;
        let mut x = NN as u64;
        let mut slot = off;
        while x > 0 {
            x /= p;
            pe = mul(pe, p);
            let t = pe + 1;
            unsafe {
                *val_1pe.get_unchecked_mut(slot) = if t >= MOD { t - MOD } else { t };
            }
            slot += 1;
        }
    }
    drop(primes);

    let mut inv_1pe = val_1pe.clone();
    batch_inv(&mut inv_1pe);

    let mut exp = vec![0u8; nprimes];
    let mut prod = 1u64;
    let mut sum_sigma = 0u64;
    let mid = NN / 2;

    for k in 1..=mid {
        // Multiply by (N+1-k)
        let mut x = NN + 1 - k;
        if x & 1 == 0 {
            let e = x.trailing_zeros();
            x >>= e;
            unsafe {
                apply::<true>(0, e, &mut exp, &offset, &val_1pe, &inv_1pe, &mut prod);
            }
        }
        while x > 1 {
            let w = unsafe { *fac_odd.get_unchecked(x >> 1) };
            x = (w & 0xFFFFFF) as usize;
            let e = ((w >> 24) & 0xFF) as u32;
            let pi = (w >> 32) as usize;
            unsafe {
                apply::<true>(pi, e, &mut exp, &offset, &val_1pe, &inv_1pe, &mut prod);
            }
        }

        // Divide by k
        x = k;
        if x & 1 == 0 {
            let e = x.trailing_zeros();
            x >>= e;
            unsafe {
                apply::<false>(0, e, &mut exp, &offset, &val_1pe, &inv_1pe, &mut prod);
            }
        }
        while x > 1 {
            let w = unsafe { *fac_odd.get_unchecked(x >> 1) };
            x = (w & 0xFFFFFF) as usize;
            let e = ((w >> 24) & 0xFF) as u32;
            let pi = (w >> 32) as usize;
            unsafe {
                apply::<false>(pi, e, &mut exp, &offset, &val_1pe, &inv_1pe, &mut prod);
            }
        }

        if k == mid {
            sum_sigma += prod;
        } else {
            sum_sigma += prod << 1;
        }
        if sum_sigma >= (MOD << 1) {
            sum_sigma -= MOD << 1;
        }
        if sum_sigma >= MOD {
            sum_sigma -= MOD;
        }
    }

    let sum_binom = (pow_mod(2, NN as u64) + MOD - 2) % MOD;
    let ans = (sum_sigma + MOD - sum_binom) % MOD;
    println!("{}", ans);
}

#[inline(always)]
unsafe fn apply<const ADD: bool>(
    pi: usize,
    e: u32,
    exp: &mut [u8],
    offset: &[u32],
    val_1pe: &[u64],
    inv_1pe: &[u64],
    prod: &mut u64,
) {
    let old_e = unsafe { *exp.get_unchecked(pi) } as u32;
    let new_e = if ADD { old_e + e } else { old_e - e };
    let off = unsafe { *offset.get_unchecked(pi) } as usize;
    let mut p = *prod;
    if old_e != 0 {
        p = mul(p, unsafe { *inv_1pe.get_unchecked(off + old_e as usize - 1) });
    }
    if new_e != 0 {
        p = mul(p, unsafe { *val_1pe.get_unchecked(off + new_e as usize - 1) });
    }
    *prod = p;
    unsafe {
        *exp.get_unchecked_mut(pi) = new_e as u8;
    }
}
