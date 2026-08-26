// Problem 893: Matchsticks
// T(N) = sum M(n) for n=1..N, M(n) = min matchsticks to represent n
// with digits and +/* (standard precedence).
// Costs: 0=6,1=2,2=5,3=5,4=4,5=5,6=6,7=3,8=7,9=6; + and * cost 2.
// T(100) = 916.

const N: usize = 1_000_000;
const DCOST: [u8; 10] = [6, 2, 5, 5, 4, 5, 6, 3, 7, 6];
// Same cutoff as pcost[v] + 2 <= 17: higher-cost addends never beat a 1-term.
const MAX_ADD_P: u8 = 15;

fn main() {
    let mut m = vec![0u8; N + 1];
    for n in 1..=N {
        m[n] = m[n / 10] + DCOST[n % 10];
    }

    // Product-chain costs. Process a increasing so m[a] is final; m[b] may
    // still be a digit cost, but the swapped pair (b, a) is applied later.
    unsafe {
        let p = m.as_mut_ptr();
        for a in 2..=N / 2 {
            // SAFETY: a <= N/2 < m.len()
            let pa = *p.add(a) + 2;
            let max_b = N / a;
            let mut prod = a * 2;
            for b in 2..=max_b {
                // SAFETY: b <= N/a, prod = a*b <= N
                let cand = pa + *p.add(b);
                let d = p.add(prod);
                if cand < *d {
                    *d = cand;
                }
                prod += a;
            }
        }
    }

    let mut addends: Vec<(usize, u8)> = Vec::new();
    for v in 1..=N {
        let pc = m[v];
        if pc <= MAX_ADD_P {
            addends.push((v, pc + 2));
        }
    }

    #[cfg(target_arch = "x86_64")]
    let avx2 = is_x86_feature_detected!("avx2");
    #[cfg(not(target_arch = "x86_64"))]
    let avx2 = false;

    for &(v, add) in &addends {
        apply_addend(&mut m, v, add, avx2);
    }

    debug_assert_eq!(m[28], 9);
    debug_assert_eq!(m[1..=100].iter().map(|&x| x as u64).sum::<u64>(), 916);

    let total: u64 = m.iter().skip(1).map(|&x| x as u64).sum();
    println!("{}", total);
}

fn apply_addend(m: &mut [u8], v: usize, add: u8, avx2: bool) {
    let n_max = m.len() - 1;
    if v >= n_max {
        return;
    }
    unsafe {
        let p = m.as_mut_ptr();
        #[cfg(target_arch = "x86_64")]
        {
            if avx2 && v >= 32 {
                apply_addend_avx2(p, v, n_max, add);
                return;
            }
            if v >= 16 {
                apply_addend_sse2(p, v, n_max, add);
                return;
            }
        }
        // SAFETY: n in v+1..=n_max => n-v >= 1 and n < m.len()
        for n in v + 1..=n_max {
            let cand = *p.add(n - v) + add;
            let d = p.add(n);
            if cand < *d {
                *d = cand;
            }
        }
    }
}

#[cfg(target_arch = "x86_64")]
#[target_feature(enable = "avx2")]
unsafe fn apply_addend_avx2(p: *mut u8, v: usize, n_max: usize, add: u8) {
    use std::arch::x86_64::*;
    unsafe {
        let padd = _mm256_set1_epi8(add as i8);
        let mut n = v + 1;
        let end = n_max.saturating_sub(31);
        while n <= end {
            // SAFETY: n+31 <= n_max and n-v >= 1, so both 32-byte windows sit in
            // 1..=n_max. v >= 32, so src [n-v, n-v+31] does not overlap dest
            // [n, n+31]; later iterations read already-updated values (unbounded).
            let prev = _mm256_loadu_si256(p.add(n - v) as *const __m256i);
            let cur = _mm256_loadu_si256(p.add(n) as *const __m256i);
            let cand = _mm256_add_epi8(prev, padd);
            let res = _mm256_min_epu8(cur, cand);
            _mm256_storeu_si256(p.add(n) as *mut __m256i, res);
            n += 32;
        }
        while n <= n_max {
            let cand = *p.add(n - v) + add;
            let d = p.add(n);
            if cand < *d {
                *d = cand;
            }
            n += 1;
        }
    }
}

#[cfg(target_arch = "x86_64")]
#[target_feature(enable = "sse2")]
unsafe fn apply_addend_sse2(p: *mut u8, v: usize, n_max: usize, add: u8) {
    use std::arch::x86_64::*;
    unsafe {
        let padd = _mm_set1_epi8(add as i8);
        let mut n = v + 1;
        let end = n_max.saturating_sub(15);
        while n <= end {
            // SAFETY: n+15 <= n_max, n-v >= 1; v >= 16 so windows do not overlap.
            let prev = _mm_loadu_si128(p.add(n - v) as *const __m128i);
            let cur = _mm_loadu_si128(p.add(n) as *const __m128i);
            let cand = _mm_add_epi8(prev, padd);
            let res = _mm_min_epu8(cur, cand);
            _mm_storeu_si128(p.add(n) as *mut __m128i, res);
            n += 16;
        }
        while n <= n_max {
            let cand = *p.add(n - v) + add;
            let d = p.add(n);
            if cand < *d {
                *d = cand;
            }
            n += 1;
        }
    }
}
