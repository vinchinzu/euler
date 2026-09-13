// Project Euler 875 - Quadruple Congruence
// Linear sieve with multiplicative function q

const M: u64 = 1001961001;

#[inline(always)]
fn power(mut a: u64, mut b: u64) -> u64 {
    let mut res: u64 = 1;
    a %= M;
    while b > 0 {
        if b & 1 == 1 { res = res * a % M; }
        a = a * a % M;
        b >>= 1;
    }
    res
}

fn main() {
    let n: usize = 12345678;

    let mut spf = vec![0u32; n + 1];
    let mut pe = vec![0u32; n + 1];
    let mut q = vec![0u64; n + 1];
    let mut primes = Vec::with_capacity(n / 2);
    let mut p3_cache = vec![0u64; n + 1];
    let mut p7_cache = vec![0u64; n + 1];

    q[1] = 1;

    for i in 2..=n {
        if spf[i] == 0 {
            spf[i] = i as u32;
            pe[i] = i as u32;
            primes.push(i);

            let p = i as u64;
            if p == 2 {
                q[i] = 128;
                p3_cache[i] = 8;
                p7_cache[i] = 128;
            } else {
                let p3 = power(p, 3);
                let p7 = power(p, 7);
                p3_cache[i] = p3;
                p7_cache[i] = p7;
                let term2 = (p - 1) % M * p3 % M;
                q[i] = (p7 + term2) % M;
            }
        }

        let mut j = 0;
        while j < primes.len() {
            let p = unsafe { *primes.get_unchecked(j) };
            let spf_i = unsafe { *spf.get_unchecked(i) };
            if p > spf_i as usize || i * p > n { break; }

            let next_val = i * p;
            unsafe { *spf.get_unchecked_mut(next_val) = p as u32; }

            if p == spf_i as usize {
                let prev_pk = unsafe { *pe.get_unchecked(i) } as usize;
                let next_pk = prev_pk * p;
                unsafe { *pe.get_unchecked_mut(next_val) = next_pk as u32; }

                let pp = p as u64;
                let q_prev_pk = unsafe { *q.get_unchecked(prev_pk) };
                let q_next_pk = if p == 2 {
                    let term = power(prev_pk as u64, 4) * 128 % M;
                    (128 * q_prev_pk % M + term) % M
                } else {
                    let p3 = unsafe { *p3_cache.get_unchecked(p) };
                    let p7 = unsafe { *p7_cache.get_unchecked(p) };
                    let mut term = power(prev_pk as u64, 4);
                    term = term * p3 % M;
                    term = term * ((pp - 1) % M) % M;
                    (p7 * q_prev_pk % M + term) % M
                };

                if next_val == next_pk {
                    unsafe { *q.get_unchecked_mut(next_val) = q_next_pk; }
                } else {
                    let rest = next_val / next_pk;
                    let q_rest = unsafe { *q.get_unchecked(rest) };
                    unsafe { *q.get_unchecked_mut(next_val) = q_next_pk * q_rest % M; }
                }
            } else {
                unsafe { 
                    *pe.get_unchecked_mut(next_val) = p as u32;
                    let q_i = *q.get_unchecked(i);
                    let q_p = *q.get_unchecked(p);
                    *q.get_unchecked_mut(next_val) = q_i * q_p % M;
                }
            }
            j += 1;
        }
    }

    let mut total: u64 = 0;
    for i in 1..=n {
        total += unsafe { *q.get_unchecked(i) };
        if i % 1000 == 0 {
            total %= M;
        }
    }
    total %= M;

    println!("{}", total);
}
