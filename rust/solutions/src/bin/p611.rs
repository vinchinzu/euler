// Project Euler 611 - Hallway of square steps
// Lucy DP for primes by residue mod 4 + DFS over prime powers

fn isqrt(n: i64) -> i64 {
    let mut x = (n as f64).sqrt() as i64;
    while x * x > n { x -= 1; }
    while (x + 1) * (x + 1) <= n { x += 1; }
    x
}

fn main() {
    let n_val: i64 = 1_000_000_000_000;
    let l = isqrt(n_val);

    // Sieve primes up to l
    let mut sieve = vec![false; (l + 1) as usize];
    sieve[0] = true;
    sieve[1] = true;
    let mut i = 2i64;
    while i * i <= l { if !sieve[i as usize] { let mut j = i*i; while j <= l { sieve[j as usize] = true; j += i; } } i += 1; }

    let sieve_primes: Vec<i64> = (3..=l).filter(|&i| !sieve[i as usize]).collect();

    let mut all_primes = vec![2i64];
    all_primes.extend_from_slice(&sieve_primes);

    let big_size = (n_val / l + 1) as usize;
    let mut big0 = vec![0i64; big_size];
    let mut big1 = vec![0i64; big_size];
    let mut small0 = vec![0i64; (l + 1) as usize];
    let mut small1 = vec![0i64; (l + 1) as usize];

    for i in 1..big_size {
        let v = n_val / i as i64;
        big0[i] = (v + 3) / 4;
        big1[i] = (v + 1) / 4;
    }
    for i in 1..=l as usize {
        small0[i] = (i as i64 + 3) / 4;
        small1[i] = (i as i64 + 1) / 4;
    }

    for &p in &sieve_primes {
        let p2 = p * p;
        let sp0 = small0[(p - 1) as usize];
        let sp1 = small1[(p - 1) as usize];
        let mod1 = p % 4 == 1;

        for i in 1..big_size {
            let v = n_val / i as i64;
            if v < p2 { break; }
            let ip = i as i64 * p;
            let (v0, v1) = if ip < big_size as i64 {
                (big0[ip as usize] - sp0, big1[ip as usize] - sp1)
            } else {
                (small0[(n_val / ip) as usize] - sp0, small1[(n_val / ip) as usize] - sp1)
            };
            if mod1 {
                big0[i] -= v0;
                big1[i] -= v1;
            } else {
                big0[i] -= v1;
                big1[i] -= v0;
            }
        }

        if p2 <= l {
            for i in (p2..=l).rev() {
                let v0 = small0[(i / p) as usize] - sp0;
                let v1 = small1[(i / p) as usize] - sp1;
                if mod1 {
                    small0[i as usize] -= v0;
                    small1[i as usize] -= v1;
                } else {
                    small0[i as usize] -= v1;
                    small1[i as usize] -= v0;
                }
            }
        }
    }

    // Remove count of 1
    for i in 1..big_size { big0[i] -= 1; }
    for i in 1..=l as usize { small0[i] -= 1; }

    let qv = |v: i64| -> i64 {
        if v <= l { small0[v as usize] } else { big0[(n_val / v) as usize] }
    };

    // DFS
    struct Frame { min_idx: usize, n: i64, p_val: i64, skip: bool }
    let mut stack = Vec::with_capacity(1 << 20);
    stack.push(Frame { min_idx: 0, n: 1, p_val: 1, skip: true });

    let mut ans: i64 = 0;

    while let Some(f) = stack.pop() {
        let p0 = all_primes[f.min_idx];
        if !f.skip && ((f.p_val + 1) / 2 - f.p_val) % 2 != 0 {
            ans += 1;
        }
        if n_val / f.n >= p0 && f.p_val % 2 != 0 {
            ans += qv(n_val / f.n) - small0[p0 as usize] + if p0 % 4 == 1 { 1 } else { 0 };
        }

        for idx in f.min_idx..all_primes.len() - 1 {
            let p = all_primes[idx];
            if f.n > n_val / p / p { break; }
            let step = if p % 4 == 3 { 2 } else { 1 };
            let mut e = step;
            let mut pe = 1i64;
            for _ in 0..step { pe *= p; }
            while f.n <= n_val / pe {
                let new_p = f.p_val * if p % 4 == 1 { e as i64 + 1 } else { 1 };
                let new_skip = p % 4 == 1 && e == 1;
                stack.push(Frame { min_idx: idx + 1, n: f.n * pe, p_val: new_p, skip: new_skip });
                e += step;
                let mut mul = 1i64;
                for _ in 0..step { mul *= p; }
                if pe > n_val / mul { break; }
                pe *= mul;
            }
        }
    }

    println!("{}", ans);
}
