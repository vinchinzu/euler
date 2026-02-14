// Project Euler 530 - GCD Sum
//
// Compute sum_{n=1}^{N} sum_{d|n} gcd(d, n/d) for N = 10^15.
// Uses Mobius function sieve and floor quotient summation.

const N: i64 = 1_000_000_000_000_000;

fn isqrt(n: i64) -> i64 {
    if n <= 0 { return 0; }
    let mut x = (n as f64).sqrt() as i64;
    while x > 0 && x * x > n { x -= 1; }
    while (x + 1) * (x + 1) <= n { x += 1; }
    x
}

fn icbrt(n: i64) -> i64 {
    if n <= 0 { return 0; }
    let mut x = (n as f64).cbrt() as i64;
    while x > 0 && x * x * x > n { x -= 1; }
    while (x + 1) * (x + 1) * (x + 1) <= n { x += 1; }
    x
}

fn sq(x: i64) -> i64 { x * x }

fn sum_floor_quotients(n: i64) -> i64 {
    if n <= 0 { return 0; }
    let mut result = 0i64;
    let mut i = 1i64;
    while i <= n {
        let q = n / i;
        let j = n / q;
        result += q * (j - i + 1);
        i = j + 1;
    }
    result
}

fn sum_powers_1(n: i64) -> i64 {
    if n <= 0 { return 0; }
    n * (n + 1) / 2
}

fn main() {
    let l = icbrt(N);
    let sqrt_n = isqrt(N) as usize;

    // Pre-compute Mobius function
    let mut mobius = vec![1i8; sqrt_n + 1];
    let mut is_prime = vec![true; sqrt_n + 1];
    is_prime[0] = false;
    if sqrt_n >= 1 { is_prime[1] = false; }

    for i in 2..=sqrt_n {
        if is_prime[i] {
            for j in (i..=sqrt_n).step_by(i) {
                if j > i { is_prime[j] = false; }
                if (j / i) % i == 0 {
                    mobius[j] = 0;
                } else {
                    mobius[j] = -mobius[j];
                }
            }
        }
    }

    let mut big = vec![0i64; (l + 2) as usize];
    let mut small = vec![0i64; (l + 2) as usize];

    for i in 1..=l {
        small[i as usize] = sum_floor_quotients(i);
    }
    for i in 1..=l {
        big[i as usize] = sum_floor_quotients(N / sq(i));
    }

    let mut ans: i64 = 0;

    let mut h = 1i64;
    while sq(h) <= N {
        if mobius[h as usize] == 0 { h += 1; continue; }

        let n_h = N / sq(h);
        let l_local = icbrt(n_h) / 10 + 1;
        let sqrt_n_over_l = isqrt(n_h / l_local);

        for g in 1..=sqrt_n_over_l {
            let gh = g * h;
            let term = if gh <= l {
                big[gh as usize]
            } else {
                let idx = n_h / sq(g);
                if idx <= l {
                    small[idx as usize]
                } else {
                    sum_floor_quotients(idx)
                }
            };
            ans += mobius[h as usize] as i64 * term * g;
        }

        for q in 1..l_local {
            let sqrt_n_q = isqrt(n_h / q);
            let sqrt_n_q1 = isqrt(n_h / (q + 1));
            let small_q = if q <= l { small[q as usize] } else { sum_floor_quotients(q) };
            ans += mobius[h as usize] as i64 * small_q * (sum_powers_1(sqrt_n_q) - sum_powers_1(sqrt_n_q1));
        }

        h += 1;
    }

    println!("{ans}");
}
