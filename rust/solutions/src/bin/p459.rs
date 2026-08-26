use rayon::prelude::*;

const N: usize = 1_000_000;
const L: usize = 512;
const LBITS: usize = L / 64;

fn nim_prod(a: i32, b: i32, cache: &mut [i32]) -> i32 {
    if a == 0 || b == 0 {
        return 0;
    }
    if a == 1 {
        return b;
    }
    if b == 1 {
        return a;
    }
    let au = a as usize;
    let bu = b as usize;
    if au < L && bu < L {
        let idx = au * L + bu;
        // SAFETY: au < L, bu < L so idx < L*L = cache.len()
        let cached = unsafe { *cache.get_unchecked(idx) };
        if cached >= 0 {
            return cached;
        }
    }

    let result;
    if a & (a - 1) != 0 {
        let low = a & (-a);
        result = nim_prod(low, b, cache) ^ nim_prod(a ^ low, b, cache);
    } else if b & (b - 1) != 0 {
        let low = b & (-b);
        result = nim_prod(a, low, cache) ^ nim_prod(a, b ^ low, cache);
    } else {
        let i = a.trailing_zeros();
        let j = b.trailing_zeros();
        if i & j == 0 {
            result = a * b;
        } else {
            let common = i & j;
            let common_bit = 31 - common.leading_zeros();
            let fk = 1u32 << common_bit;
            let f_k = 1i32 << fk;
            let sq_fk = f_k + (f_k >> 1);
            let a2 = 1i32 << (i - fk);
            let b2 = 1i32 << (j - fk);
            result = nim_prod(nim_prod(a2, b2, cache), sq_fk, cache);
        }
    }

    if au < L && bu < L {
        let idx = au * L + bu;
        // SAFETY: same bounds as above; nim-product is commutative
        unsafe {
            *cache.get_unchecked_mut(idx) = result;
            *cache.get_unchecked_mut(bu * L + au) = result;
        }
    }
    result
}

fn build_nim_table() -> Vec<i32> {
    let mut cache = vec![-1i32; L * L];
    for a in 0..L {
        for b in 0..=a {
            let _ = nim_prod(a as i32, b as i32, &mut cache);
        }
    }
    cache
}

#[inline(always)]
fn mark_bit(used: &mut [u64; LBITS], val: u16) {
    let v = val as usize;
    if v < L {
        // SAFETY: v < 512 so v>>6 < 8 = used.len()
        unsafe {
            *used.get_unchecked_mut(v >> 6) |= 1u64 << (v & 63);
        }
    }
}

#[inline(always)]
fn mex_of(used: &[u64; LBITS]) -> u16 {
    for i in 0..LBITS {
        let w = used[i];
        if w != u64::MAX {
            return (i as u16) * 64 + w.trailing_ones() as u16;
        }
    }
    L as u16
}

/// Prefix-XOR of mex values for the 1D subtraction / flipping game.
fn fill_rn(steps: &[usize]) -> Vec<u16> {
    let mut rn = vec![0u16; N + 1];
    let nst = steps.len();
    let mut kmax = 0usize;
    for j in 1..=N {
        while kmax < nst && unsafe { *steps.get_unchecked(kmax) } <= j {
            kmax += 1;
        }
        let prev = unsafe { *rn.get_unchecked(j - 1) };
        let mut used = [0u64; LBITS];
        // Squares and triangular numbers both include 1, which yields val = 0.
        used[0] = 1;
        let mut k = 1usize;
        while k + 8 <= kmax {
            unsafe {
                let rp = rn.as_ptr();
                let sp = steps.as_ptr();
                #[cfg(target_arch = "x86_64")]
                {
                    use std::arch::x86_64::{_mm_prefetch, _MM_HINT_T0};
                    if k + 16 <= kmax {
                        // SAFETY: k+15 < kmax, each step s <= j so j-s is in-range
                        _mm_prefetch(rp.add(j - *sp.add(k + 8)) as *const i8, _MM_HINT_T0);
                        _mm_prefetch(rp.add(j - *sp.add(k + 9)) as *const i8, _MM_HINT_T0);
                        _mm_prefetch(rp.add(j - *sp.add(k + 10)) as *const i8, _MM_HINT_T0);
                        _mm_prefetch(rp.add(j - *sp.add(k + 11)) as *const i8, _MM_HINT_T0);
                        _mm_prefetch(rp.add(j - *sp.add(k + 12)) as *const i8, _MM_HINT_T0);
                        _mm_prefetch(rp.add(j - *sp.add(k + 13)) as *const i8, _MM_HINT_T0);
                        _mm_prefetch(rp.add(j - *sp.add(k + 14)) as *const i8, _MM_HINT_T0);
                        _mm_prefetch(rp.add(j - *sp.add(k + 15)) as *const i8, _MM_HINT_T0);
                    }
                }
                // SAFETY: k+7 < kmax, steps[k+i] <= j, rn.len() == N+1
                let v0 = prev ^ *rp.add(j - *sp.add(k));
                let v1 = prev ^ *rp.add(j - *sp.add(k + 1));
                let v2 = prev ^ *rp.add(j - *sp.add(k + 2));
                let v3 = prev ^ *rp.add(j - *sp.add(k + 3));
                let v4 = prev ^ *rp.add(j - *sp.add(k + 4));
                let v5 = prev ^ *rp.add(j - *sp.add(k + 5));
                let v6 = prev ^ *rp.add(j - *sp.add(k + 6));
                let v7 = prev ^ *rp.add(j - *sp.add(k + 7));
                mark_bit(&mut used, v0);
                mark_bit(&mut used, v1);
                mark_bit(&mut used, v2);
                mark_bit(&mut used, v3);
                mark_bit(&mut used, v4);
                mark_bit(&mut used, v5);
                mark_bit(&mut used, v6);
                mark_bit(&mut used, v7);
            }
            k += 8;
        }
        while k < kmax {
            let s = unsafe { *steps.get_unchecked(k) };
            let val = prev ^ unsafe { *rn.get_unchecked(j - s) };
            mark_bit(&mut used, val);
            k += 1;
        }
        let mex = mex_of(&used);
        unsafe {
            *rn.get_unchecked_mut(j) = prev ^ mex;
        }
    }
    rn
}

fn count_range(rn: &[u16], steps: &[usize], lo: usize, hi: usize) -> [i64; L] {
    let mut cnt = [0i64; L];
    if lo > hi {
        return cnt;
    }
    for &s in steps {
        if s > hi {
            break;
        }
        let start = lo.max(s);
        for j in start..=hi {
            // SAFETY: 1 <= start <= j <= hi <= N, j >= s, rn.len() == N+1
            let val = unsafe { (*rn.get_unchecked(j) ^ *rn.get_unchecked(j - s)) as usize };
            if val < L {
                unsafe {
                    *cnt.get_unchecked_mut(val) += 1;
                }
            }
        }
    }
    cnt
}

fn count_xors(rn: &[u16], steps: &[usize]) -> [i64; L] {
    let nthreads = rayon::current_num_threads().max(1);
    let nchunks = (nthreads * 8).max(8);
    (0..nchunks)
        .into_par_iter()
        .map(|c| {
            let lo = c * N / nchunks + 1;
            let hi = (c + 1) * N / nchunks;
            count_range(rn, steps, lo, hi)
        })
        .reduce(
            || [0i64; L],
            |mut a, b| {
                for i in 0..L {
                    a[i] += b[i];
                }
                a
            },
        )
}

fn main() {
    let mut sq_steps = Vec::new();
    let mut tr_steps = Vec::new();
    let mut i = 1i64;
    while i * i <= N as i64 {
        sq_steps.push((i * i) as usize);
        i += 1;
    }
    i = 1;
    while i * (i + 1) / 2 <= N as i64 {
        tr_steps.push((i * (i + 1) / 2) as usize);
        i += 1;
    }

    let ((rn_x, rn_y), table) = rayon::join(
        || rayon::join(|| fill_rn(&sq_steps), || fill_rn(&tr_steps)),
        build_nim_table,
    );

    let (cnt_x, cnt_y) = rayon::join(
        || count_xors(&rn_x, &sq_steps),
        || count_xors(&rn_y, &tr_steps),
    );

    let rxn = unsafe { *rn_x.get_unchecked(N) } as usize;
    let ryn = unsafe { *rn_y.get_unchecked(N) } as usize;
    let target = unsafe { *table.get_unchecked(rxn * L + ryn) };

    let mut ans: i64 = 0;
    for n0 in 0..L {
        let cx = cnt_x[n0];
        if cx == 0 {
            continue;
        }
        let row = n0 * L;
        for n1 in 0..L {
            if unsafe { *table.get_unchecked(row + n1) } == target {
                ans += cx * cnt_y[n1];
            }
        }
    }

    println!("{}", ans);
}
