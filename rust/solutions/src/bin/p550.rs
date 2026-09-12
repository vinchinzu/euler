// Project Euler 550 - Divisor Game
//
// Compute Sprague-Grundy nimbers for the divisor game, then use
// Walsh-Hadamard XOR convolution to count losing positions for K rounds.

const N: usize = 10_000_000;
const K: u64 = 1_000_000_000_000;
const L: usize = 64;
const M: u64 = 987_654_321;

#[inline(always)]
fn mul_mod(a: u64, b: u64) -> u64 {
    a * b % M
}

fn power(mut base: u64, mut exp: u64) -> u64 {
    let mut result = 1u64;
    base %= M;
    while exp > 0 {
        if exp & 1 == 1 {
            result = mul_mod(result, base);
        }
        base = mul_mod(base, base);
        exp >>= 1;
    }
    result
}

fn mod_inverse(a: u64, m: u64) -> u64 {
    fn extended_gcd(a: i64, b: i64) -> (i64, i64, i64) {
        if a == 0 {
            return (b, 0, 1);
        }
        let (g, x1, y1) = extended_gcd(b % a, a);
        (g, y1 - (b / a) * x1, x1)
    }
    let (_, x, _) = extended_gcd((a % m) as i64, m as i64);
    let x = x % m as i64;
    if x < 0 { (x + m as i64) as u64 } else { x as u64 }
}

/// Generate all divisors of n from its prime factorization into `divs`.
/// Returns the number of divisors written.
#[inline(always)]
fn gen_divisors(factors: &[(u32, u8); 12], nfac: usize, divs: &mut [u32; 1024]) -> usize {
    divs[0] = 1;
    let mut nd = 1usize;
    for k in 0..nfac {
        let p = factors[k].0;
        let e = factors[k].1 as usize;
        let prev = nd;
        let mut mul = 1u32;
        for _ in 0..e {
            mul *= p;
            for i in 0..prev {
                divs[nd] = divs[i] * mul;
                nd += 1;
            }
        }
    }
    nd
}

fn helper(
    min_idx: usize,
    n: usize,
    nfac: usize,
    factors: &mut [(u32, u8); 12],
    divs: &mut [u32; 1024],
    primes: &[u32],
    num_primes: &[u32],
    nimbers: &mut [u8],
    counts: &mut [i64; L],
) {
    if n > 1 {
        let nd = gen_divisors(factors, nfac, divs);
        let mut used: u64 = 0;
        for i in 0..nd {
            let di = divs[i] as usize;
            if di == 1 || di == n {
                continue;
            }
            let ni = nimbers[di];
            for j in i..nd {
                let dj = divs[j] as usize;
                if dj == 1 || dj == n {
                    continue;
                }
                used |= 1u64 << (ni ^ nimbers[dj]);
            }
        }
        let nim = used.trailing_ones() as u8;
        nimbers[n] = nim;
        counts[nim as usize] += 1;
    }

    let n64 = n as u64;
    for idx in min_idx..primes.len() {
        let p = primes[idx];
        let p64 = p as u64;
        if n64.saturating_mul(p64) > N as u64 {
            break;
        }

        factors[nfac] = (p, 0);
        let mut new_n = n;
        while (new_n as u64) * p64 <= N as u64 {
            new_n *= p as usize;
            factors[nfac].1 += 1;
            helper(
                idx + 1,
                new_n,
                nfac + 1,
                factors,
                divs,
                primes,
                num_primes,
                nimbers,
                counts,
            );
        }

        // Tail: remaining primes only multiply once
        if n64 * p64 * p64 > N as u64 {
            if idx > 0 {
                let prev_p = primes[idx - 1];
                let add = num_primes[N / n] as i64 - num_primes[prev_p as usize] as i64 - 1;
                if add > 0 {
                    counts[nimbers[n * p as usize] as usize] += add;
                }
            }
            return;
        }
    }
}

fn fwht(arr: &mut [u64; L]) {
    let mut len = 1usize;
    while len < L {
        let mut i = 0usize;
        while i < L {
            for j in 0..len {
                let u = arr[i + j];
                let v = arr[i + j + len];
                arr[i + j] = (u + v) % M;
                arr[i + j + len] = (u + M - v) % M;
            }
            i += len << 1;
        }
        len <<= 1;
    }
}

fn main() {
    let mut is_prime = vec![true; N + 1];
    is_prime[0] = false;
    is_prime[1] = false;
    {
        let mut i = 2usize;
        while i * i <= N {
            if is_prime[i] {
                let mut j = i * i;
                while j <= N {
                    is_prime[j] = false;
                    j += i;
                }
            }
            i += 1;
        }
    }

    let mut primes: Vec<u32> = Vec::new();
    let mut num_primes = vec![0u32; N + 1];
    let mut cnt = 0u32;
    for i in 0..=N {
        if i >= 2 && is_prime[i] {
            primes.push(i as u32);
            cnt += 1;
        }
        num_primes[i] = cnt;
    }

    let mut nimbers = vec![0u8; N + 1];
    let mut counts = [0i64; L];
    let mut factors = [(0u32, 0u8); 12];
    let mut divs = [0u32; 1024];

    helper(
        0,
        1,
        0,
        &mut factors,
        &mut divs,
        &primes,
        &num_primes,
        &mut nimbers,
        &mut counts,
    );

    let mut arr = [0u64; L];
    for i in 0..L {
        arr[i] = (counts[i] % M as i64) as u64;
    }

    fwht(&mut arr);
    for i in 0..L {
        arr[i] = power(arr[i], K);
    }
    fwht(&mut arr);

    let inv_n = mod_inverse(L as u64, M);
    for i in 0..L {
        arr[i] = mul_mod(arr[i], inv_n);
    }

    let mut ans: u64 = 0;
    for i in 1..L {
        ans += arr[i];
        if ans >= M {
            ans -= M;
        }
    }

    println!("{ans}");
}
