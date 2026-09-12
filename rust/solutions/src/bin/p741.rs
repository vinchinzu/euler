// Project Euler 741 - Binary Grid Colourings
//
// Burnside's lemma with rotation/reflection symmetries on grid colourings.
// g(n) is a sum of 5 independent recurrences; N2 dominates, so those run in parallel.

const M: u64 = 1_000_000_007;
const N1: i32 = 823543; // 7^7
const N2: i32 = 16777216; // 8^8

#[inline]
fn mul(a: u64, b: u64) -> u64 {
    a * b % M
}

fn pow_mod(mut base: u64, mut exp: u64) -> u64 {
    let mut result: u64 = 1;
    base %= M;
    while exp > 0 {
        if exp & 1 == 1 {
            result = mul(result, base);
        }
        exp >>= 1;
        base = mul(base, base);
    }
    result
}

fn mod_inv(a: u64) -> u64 {
    pow_mod(a, M - 2)
}

fn f(n: i32) -> u64 {
    let n = n as usize;
    let mut f_arr = vec![0u64; n + 1];
    let mut fp = vec![0u64; n + 1];
    f_arr[0] = 1;
    for k in 2..=n {
        let ku = k as u64;
        fp[k] = mul(ku, ku - 1);
        fp[k] = mul(fp[k], (mul(ku - 1, f_arr[k - 2]) + fp[k - 1]) % M);
        f_arr[k] = mul((M + 1) / 2, fp[k]);
    }
    f_arr[n]
}

fn rotate90(n: i32) -> u64 {
    if n % 2 == 1 {
        return 0;
    }
    let n = n as usize;
    let mut f_arr = vec![0u64; n + 1];
    let mut fp = vec![0u64; n + 1];
    f_arr[0] = 1;
    if n >= 2 {
        f_arr[2] = 1;
    }
    let mut k = 4;
    while k <= n {
        let ku = k as u64;
        fp[k] = mul(ku - 2, (f_arr[k - 4] + fp[k - 2]) % M);
        f_arr[k] = (f_arr[k - 2] + mul(ku / 2 - 1, f_arr[k - 4]) + mul(ku - 2, fp[k - 2])) % M;
        k += 2;
    }
    f_arr[n]
}

fn rotate180(n: i32) -> u64 {
    let n = n as usize;
    let mut f_arr = vec![0u64; n + 1];
    let mut fp = vec![0u64; n + 1];
    f_arr[0] = 1;
    if n >= 2 {
        f_arr[2] = 1;
        fp[2] = 2;
    }
    for k in 3..=n {
        let ku = k as u64;
        if k % 2 == 0 {
            let inner = (mul(ku - 2, f_arr[k - 4]) + fp[k - 2]) % M;
            fp[k] = mul(ku, (mul(inner, ku - 2) + f_arr[k - 2]) % M);
            f_arr[k] = mul((M + 1) / 2, fp[k]);
        } else {
            fp[k] = mul(ku - 1, (f_arr[k - 3] + mul(ku - 3, fp[k - 2])) % M);
            f_arr[k] = mul(ku / 2, fp[k]);
        }
    }
    f_arr[n]
}

fn flip_y(n: i32) -> u64 {
    if n % 2 == 1 {
        return 0;
    }
    let n = n as usize;
    let mut fact: u64 = 1;
    for i in 1..=n {
        fact = mul(fact, i as u64);
    }
    mul(fact, mod_inv(pow_mod(2, n as u64 / 2)))
}

fn flip_diagonal(n: i32) -> u64 {
    let n = n as usize;
    let mut f_arr = vec![0u64; n + 1];
    let mut fp = vec![0u64; n + 1];
    let mut fpp = vec![0u64; n + 1];
    f_arr[0] = 1;
    if n >= 1 {
        fp[1] = 1;
    }
    for k in 2..=n {
        let ku = k as u64;
        fp[k] = (f_arr[k - 1] + mul(ku - 1, fp[k - 1])) % M;
        fpp[k] = (f_arr[k - 2] + fp[k - 1] + mul(ku - 2, fpp[k - 1])) % M;
        let ncr = (ku - 1) * (ku - 2) / 2 % M;
        f_arr[k] = (mul(ku - 1, fp[k - 1]) + mul(ncr, fpp[k - 1])) % M;
    }
    f_arr[n]
}

fn g_parts(n: i32) -> u64 {
    // Four heavy recurrences in parallel; flip_y is a cheap factorial.
    let ((f_n, r90), (r180, fd)) = rayon::join(
        || rayon::join(|| f(n), || rotate90(n)),
        || rayon::join(|| rotate180(n), || flip_diagonal(n)),
    );
    let fy = flip_y(n);
    let val = (f_n + 2 * r90 % M + r180 + 2 * fy % M + 2 * fd % M) % M;
    mul(val, mod_inv(8))
}

fn main() {
    // N2 (~16.8M) dominates; N1 is ~20× smaller and overlaps the N2 recurrences.
    let (a, b) = rayon::join(|| g_parts(N1), || g_parts(N2));
    println!("{}", (a + b) % M);
}
