// Project Euler 311 - Biclinic Integral Quadrilaterals
// Sieve primes 1 mod 4 and 3 mod 4, enumerate products.

const SIEVE1_LIMIT: usize = 100_000_000;
const SIEVE3_LIMIT: usize = 2775;

static mut L: i64 = 0;
static mut L2: i64 = 0;
static mut ANS: i64 = 0;

fn main() {
    let big_n: i64 = 10_000_000_000;
    unsafe {
        L = big_n / 4;
        L2 = L / (5 * 5 * 13);
    }
    let l = unsafe { L };
    let l2 = unsafe { L2 } as usize;

    // Bit sieve
    let bytes = (SIEVE1_LIMIT >> 3) + 1;
    let mut sieve_bits = vec![0xFFu8; bytes];
    let clear = |bits: &mut Vec<u8>, i: usize| { bits[i >> 3] &= !(1 << (i & 7)); };
    let get = |bits: &[u8], i: usize| -> bool { (bits[i >> 3] >> (i & 7)) & 1 == 1 };
    clear(&mut sieve_bits, 0);
    clear(&mut sieve_bits, 1);

    let sq = (SIEVE1_LIMIT as f64).sqrt() as usize;
    for i in 2..=sq {
        if get(&sieve_bits, i) {
            let mut j = i * i;
            while j <= SIEVE1_LIMIT {
                clear(&mut sieve_bits, j);
                j += i;
            }
        }
    }

    let mut primes1mod4 = Vec::new();
    let mut i = 5;
    while i <= SIEVE1_LIMIT {
        if get(&sieve_bits, i) { primes1mod4.push(i as i64); }
        i += 4;
    }

    let mut primes3mod4 = Vec::new();
    let mut i = 3;
    while i <= SIEVE3_LIMIT {
        if i <= SIEVE1_LIMIT && get(&sieve_bits, i) { primes3mod4.push(i as i64); }
        i += 4;
    }

    drop(sieve_bits);

    let mut num3mod4_prods = vec![0i32; l2 + 1];
    let mut cum_c = vec![0i32; l2 + 1];

    fn helper1(min_idx: usize, n: i64, primes3mod4: &[i64], num3: &mut [i32], l2: i64) {
        num3[n as usize] += 1;
        for idx in min_idx..primes3mod4.len() {
            let p = primes3mod4[idx];
            let p2 = p * p;
            if n * p2 > l2 { return; }
            let mut new_n = n;
            while new_n * p2 <= l2 {
                new_n *= p2;
                helper1(idx + 1, new_n, primes3mod4, num3, l2);
            }
        }
    }

    helper1(0, 1, &primes3mod4, &mut num3mod4_prods, unsafe { L2 });

    cum_c[0] = num3mod4_prods[0];
    for i in 1..=l2 {
        cum_c[i] = cum_c[i - 1] + num3mod4_prods[i];
    }
    drop(num3mod4_prods);

    fn helper2(
        min_idx: usize, n: i64, a0: i32, b: i32,
        primes1: &[i64], cum_c: &[i32], l: i64, l2: usize, ans: &mut i64,
    ) {
        if b >= 5 {
            let r2_contrib = (b + if a0 % 2 == 1 { 1 } else { 0 }) / 2;
            if r2_contrib >= 3 {
                let ways = r2_contrib as i64 * (r2_contrib as i64 - 1) * (r2_contrib as i64 - 2) / 6;
                let idx = (l / n) as usize;
                if idx <= l2 {
                    *ans += cum_c[idx] as i64 * ways;
                }
            }
        }

        if n > l { return; }

        let dlimit = l as f64 / n as f64;
        let ilimit = if b == 1 {
            dlimit.cbrt() as i64
        } else if b == 2 {
            dlimit.sqrt() as i64
        } else {
            dlimit as i64
        };

        for idx in min_idx..primes1.len() {
            let p = primes1[idx];
            if p > ilimit { return; }

            let mut e = 1;
            let mut new_n = n;
            while new_n <= l / p {
                new_n *= p;
                helper2(idx + 1, new_n, a0, b * (e + 1), primes1, cum_c, l, l2, ans);
                e += 1;
            }
        }
    }

    let mut ans: i64 = 0;
    let mut a0 = 0i32;
    let mut prod = 1i64;
    while prod <= l {
        helper2(0, prod, a0, 1, &primes1mod4, &cum_c, l, l2, &mut ans);
        a0 += 1;
        prod *= 2;
    }

    println!("{}", ans);
}
