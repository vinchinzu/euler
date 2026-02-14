// Project Euler 241: Perfection Quotients
//
// Find the sum of all positive integers n <= 10^18 such that sigma(n)/n
// is of the form (2k+1)/2.
// Recursive search over prime power factorizations with exact rational arithmetic.

const NMAX: i64 = 1_000_000_000_000_000_000; // 10^18
const L: usize = 1_000_000;

static mut ANS: i64 = 0;

fn build_sieve() -> (Vec<i32>, Vec<i32>) {
    let mut is_prime = vec![true; L + 1];
    let mut spf = vec![0i32; L + 1];
    is_prime[0] = false;
    is_prime[1] = false;
    for i in 2..=L {
        spf[i] = i as i32;
    }
    let mut i = 2;
    while i * i <= L {
        if is_prime[i] {
            let mut j = i * i;
            while j <= L {
                is_prime[j] = false;
                if spf[j] == j as i32 {
                    spf[j] = i as i32;
                }
                j += i;
            }
        }
        i += 1;
    }
    let primes: Vec<i32> = (2..=L).filter(|&i| is_prime[i]).map(|i| i as i32).collect();
    (spf, primes)
}

fn gcd(mut a: i64, mut b: i64) -> i64 {
    if a < 0 { a = -a; }
    if b < 0 { b = -b; }
    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }
    a
}

fn helper(prod_val: i64, r_num: i64, r_den: i64, spf: &[i32]) {
    if r_num == r_den {
        unsafe { ANS += prod_val; }
        return;
    }

    if r_den > L as i64 {
        return;
    }

    let p: i64 = if (r_den as usize) < spf.len() {
        spf[r_den as usize] as i64
    } else {
        r_den
    };

    if p > 1 && prod_val % p != 0 {
        let mut pe: i64 = 1;
        let mut mult: i64 = 1; // 1 + p + p^2 + ...
        loop {
            if pe > NMAX / p {
                break;
            }
            pe *= p;
            if prod_val > NMAX / pe {
                break;
            }
            mult += pe;

            let new_num_raw = r_num * pe;
            let new_den_raw = r_den * mult;
            let g = gcd(new_num_raw, new_den_raw);
            let new_num = new_num_raw / g;
            let new_den = new_den_raw / g;

            helper(prod_val * pe, new_num, new_den, spf);
        }
    }
}

fn main() {
    let (spf, primes) = build_sieve();

    // Compute max perfection quotient
    let mut max_pq: f64 = 1.0;
    let mut prod: i64 = 1;
    for &p in &primes {
        let p64 = p as i64;
        if prod > NMAX / p64 {
            break;
        }
        prod *= p64;
        max_pq *= p64 as f64 / (p64 - 1) as f64;
    }

    for k in 1..(max_pq as i32) {
        let t_num = 2 * k as i64 + 1;
        let t_den = 2i64;
        helper(1, t_num, t_den, &spf);
    }

    println!("{}", unsafe { ANS });
}
