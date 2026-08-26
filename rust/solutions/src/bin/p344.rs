// Project Euler 344 - Silver Dollar Game
// Losing positions: xor-zero gap vectors counted by bit-column carry DP (O(m^2 log n)).

fn mod_pow(mut base: u64, mut exp: u64, m: u64) -> u64 {
    let mut r = 1u64;
    base %= m;
    while exp > 0 {
        if exp & 1 == 1 {
            r = r * base % m;
        }
        base = base * base % m;
        exp >>= 1;
    }
    r
}

fn binom_mod(n: u64, mut k: u64, p: u64) -> u64 {
    if k > n {
        return 0;
    }
    k = k.min(n - k);
    let mut num = 1u64;
    let mut den = 1u64;
    for i in 1..=k {
        num = num * (n - k + i) % p;
        den = den * i % p;
    }
    num * mod_pow(den, p - 2, p) % p
}

fn binom_row(n: usize, p: u64) -> [u64; 128] {
    let mut c = [0u64; 128];
    c[0] = 1;
    for i in 1..=n {
        for j in (1..=i).rev() {
            let s = c[j] + c[j - 1];
            c[j] = if s >= p { s - p } else { s };
        }
    }
    c
}

fn build_ways(active: usize, passive: usize, p: u64) -> [u64; 128] {
    let ca = binom_row(active, p);
    let cp = binom_row(passive, p);
    let mut ways = [0u64; 128];
    for ax in (0..=active).step_by(2) {
        let a = ca[ax];
        if a == 0 {
            continue;
        }
        for py in 0..=passive {
            let s = ax + py;
            ways[s] += a * cp[py] % p;
            if ways[s] >= p {
                ways[s] -= p;
            }
        }
    }
    ways
}

/// Number of (active, passive) gap vectors with arithmetic sum `total` and active xor 0.
fn count_xor_sum(total: u64, ways: &[u64], n_ways: usize, p: u64) -> u64 {
    const CAP: usize = 160;
    let mut even = [(0u16, 0u64); 64];
    let mut odd = [(0u16, 0u64); 64];
    let mut n_even = 0usize;
    let mut n_odd = 0usize;
    for ones in 0..n_ways {
        let cnt = ways[ones];
        if cnt == 0 {
            continue;
        }
        if ones & 1 == 0 {
            even[n_even] = (ones as u16, cnt);
            n_even += 1;
        } else {
            odd[n_odd] = (ones as u16, cnt);
            n_odd += 1;
        }
    }

    let mut dp = [0u64; CAP];
    let mut ndp = [0u64; CAP];
    dp[0] = 1;
    let mut len = 1usize;

    let bits = 64 - total.leading_zeros();
    for b in 0..bits {
        ndp.fill(0);
        let bit = ((total >> b) & 1) as usize;
        let mut new_len = 0usize;
        for carry in 0..len {
            let v = dp[carry];
            if v == 0 {
                continue;
            }
            let (terms, nt) = if ((carry ^ bit) & 1) == 0 {
                (&even, n_even)
            } else {
                (&odd, n_odd)
            };
            for i in 0..nt {
                let ones = terms[i].0 as usize;
                let count = terms[i].1;
                // carry + ones <= ~200, bit in {0,1}; next carry fits CAP
                let nc = (carry + ones - bit) >> 1;
                ndp[nc] = ndp[nc].wrapping_add(v.wrapping_mul(count));
                if nc >= new_len {
                    new_len = nc + 1;
                }
            }
        }
        for x in ndp.iter_mut().take(new_len) {
            *x %= p;
        }
        std::mem::swap(&mut dp, &mut ndp);
        len = new_len;
        if len == 0 {
            return 0;
        }
    }
    dp[0]
}

fn losing(n: u64, c: u64, p: u64) -> u64 {
    let coins = c + 1;
    let empty = n - coins;
    let active = ((coins + 1) / 2) as usize;
    let passive = coins as usize - active + 1;
    let ways = build_ways(active, passive, p);
    let ways_m = build_ways(active - 1, passive, p);
    let n_ways = active + passive + 1;
    let n_ways_m = active - 1 + passive + 1;
    let l0 = count_xor_sum(empty, &ways, n_ways, p);
    let l1a = count_xor_sum(empty + 1, &ways, n_ways, p);
    let l1b = count_xor_sum(empty + 1, &ways_m, n_ways_m, p);
    let l1 = (l1a + p - l1b) % p;
    (l0 + (coins - 2) % p * l1) % p
}

fn w(n: u64, c: u64, p: u64) -> u64 {
    let coins = c + 1;
    let total = coins % p * binom_mod(n, coins, p) % p;
    (total + p - losing(n, c, p)) % p
}

fn main() {
    let n = 1_000_000u64;
    let c = 100u64;
    let m1 = 1_000_003u64;
    let m2 = 1_000_033u64;

    let w1 = w(n, c, m1);
    let w2 = w(n, c, m2);

    let m = m1 * m2;
    let t1 = (w1 as u128) * (m2 as u128) % (m as u128)
        * (mod_pow(m2, m1 - 2, m1) as u128)
        % (m as u128);
    let t2 = (w2 as u128) * (m1 as u128) % (m as u128)
        * (mod_pow(m1, m2 - 2, m2) as u128)
        % (m as u128);
    let x = (t1 + t2) % (m as u128);
    println!("{}", x);
}
