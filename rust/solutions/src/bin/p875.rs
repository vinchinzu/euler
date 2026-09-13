// Project Euler 875 - Quadruple Congruence
// Linear sieve with multiplicative function q

const M: i64 = 1001961001;

fn power(mut a: i64, mut b: i64, m: i64) -> i64 {
    let mut res: i64 = 1;
    a = a.rem_euclid(m);
    while b > 0 {
        if b & 1 == 1 { res = (res as i128 * a as i128 % m as i128) as i64; }
        a = (a as i128 * a as i128 % m as i128) as i64;
        b >>= 1;
    }
    res
}

fn main() {
    let n: usize = 12345678;

    let mut spf = vec![0u32; n + 1];
    let mut pe = vec![0u32; n + 1];
    let mut q = vec![0i64; n + 1];
    let mut primes = Vec::with_capacity(n / 2);

    q[1] = 1;

    for i in 2..=n {
        if spf[i] == 0 {
            spf[i] = i as u32;
            pe[i] = i as u32;
            primes.push(i);

            let p = i as i64;
            if p == 2 {
                q[i] = 128;
            } else {
                let p3 = power(p, 3, M);
                let p7 = power(p, 7, M);
                let term2 = (p - 1) % M * p3 % M;
                q[i] = (p7 + term2) % M;
            }
        }

        let mut j = 0;
        while j < primes.len() {
            let p = primes[j];
            if p > spf[i] as usize || i * p > n { break; }

            let next_val = i * p;
            spf[next_val] = p as u32;

            if p == spf[i] as usize {
                let prev_pk = pe[i] as usize;
                let next_pk = prev_pk * p;
                pe[next_val] = next_pk as u32;

                let pp = p as i64;
                let q_next_pk = if p == 2 {
                    let term = power(prev_pk as i64, 4, M) * 128 % M;
                    (128 * q[prev_pk] % M + term) % M
                } else {
                    let p3 = power(pp, 3, M);
                    let p7 = power(pp, 7, M);
                    let mut term = power(prev_pk as i64, 4, M);
                    term = term * p3 % M;
                    term = term * ((pp - 1) % M) % M;
                    (p7 * q[prev_pk] % M + term) % M
                };

                if next_val == next_pk {
                    q[next_val] = q_next_pk;
                } else {
                    let rest = next_val / next_pk;
                    q[next_val] = q_next_pk * q[rest] % M;
                }
            } else {
                pe[next_val] = p as u32;
                q[next_val] = q[i] * q[p] % M;
            }
            j += 1;
        }
    }

    let mut total: i64 = 0;
    for i in 1..=n {
        total = (total + q[i]) % M;
    }

    println!("{}", total);
}
