// Project Euler 176 - Right-angled triangles sharing a cathetus
// Find smallest n with exactly 47547 right triangles sharing a cathetus.

fn factorize(mut n: u64) -> Vec<(u64, u32)> {
    let mut factors = Vec::new();
    let mut d = 2u64;
    while d * d <= n {
        if n % d == 0 {
            let mut e = 0u32;
            while n % d == 0 {
                n /= d;
                e += 1;
            }
            factors.push((d, e));
        }
        d += 1;
    }
    if n > 1 {
        factors.push((n, 1));
    }
    factors
}

fn get_divisors(n: u64) -> Vec<u64> {
    let factors = factorize(n);
    let mut divs = vec![1u64];
    for (p, e) in factors {
        let prev_len = divs.len();
        let mut pk = 1u64;
        for _ in 1..=e {
            pk *= p;
            for j in 0..prev_len {
                divs.push(divs[j] * pk);
            }
        }
    }
    divs
}

fn minimal_m_for(f: u64) -> u64 {
    let odd_primes = [3u64, 5, 7, 11, 13, 17, 19, 23, 29, 31];
    let factors = factorize(f);

    let mut exponents = Vec::new();
    for (p, e) in &factors {
        for _ in 0..*e {
            exponents.push((p - 1) / 2);
        }
    }
    exponents.sort_unstable_by(|a, b| b.cmp(a));

    let mut m = 1u64;
    for (i, &exp) in exponents.iter().enumerate() {
        let p = odd_primes[i];
        for _ in 0..exp {
            m *= p;
        }
    }
    m
}

fn main() {
    const VALUE: u64 = 95095; // 2 * 47547 + 1

    let mut best = u64::MAX;

    // Case a = 0 or a = 1
    for a in 0..=1u32 {
        let m = minimal_m_for(VALUE);
        let n = (1u64 << a) * m;
        if n < best {
            best = n;
        }
    }

    // Case a >= 2: (2a-1) * prod = VALUE
    let divs = get_divisors(VALUE);
    for &d in &divs {
        if d < 3 {
            continue;
        }
        if (d + 1) % 2 != 0 {
            continue;
        }
        let a = (d + 1) / 2;
        if a < 2 || a >= 64 {
            continue;
        }
        let f = VALUE / d;
        let m = minimal_m_for(f);
        let power = 1u64 << a;
        if m > 0 && power > u64::MAX / m {
            continue;
        }
        let n = power * m;
        if n < best {
            best = n;
        }
    }

    println!("{}", best);
}
