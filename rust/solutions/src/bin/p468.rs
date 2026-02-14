// Project Euler 468 - Smooth divisors of binomial coefficients
// Segment tree approach for range multiplication.

const N: usize = 11_111_111;
const M: i64 = 1_000_000_993;

fn main() {
    // Sieve
    let mut is_prime = vec![false; N + 1];
    for i in 2..=N {
        is_prime[i] = true;
    }
    {
        let mut i = 2;
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

    // Modular inverses
    let mut mod_invs = vec![0i64; N + 1];
    mod_invs[1] = 1;
    for i in 2..=N {
        mod_invs[i] = (M - (M / i as i64) * mod_invs[(M % i as i64) as usize] % M) % M;
    }

    // L = ilog2(N)
    let mut l_val = 0;
    {
        let mut t = N;
        while t > 0 {
            l_val += 1;
            t >>= 1;
        }
    }

    // L2 = smallest power of 2 >= N/2 + 1, doubled
    let mut l2 = 1usize;
    while l2 < N / 2 + 1 {
        l2 *= 2;
    }
    l2 *= 2;

    // S[r] for r=0..N/2
    let half = N / 2;
    let mut s = vec![1i64; half + 1];

    let mut ans: i64 = 0;

    // Small B < L
    for b in 1..l_val {
        if is_prime[b] {
            let bb = b as i64;
            let mut prod = 1i64;
            for r in 1..=half {
                let mut nn = N + 1 - r;
                while nn % b == 0 {
                    prod = (prod as i128 * bb as i128 % M as i128) as i64;
                    nn /= b;
                }
                let mut nn = r;
                while nn % b == 0 {
                    prod = (prod as i128 * mod_invs[b] as i128 % M as i128) as i64;
                    nn /= b;
                }
                s[r] = (s[r] as i128 * prod as i128 % M as i128) as i64;
            }
        }
        for r in 0..=half {
            ans = (ans + 2 * s[r]) % M;
        }
    }

    // Segment tree
    let mut mults = vec![1i64; l2];
    let mut sums = vec![0i64; l2];
    for i in 0..=half {
        sums[l2 / 2 + i] = s[i];
    }
    for i in (1..l2 / 2).rev() {
        sums[i] = ((mults[2 * i] as i128 * sums[2 * i] as i128
            + mults[2 * i + 1] as i128 * sums[2 * i + 1] as i128)
            % M as i128) as i64;
    }

    let multiply_range = |start: usize, mult: i64, mults: &mut Vec<i64>, sums: &mut Vec<i64>| {
        let mut i = start + l2 / 2;
        while i % 2 == 0 {
            i /= 2;
        }
        loop {
            mults[i] = (mults[i] as i128 * mult as i128 % M as i128) as i64;
            while i % 2 != 0 {
                i /= 2;
                sums[i] = ((mults[2 * i] as i128 * sums[2 * i] as i128
                    + mults[2 * i + 1] as i128 * sums[2 * i + 1] as i128)
                    % M as i128) as i64;
                if i == 0 {
                    return;
                }
            }
            i += 1;
        }
    };

    // Large B >= L
    for b in l_val..=N {
        if is_prime[b] {
            let bb = b as i64;
            let mut r = N % b + 1;
            while r <= half {
                let mut nn = N + 1 - r;
                while nn % b == 0 {
                    multiply_range(r, bb, &mut mults, &mut sums);
                    nn /= b;
                }
                r += b;
            }
            r = b;
            while r <= half {
                let mut nn = r;
                while nn % b == 0 {
                    multiply_range(r, mod_invs[b], &mut mults, &mut sums);
                    nn /= b;
                }
                r += b;
            }
        }
        ans = (ans + 2 * sums[1]) % M;
    }

    println!("{}", ans);
}
