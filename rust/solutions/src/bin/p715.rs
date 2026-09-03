use rayon::prelude::*;

const M: i64 = 1_000_000_007;
const INV2: i64 = (M + 1) / 2;

#[inline(always)]
fn imod(a: i64) -> i64 {
    let r = a % M;
    if r < 0 { r + M } else { r }
}

#[inline(always)]
fn sum_cubes(n: i64) -> i64 {
    // (n(n+1)/2)^2 mod M; M^2 fits in i64 so no i128.
    let n = n % M;
    let s = n * ((n + 1) % M) % M * INV2 % M;
    s * s % M
}

#[inline(always)]
fn isqrt(n: u64) -> u64 {
    let mut x = (n as f64).sqrt() as u64;
    while x.saturating_mul(x) > n {
        x -= 1;
    }
    while x + 1 <= n / (x + 1) {
        x += 1;
    }
    x
}


#[inline(always)]
fn compute_big(i: usize, big: &[i64], small: &[i32], big_n: u64, l1: usize) -> i64 {
    let ni = big_n / i as u64;
    let sqrtni = isqrt(ni) as usize;
    let mut acc = 1i64;

    let k_split = (l1 + i - 1) / i;
    let k_lim = sqrtni;

    let k_big_end = k_lim.min(k_split);
    let mut k = 3usize;
    if k < k_big_end && (k & 3) == 1 {
        let val = unsafe { *big.get_unchecked(i * k) };
        acc -= val;
        k += 2;
    }
    while k + 2 < k_big_end {
        unsafe {
            acc += *big.get_unchecked(i * k);
            acc -= *big.get_unchecked(i * (k + 2));
        }
        k += 4;
    }
    if k < k_big_end {
        acc += unsafe { *big.get_unchecked(i * k) };
        k += 2;
    }

    if k < 3 {
        k = 3;
    }
    if k < k_lim && (k & 1) == 0 {
        k += 1;
    }
    if k < k_lim && (k & 3) == 1 {
        let idx = (((ni / k as u64) + 1) >> 1) as usize;
        acc -= unsafe { *small.get_unchecked(idx) } as i64;
        k += 2;
    }
    if ni <= u32::MAX as u64 {
        let ni32 = ni as u32;
        while k + 6 < k_lim {
            unsafe {
                let idx0 = (((ni32 / k as u32) + 1) >> 1) as usize;
                let idx1 = (((ni32 / (k as u32 + 2)) + 1) >> 1) as usize;
                let idx2 = (((ni32 / (k as u32 + 4)) + 1) >> 1) as usize;
                let idx3 = (((ni32 / (k as u32 + 6)) + 1) >> 1) as usize;
                acc += *small.get_unchecked(idx0) as i64;
                acc -= *small.get_unchecked(idx1) as i64;
                acc += *small.get_unchecked(idx2) as i64;
                acc -= *small.get_unchecked(idx3) as i64;
            }
            k += 8;
        }
        while k + 2 < k_lim {
            unsafe {
                let idx0 = (((ni32 / k as u32) + 1) >> 1) as usize;
                let idx1 = (((ni32 / (k as u32 + 2)) + 1) >> 1) as usize;
                acc += *small.get_unchecked(idx0) as i64;
                acc -= *small.get_unchecked(idx1) as i64;
            }
            k += 4;
        }
        if k < k_lim {
            let idx = (((ni32 / k as u32) + 1) >> 1) as usize;
            acc += unsafe { *small.get_unchecked(idx) } as i64;
        }

        let max_t = (ni32 / sqrtni as u32) as usize;
        let mut chi_nit = ((ni32 + 1) >> 1) as i64 & 1;
        for t in 1..=max_t {
            let nit1 = ni32 / (t as u32 + 1);
            let chi_nit1 = ((nit1 + 1) >> 1) as i64 & 1;
            let diff = chi_nit - chi_nit1;
            if diff != 0 {
                acc -= diff * unsafe { *small.get_unchecked((t + 1) >> 1) } as i64;
            }
            chi_nit = chi_nit1;
        }
    } else {
        while k + 6 < k_lim {
            unsafe {
                let idx0 = (((ni / k as u64) + 1) >> 1) as usize;
                let idx1 = (((ni / (k as u64 + 2)) + 1) >> 1) as usize;
                let idx2 = (((ni / (k as u64 + 4)) + 1) >> 1) as usize;
                let idx3 = (((ni / (k as u64 + 6)) + 1) >> 1) as usize;
                acc += *small.get_unchecked(idx0) as i64;
                acc -= *small.get_unchecked(idx1) as i64;
                acc += *small.get_unchecked(idx2) as i64;
                acc -= *small.get_unchecked(idx3) as i64;
            }
            k += 8;
        }
        while k + 2 < k_lim {
            unsafe {
                let idx0 = (((ni / k as u64) + 1) >> 1) as usize;
                let idx1 = (((ni / (k as u64 + 2)) + 1) >> 1) as usize;
                acc += *small.get_unchecked(idx0) as i64;
                acc -= *small.get_unchecked(idx1) as i64;
            }
            k += 4;
        }
        if k < k_lim {
            let idx = (((ni / k as u64) + 1) >> 1) as usize;
            acc += unsafe { *small.get_unchecked(idx) } as i64;
        }

        let max_t = (ni / sqrtni as u64) as usize;
        let mut chi_nit = ((ni + 1) >> 1) as i64 & 1;
        for t in 1..=max_t {
            let nit1 = ni / (t as u64 + 1);
            let chi_nit1 = ((nit1 + 1) >> 1) as i64 & 1;
            let diff = chi_nit - chi_nit1;
            if diff != 0 {
                acc -= diff * unsafe { *small.get_unchecked((t + 1) >> 1) } as i64;
            }
            chi_nit = chi_nit1;
        }
    }

    imod(acc)
}

fn main() {
    let big_n: u64 = 1_000_000_000_000;
    let mut l1 = (big_n as f64).cbrt() as u64;
    while l1 * l1 * l1 > big_n {
        l1 -= 1;
    }
    while (l1 + 1) * (l1 + 1) * (l1 + 1) <= big_n {
        l1 += 1;
    }
    let l1 = l1 as usize;
    let l2 = (big_n / l1 as u64) as usize;

    let m = (l2 + 1) / 2;
    let mut small_i8 = vec![0i8; m + 1];
    let mut primes: Vec<u32> = Vec::with_capacity(2_065_000);
    small_i8[1] = 1;

    let sieve_limit = l2 / 3;
    for i in (3..=sieve_limit).step_by(2) {
        let s_idx = (i >> 1) + 1;
        unsafe {
            let mut mu_i = *small_i8.get_unchecked(s_idx);
            let is_prime = mu_i == 0;
            if is_prime {
                primes.push(i as u32);
                mu_i = if (s_idx & 1) == 1 { -1 } else { 1 };
                *small_i8.get_unchecked_mut(s_idx) = mu_i;
            }
            let iu = i as u32;
            let max_p = (l2 / i) as u32;
            if is_prime {
                if i <= 10_000 {
                    for &p in &primes {
                        if p > max_p {
                            break;
                        }
                        let v = iu * p;
                        let v_s_idx = ((v >> 1) + 1) as usize;
                        if p == iu {
                            *small_i8.get_unchecked_mut(v_s_idx) = 2;
                            break;
                        }
                        let mu_v = if (p & 3) == 1 { -mu_i } else { mu_i };
                        *small_i8.get_unchecked_mut(v_s_idx) = mu_v;
                    }
                } else {
                    for &p in &primes {
                        if p > max_p {
                            break;
                        }
                        let v = iu * p;
                        let v_s_idx = ((v >> 1) + 1) as usize;
                        let mu_v = if (p & 3) == 1 { -mu_i } else { mu_i };
                        *small_i8.get_unchecked_mut(v_s_idx) = mu_v;
                    }
                }
            } else if mu_i == 2 {
                for &p in &primes {
                    if p > max_p {
                        break;
                    }
                    let v = iu * p;
                    let v_s_idx = ((v >> 1) + 1) as usize;
                    *small_i8.get_unchecked_mut(v_s_idx) = 2;
                    if iu % p == 0 {
                        break;
                    }
                }
            } else {
                for &p in &primes {
                    if p > max_p {
                        break;
                    }
                    let v = iu * p;
                    let v_s_idx = ((v >> 1) + 1) as usize;
                    if iu % p == 0 {
                        *small_i8.get_unchecked_mut(v_s_idx) = 2;
                        break;
                    }
                    let mu_v = if (p & 3) == 1 { -mu_i } else { mu_i };
                    *small_i8.get_unchecked_mut(v_s_idx) = mu_v;
                }
            }
        }
    }
    drop(primes);

    let mut small = Vec::with_capacity(m + 1);
    unsafe {
        small.set_len(m + 1);
        *small.get_unchecked_mut(0) = 0;
    }
    let num_chunks = 64;
    let chunk_size = (m + num_chunks - 1) / num_chunks;

    let chunk_sums: Vec<i32> = (0..num_chunks)
        .into_par_iter()
        .map(|c| {
            let start = 1 + c * chunk_size;
            let end = (start + chunk_size).min(m + 1);
            let mut sum = 0i32;
            for s_idx in start..end {
                let v = unsafe { *small_i8.get_unchecked(s_idx) };
                let delta = if v == 0 {
                    if (s_idx & 1) == 1 { -1 } else { 1 }
                } else if v == 2 {
                    0
                } else {
                    v as i32
                };
                sum += delta;
            }
            sum
        })
        .collect();

    // Pass 1.5: sequential prefix sum of chunk sums
    let mut chunk_offsets = vec![0i32; chunk_sums.len() + 1];
    for i in 0..chunk_sums.len() {
        chunk_offsets[i + 1] = chunk_offsets[i] + chunk_sums[i];
    }

    // Pass 2: fill small in parallel
    let small_slice = &mut small[1..];
    small_slice
        .par_chunks_mut(chunk_size)
        .enumerate()
        .for_each(|(c, chunk)| {
            let mut cur = chunk_offsets[c];
            let start_idx = 1 + c * chunk_size;
            for (offset, val) in chunk.iter_mut().enumerate() {
                let s_idx = start_idx + offset;
                let v = unsafe { *small_i8.get_unchecked(s_idx) };
                let delta = if v == 0 {
                    if (s_idx & 1) == 1 { -1 } else { 1 }
                } else if v == 2 {
                    0
                } else {
                    v as i32
                };
                cur += delta;
                *val = cur;
            }
        });
    drop(small_i8);

    let mut big = vec![0i64; l1 + 2];
    let big_ptr = big.as_mut_ptr() as usize;
    let mut upper = l1;
    while upper >= 1 {
        let lower = (upper / 3) + 1;
        (lower..=upper).into_par_iter().for_each(|i| {
            let val = compute_big(i, &big, &small, big_n, l1);
            unsafe {
                let ptr = big_ptr as *mut i64;
                *ptr.add(i) = val;
            }
        });
        upper = lower - 1;
    }

    // Part A: i in [1, 1_000_000] where each block has r = i.
    // For even i, mu_sum = 0, so only odd i contribute!
    let sqrt_n = 1_000_000u64;
    let ans_a: i64 = (1..=sqrt_n / 2)
        .into_par_iter()
        .map(|k| {
            let i = 2 * k - 1;
            let q = big_n / i;
            let r_idx = ((i + 1) >> 1) as usize;
            let l_idx = (i >> 1) as usize;
            let mu_sum = unsafe { (*small.get_unchecked(r_idx) - *small.get_unchecked(l_idx)) as i64 };
            sum_cubes(q as i64) * mu_sum % M
        })
        .reduce(|| 0i64, |a, b| (a + b) % M);

    // Part B: q in [l1, sqrt_n - 1]
    let l1u = l1 as u64;
    let l2u = l2 as u64;
    let ans_b: i64 = (l1u..sqrt_n)
        .into_par_iter()
        .map(|q| {
            let i = big_n / (q + 1) + 1;
            let r = (big_n / q).min(l2u);
            let r_idx = ((r + 1) >> 1) as usize;
            let l_idx = (i >> 1) as usize;
            let mu_sum = unsafe { (*small.get_unchecked(r_idx) - *small.get_unchecked(l_idx)) as i64 };
            sum_cubes(q as i64) * mu_sum % M
        })
        .reduce(|| 0i64, |a, b| (a + b) % M);

    let mut ans = (ans_a + ans_b) % M;

    for t in 1..l1 {
        let sc = sum_cubes(t as i64);
        let diff = big[t] - big[t + 1];
        ans = (ans + sc * diff) % M;
    }

    println!("{}", imod(ans));
}
