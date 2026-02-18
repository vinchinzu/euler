// Problem 925 – Larger Digit Permutation III
//
// B(n) = smallest number larger than n formed by rearranging digits of n, or 0.
// T(N) = sum_{n=1..N} B(n^2).  Compute T(10^16) mod 10^9+7.
//
// Algorithm: decompose T(N) = sum(n^2) + sum(delta) where delta = B(n^2) - n^2.
// Use suffix pruning: build n from lsd upward; when the last t digits of n^2
// already contain a non-trivial permutation pivot, delta is fixed for all
// extensions of n, so we can count completions analytically.

const MOD: u64 = 1_000_000_007;

/// In-place next lexicographic permutation (msd -> lsd order).
/// Returns true if a next permutation exists.
fn next_permutation(digs: &mut [u8]) -> bool {
    let n = digs.len();
    if n < 2 {
        return false;
    }
    let mut i = n - 2;
    loop {
        if digs[i] < digs[i + 1] {
            break;
        }
        if i == 0 {
            return false;
        }
        i -= 1;
    }
    let mut j = n - 1;
    while digs[j] <= digs[i] {
        j -= 1;
    }
    digs.swap(i, j);
    digs[i + 1..].reverse();
    true
}

/// Convert digit array (msd first) to u128.
fn digits_to_int(digs: &[u8]) -> u128 {
    let mut x: u128 = 0;
    for &d in digs {
        x = x * 10 + d as u128;
    }
    x
}

/// Compute B(n) exactly using u128 arithmetic.
fn b_func(n: u128) -> u128 {
    if n == 0 {
        return 0;
    }
    // Extract digits into stack-allocated buffer (msd first).
    // n^2 for n up to ~10^16 has at most 33 digits.
    let mut buf = [0u8; 40];
    let mut len = 0usize;
    let mut tmp = n;
    while tmp > 0 {
        buf[len] = (tmp % 10) as u8;
        len += 1;
        tmp /= 10;
    }
    buf[..len].reverse();
    if next_permutation(&mut buf[..len]) {
        digits_to_int(&buf[..len])
    } else {
        0
    }
}

/// Sum_{n=1..N} n^2 mod MOD, using the formula n(n+1)(2n+1)/6.
fn sum_squares_mod(big_n: u64) -> u64 {
    if big_n == 0 {
        return 0;
    }
    let m = MOD;
    let n = big_n % m;
    let n1 = (big_n + 1) % m;
    let n2 = (2 * (big_n % m) + 1) % m;
    // inv6 = 6^(MOD-2) mod MOD
    let inv6 = mod_pow(6, m - 2, m);
    let part = n % m * (n1 % m) % m;
    part % m * (n2 % m * inv6 % m) % m
}

fn mod_pow(mut base: u64, mut exp: u64, m: u64) -> u64 {
    let mut result = 1u64;
    base %= m;
    while exp > 0 {
        if exp & 1 == 1 {
            result = result * base % m;
        }
        base = base * base % m;
        exp >>= 1;
    }
    result
}

/// Given the last t digits in lsd->msd order and their numeric value s_val,
/// compute delta = next_permutation_value - s_val (as i128, since always positive
/// when called with a confirmed ascent in the suffix).
fn delta_from_suffix_digits_low(digits_low: &[u8], s_val: u128) -> i128 {
    // Reverse to msd order using a stack-allocated buffer.
    let t = digits_low.len();
    let mut msd = [0u8; 33];
    for k in 0..t {
        msd[k] = digits_low[t - 1 - k];
    }
    let ok = next_permutation(&mut msd[..t]);
    debug_assert!(ok);
    let new_s = digits_to_int(&msd[..t]);
    new_s as i128 - s_val as i128
}

/// Sum over all L-digit n: (B(n^2) - n^2) modulo MOD.
fn sum_delta_for_length(l: usize, pow10: &[u128], pow10_mod: &[u64]) -> u64 {
    let m = MOD as i128;
    let mut total: i128 = 0;

    // For L==1, n ranges 1..9 (single digit, no leading-zero issue).
    // For L>=2, the least significant digit can be 0..9.
    let start_digits: std::ops::Range<u8> = if l == 1 { 1..10 } else { 0..10 };

    // Stack: (c, v, tz)
    //   c  = number of digits of n determined so far (from lsd)
    //   v  = the suffix value (the last c digits of n as an integer)
    //   tz = trailing zero count of v
    let mut stack: Vec<(usize, u64, usize)> = Vec::new();
    for d0 in start_digits {
        let tz0 = if d0 == 0 { 1 } else { 0 };
        stack.push((1, d0 as u64, tz0));
    }

    while let Some((c, v, tz)) = stack.pop() {
        let t = c + tz;
        // s = (v*v) mod 10^t.  v is at most ~10^16, v*v at most ~10^32.
        let v128 = v as u128;
        let s = (v128 * v128) % pow10[t];

        // Extract t digits (lsd -> msd) and check if non-decreasing in that order.
        let mut digits_low = [0u8; 33];
        let mut tmp = s;
        let mut prev: i8 = -1;
        let mut nondecreasing = true;
        for k in 0..t {
            let d = (tmp % 10) as u8;
            tmp /= 10;
            if prev > d as i8 {
                nondecreasing = false;
            }
            prev = d as i8;
            digits_low[k] = d;
        }

        if !nondecreasing {
            let delta = delta_from_suffix_digits_low(&digits_low[..t], s);
            if c < l {
                // Remaining digits: l-c, most significant forced nonzero.
                let count_mod = (9u64 * pow10_mod[l - c - 1]) % MOD;
                total = (total + (delta % m) * (count_mod as i128)) % m;
            } else {
                total = (total + delta % m) % m;
            }
            continue;
        }

        if c == l {
            // Fully determined n; compute delta exactly.
            let x = v as u128 * v as u128;
            let bx = b_func(x);
            let delta_full = bx as i128 - x as i128;
            total = (total + delta_full % m) % m;
            continue;
        }

        let next_is_msd = c + 1 == l;
        let digit_start: u8 = if next_is_msd { 1 } else { 0 };

        // Push children in reverse digit order for depth-first traversal.
        for d in (digit_start..=9).rev() {
            let nv = v + (d as u64) * pow10[c] as u64;
            let ntz = if tz == c && d == 0 { c + 1 } else { tz };
            stack.push((c + 1, nv, ntz));
        }
    }

    ((total % m + m) % m) as u64
}

fn main() {
    let big_n: u64 = 10u64.pow(16) - 1;

    // Powers of 10 as u128 (for modulus 10^t in suffix checks; t up to 32).
    let mut pow10 = [0u128; 33];
    pow10[0] = 1;
    for i in 1..33 {
        pow10[i] = pow10[i - 1] * 10;
    }

    // Powers of 10 mod MOD (for counting completions).
    let mut pow10_mod = [0u64; 17];
    pow10_mod[0] = 1;
    for i in 1..17 {
        pow10_mod[i] = pow10_mod[i - 1] * 10 % MOD;
    }

    let mut sum_delta: u64 = 0;
    for l in 1..=16 {
        sum_delta = (sum_delta + sum_delta_for_length(l, &pow10, &pow10_mod)) % MOD;
    }

    let ans = (sum_squares_mod(big_n) + sum_delta) % MOD;
    println!("{}", ans);
}
