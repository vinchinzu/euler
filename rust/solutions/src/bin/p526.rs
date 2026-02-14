// Project Euler 526 - Largest Prime Factor Sum
//
// N=10^16, K=9. Find the largest sum of largest prime factors
// of K consecutive integers. Uses residue states mod product of prime powers.

const K: usize = 9;
const L: usize = 30;

type I64 = i64;

#[derive(Clone)]
struct State {
    a: I64,
    denom: [I64; K],
}

fn prime_power_base(n: i32) -> i32 {
    let mut m = n;
    let mut d = 2;
    while d * d <= m {
        if m % d == 0 {
            while m % d == 0 { m /= d; }
            return if m == 1 { d } else { 0 };
        }
        d += 1;
    }
    if m > 1 { m } else { 0 }
}

fn sum_inv(denom: &[I64; K]) -> f64 {
    denom.iter().map(|&d| 1.0 / d as f64).sum()
}

fn sieve_primes(limit: usize) -> Vec<i32> {
    let mut is_prime = vec![true; limit + 1];
    is_prime[0] = false;
    if limit >= 1 { is_prime[1] = false; }
    let mut i = 2;
    while i * i <= limit {
        if is_prime[i] {
            let mut j = i * i;
            while j <= limit { is_prime[j] = false; j += i; }
        }
        i += 1;
    }
    (2..=limit).filter(|&i| is_prime[i]).map(|i| i as i32).collect()
}

fn main() {
    let n: I64 = 10_000_000_000_000_000;

    let mut states = vec![State {
        a: 0,
        denom: [1; K],
    }];

    let mut modulus: I64 = 1;

    for pe in 2..=L as i32 {
        let p = prime_power_base(pe);
        if p == 0 { continue; }

        let mut max_sum = -1.0f64;
        let mut new_states: Vec<State> = Vec::new();

        for si in 0..states.len() {
            for ii in 0..p {
                let start = states[si].a + ii as I64 * modulus;
                let mut nd = states[si].denom;
                for j in 0..K {
                    if (start + j as I64) % pe as I64 == 0 {
                        nd[j] *= p as I64;
                    }
                }
                let sr = sum_inv(&nd);
                if sr > max_sum + 1e-12 {
                    new_states.clear();
                    max_sum = sr;
                }
                if sr > max_sum - 1e-12 {
                    new_states.push(State { a: start, denom: nd });
                }
            }
        }

        states = new_states;
        modulus *= p as I64;
    }

    let sqrt_n = (n as f64).sqrt() as usize + 1;
    let all_primes = sieve_primes(sqrt_n);

    let lp_start = all_primes.iter().position(|&p| p > L as i32).unwrap_or(all_primes.len());
    let large_primes = &all_primes[lp_start..];

    let mut ans: I64 = 0;
    let mut base = (n / modulus) * modulus;

    while base >= 0 && ans == 0 {
        for si in 0..states.len() {
            let num = base + states[si].a;
            if num > n || num <= 0 { continue; }

            let mut good = true;
            for &p in large_primes {
                let r = num % p as I64;
                if r == 0 || r > p as I64 - K as I64 {
                    good = false;
                    break;
                }
            }
            if good {
                let mut h: I64 = 0;
                for j in 0..K {
                    h += (num + j as I64) / states[si].denom[j];
                }
                if h > ans { ans = h; }
            }
        }
        if ans != 0 { break; }
        base -= modulus;
    }

    println!("{ans}");
}
