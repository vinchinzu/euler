// Project Euler 485 - Maximum number of divisors
//
// Linear sieve for divisor counts + sliding window maximum over windows of size K.
// Uses multiplicative property: d(p^a * m) = (a+1) * d(m) when gcd(p,m) = 1.

const N: usize = 100_000_000;
const K: usize = 100_000;

fn main() {
    // Linear sieve to compute divisor count d(n) in O(N).
    // spf[n] = smallest prime factor of n
    // We also track e[n] = exponent of spf in n, and d[n] = divisor count.
    let mut divs = vec![0u16; N + 1];
    let mut exp = vec![0u8; N + 1]; // exponent of smallest prime factor
    let mut primes = Vec::with_capacity(6_000_000); // pi(10^8) ~ 5.7M

    divs[1] = 1;

    for i in 2..=N {
        if divs[i] == 0 {
            // i is prime: d(i) = 2, exponent of spf = 1
            divs[i] = 2;
            exp[i] = 1;
            primes.push(i);
        }
        for &p in &primes {
            let ip = i * p;
            if ip > N {
                break;
            }
            if i % p == 0 {
                // p divides i, so spf(i*p) = p and exponent increases
                let e = exp[i] + 1;
                exp[ip] = e;
                // d(i*p) = d(i) / (exp[i]+1) * (exp[i]+2)
                // = d(i) * (e+1) / e  where e = exp[i]+1 = exp[ip]
                divs[ip] = (divs[i] as u32 * (e as u32 + 1) / e as u32) as u16;
                break;
            } else {
                // p does not divide i, so spf(i*p) = p, exponent = 1
                exp[ip] = 1;
                // d(i*p) = d(i) * d(p) = d(i) * 2
                divs[ip] = divs[i] * 2;
            }
        }
    }

    // Free exp memory before sliding window
    drop(exp);

    // Sliding window maximum using monotonic deque
    // Use a circular buffer approach with raw array for speed
    let mut deque = vec![0u32; N + 1];
    let mut head: usize = 0;
    let mut tail: usize = 0;
    let mut sum: u64 = 0;

    for i in 1..=N {
        // SAFETY: i is in bounds [1, N], divs has size N+1
        unsafe {
            let di = *divs.get_unchecked(i);
            while tail > head && *divs.get_unchecked(*deque.get_unchecked(tail - 1) as usize) <= di {
                tail -= 1;
            }
            *deque.get_unchecked_mut(tail) = i as u32;
            tail += 1;

            if i >= K {
                let left = (i - K + 1) as u32;
                while *deque.get_unchecked(head) < left {
                    head += 1;
                }
                sum += *divs.get_unchecked(*deque.get_unchecked(head) as usize) as u64;
            }
        }
    }

    println!("{}", sum);
}
