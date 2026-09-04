use rayon::prelude::*;

const N: usize = 1_000_000;
const L: usize = 512;

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

fn fill_rn(steps: &[usize]) -> Vec<u16> {
    let mut rn = vec![0u16; N + 1];
    let nst = steps.len();
    let mut kmax = 0usize;
    let mut used = [0u32; 512];
    for j in 1..=N {
        while kmax < nst && unsafe { *steps.get_unchecked(kmax) } <= j {
            kmax += 1;
        }
        let ju = j as u32;
        let prev = unsafe { *rn.get_unchecked(j - 1) };
        // Squares and triangular numbers both include s=1, yielding val = 0.
        used[0] = ju;
        let mut k = 1usize;
        let rp = rn.as_ptr();
        let sp = steps.as_ptr();
        let up = used.as_mut_ptr();
        while k + 8 <= kmax {
            unsafe {
                let v0 = (prev ^ *rp.add(j - *sp.add(k))) as usize;
                let v1 = (prev ^ *rp.add(j - *sp.add(k + 1))) as usize;
                let v2 = (prev ^ *rp.add(j - *sp.add(k + 2))) as usize;
                let v3 = (prev ^ *rp.add(j - *sp.add(k + 3))) as usize;
                let v4 = (prev ^ *rp.add(j - *sp.add(k + 4))) as usize;
                let v5 = (prev ^ *rp.add(j - *sp.add(k + 5))) as usize;
                let v6 = (prev ^ *rp.add(j - *sp.add(k + 6))) as usize;
                let v7 = (prev ^ *rp.add(j - *sp.add(k + 7))) as usize;
                *up.add(v0) = ju;
                *up.add(v1) = ju;
                *up.add(v2) = ju;
                *up.add(v3) = ju;
                *up.add(v4) = ju;
                *up.add(v5) = ju;
                *up.add(v6) = ju;
                *up.add(v7) = ju;
            }
            k += 8;
        }
        while k < kmax {
            unsafe {
                let s = *sp.add(k);
                let val = (prev ^ *rp.add(j - s)) as usize;
                *up.add(val) = ju;
            }
            k += 1;
        }
        let mut mex = 0u16;
        unsafe {
            while *up.add(mex as usize) == ju {
                mex += 1;
            }
            *rn.get_unchecked_mut(j) = prev ^ mex;
        }
    }
    rn
}

fn count_range(rn: &[u16], steps: &[usize], lo: usize, hi: usize) -> [i64; L] {
    let mut cnt = [0u32; L];
    if lo > hi {
        return [0i64; L];
    }
    let rp = rn.as_ptr();
    let cp = cnt.as_mut_ptr();
    for &s in steps {
        if s > hi {
            break;
        }
        let start = lo.max(s);
        let len = hi - start + 1;
        let mut offset = 0usize;
        let p_j = unsafe { rp.add(start) };
        let p_js = unsafe { rp.add(start - s) };
        let end4 = len / 4 * 4;
        while offset < end4 {
            unsafe {
                let v0 = (*p_j.add(offset) ^ *p_js.add(offset)) as usize;
                let v1 = (*p_j.add(offset + 1) ^ *p_js.add(offset + 1)) as usize;
                let v2 = (*p_j.add(offset + 2) ^ *p_js.add(offset + 2)) as usize;
                let v3 = (*p_j.add(offset + 3) ^ *p_js.add(offset + 3)) as usize;
                *cp.add(v0) += 1;
                *cp.add(v1) += 1;
                *cp.add(v2) += 1;
                *cp.add(v3) += 1;
            }
            offset += 4;
        }
        while offset < len {
            unsafe {
                let v = (*p_j.add(offset) ^ *p_js.add(offset)) as usize;
                *cp.add(v) += 1;
            }
            offset += 1;
        }
    }
    let mut res = [0i64; L];
    for i in 0..L {
        res[i] = cnt[i] as i64;
    }
    res
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

fn solve() -> i64 {
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
    ans
}

fn main() {
    println!("{}", solve());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_answer() {
        assert_eq!(solve(), 3996390106631);
    }
}
