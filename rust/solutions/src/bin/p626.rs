// Project Euler 626 - Counting Binary Matrices
// Burnside's lemma over partition pairs for N=20

const N: usize = 20;
const M: i64 = 1_001_001_011;

fn powmod(mut base: i64, mut exp: i64, m: i64) -> i64 {
    let mut result = 1i64;
    base = ((base % m) + m) % m;
    while exp > 0 {
        if exp & 1 == 1 { result = (result as i128 * base as i128 % m as i128) as i64; }
        base = (base as i128 * base as i128 % m as i128) as i64;
        exp >>= 1;
    }
    result
}

fn modinv(a: i64) -> i64 { powmod(a, M - 2, M) }
fn gcd(mut a: i64, mut b: i64) -> i64 { while b != 0 { let t = b; b = a % b; a = t; } a }
fn factorial_mod(n: usize) -> i64 { (2..=n as i64).fold(1i64, |a, b| (a as i128 * b as i128 % M as i128) as i64) }

#[derive(Clone)]
struct Part { sizes: Vec<i64>, counts: Vec<i64> }

fn gen_partitions(n: usize) -> Vec<Part> {
    let mut result = Vec::new();
    let mut stack: Vec<(usize, usize, Vec<i64>)> = vec![(n, n, vec![])];
    while let Some((rem, max_val, parts)) = stack.pop() {
        if rem == 0 {
            let mut p = Part { sizes: Vec::new(), counts: Vec::new() };
            let mut i = 0;
            while i < parts.len() {
                let v = parts[i];
                let mut c = 0;
                while i < parts.len() && parts[i] == v { c += 1; i += 1; }
                p.sizes.push(v as i64);
                p.counts.push(c);
            }
            result.push(p);
            continue;
        }
        let top = if max_val < rem { max_val } else { rem };
        for i in (1..=top).rev() {
            let mut new_parts = parts.clone();
            new_parts.push(i as i64);
            stack.push((rem - i, i, new_parts));
        }
    }
    result
}

fn num_arrangements(p: &Part) -> i64 {
    let mut result = factorial_mod(N);
    for i in 0..p.sizes.len() {
        for _ in 0..p.counts[i] {
            result = (result as i128 * modinv(p.sizes[i]) as i128 % M as i128) as i64;
        }
        result = (result as i128 * modinv(factorial_mod(p.counts[i] as usize)) as i128 % M as i128) as i64;
    }
    result
}

fn num_restricted_rows(perm: &Part, other: &Part) -> i64 {
    let mut count = 0i64;
    for i in 0..perm.sizes.len() {
        let size = perm.sizes[i];
        let mut found = false;
        for j in 0..other.sizes.len() {
            if (other.sizes[j] / gcd(size, other.sizes[j])) % 2 == 1 { found = true; break; }
        }
        if found { count += perm.counts[i]; }
    }
    count
}

fn total_parts(p: &Part) -> i64 { p.counts.iter().sum() }

fn main() {
    let partitions = gen_partitions(N);
    let mut ans = 0i64;

    for p1 in &partitions {
        for p2 in &partitions {
            let mut num_grid_cycles = 0i64;
            for a in 0..p1.sizes.len() {
                for b in 0..p2.sizes.len() {
                    num_grid_cycles += gcd(p1.sizes[a], p2.sizes[b]) * p1.counts[a] * p2.counts[b];
                }
            }
            let nr1 = num_restricted_rows(p1, p2);
            let nr2 = num_restricted_rows(p2, p1);
            let tp1 = total_parts(p1);
            let tp2 = total_parts(p2);
            let all_restricted = nr1 == tp1 && nr2 == tp2;

            let mut term = (num_arrangements(p1) as i128 * num_arrangements(p2) as i128 % M as i128) as i64;
            term = (term as i128 * powmod(2, num_grid_cycles, M) as i128 % M as i128) as i64;
            let exp2 = 2 * N as i64 - nr1 - nr2 - if all_restricted { 0 } else { 1 };
            term = (term as i128 * powmod(2, exp2, M) as i128 % M as i128) as i64;
            ans = (ans + term) % M;
        }
    }

    let inv_fact_n = modinv(factorial_mod(N));
    let inv_2_pow = modinv(powmod(2, 2 * N as i64 - 1, M));
    ans = (ans as i128 * inv_fact_n as i128 % M as i128) as i64;
    ans = (ans as i128 * inv_fact_n as i128 % M as i128) as i64;
    ans = (ans as i128 * inv_2_pow as i128 % M as i128) as i64;

    println!("{}", ans);
}
