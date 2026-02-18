// Project Euler 963 - Base-3 combinatorics
// F(N) computes a value based on base-3 representations of numbers 1..N,
// building tuples from digit patterns, counting pair combinations, then
// summing squared counts divided by 4.

use std::collections::HashMap;

fn compute_f(n: usize) -> u128 {
    // Compute m = smallest power of 2 >= ceil(log_3(max(N,1)))
    // Actually: m = 1 << ceil(log2(max(N,1)) / log2(3)) ... let me re-read the Python.
    // m = 1 << ceil(log(max(N,1), 3))
    // That's 2^ceil(log_3(N))
    let mut m_exp = 0u32;
    {
        let mut val = 1usize;
        while val < n.max(1) {
            val *= 3;
            m_exp += 1;
        }
    }
    let m = 1usize << m_exp;

    // Build B array
    let mut b = vec![0.0f64; m + 1];
    for i in 1..=m {
        let h = b[i >> 1];
        if i & 1 == 1 {
            b[i] = h + 1.0;
        } else if h < 2.0 {
            b[i] = h / 2.0;
        } else {
            b[i] = h - 1.0;
        }
    }

    // For each n in 1..=N, compute R[n] = (tuple_of_L, parity_of_2count, B_value)
    // We need to represent the tuple as something hashable.
    // L is a tuple of integers, parity is 0 or 1, B_value is f64.
    // Since B_value comes from the B array with index from binary string of digits < 2,
    // we can represent it as the integer index into B instead, to avoid floating point hashing.

    // Actually, let's use the B_value as an f64 but we need to hash it.
    // The Python uses Counter with float keys. Let's discretize: since B values are rational
    // with denominators being powers of 2, we can represent them as (numerator, denominator_exp).
    // But it's simpler to just use the integer index into B[] as the key, since two different
    // indices could give the same B value... Actually the Python code does B[int(u, 2)].
    // Let's just use the integer u_val directly as the key (the binary interpretation).
    // Two different u_val could map to same B value, but Python uses B[int(u,2)] as part of
    // the tuple, so it uses the B *value*, not the index.

    // Since we need exact arithmetic for the final answer, and B values are rational with
    // power-of-2 denominators, let's represent B as rational: numerator * 2^(-shift).
    // Actually, let's look at B more carefully:
    // B[0] = 0, B[1] = 0+1=1, B[2] = B[1]/2 = 0.5, B[3] = B[1]+1 = 2,
    // B[4] = B[2]/2 = 0.25, B[5] = B[2]+1 = 1.5, B[6] = B[3]-1 = 1, B[7] = B[3]+1 = 3
    // These are all multiples of 2^(-k) for some k.
    // For m up to about 2^17 = 131072 (since N=10^5, log_3(10^5) ~ 10.5, ceil = 11, m=2048),
    // B values fit nicely in f64 with exact representation.
    // m = 2^11 = 2048, so B has 2049 entries. All values are dyadic rationals with
    // denominator at most 2^11, which is exactly representable in f64.

    // For hashing f64, we'll convert to bits representation (since all values are exact).
    // Or better: represent as i64 * 2^(-precision). With precision = m_exp (11 bits),
    // multiply B by 2^m_exp to get integers.

    // Actually let's just use i64 representation. Multiply all B values by 2^m_exp.
    let scale = 1i64 << m_exp;
    let mut b_int = vec![0i64; m + 1];
    for i in 1..=m {
        let h = b_int[i >> 1];
        if i & 1 == 1 {
            b_int[i] = h + scale;
        } else if h < 2 * scale {
            b_int[i] = h / 2;
        } else {
            b_int[i] = h - scale;
        }
    }
    // Verify: b_int[i] = b[i] * scale
    // b_int[1] = 0 + scale = scale (b[1]=1, correct)
    // b_int[2] = b_int[1]/2 = scale/2 (b[2]=0.5, correct since 0.5*scale = scale/2)
    // b_int[3] = scale + scale = 2*scale... wait, h=b_int[1]=scale, i&1==1, so b_int[3] = scale + scale = 2*scale
    // b[3] = b[1]+1 = 2, 2*scale, correct.
    // b_int[6]: i=6, i>>1=3, h=b_int[3]=2*scale, i&1==0, h >= 2*scale, so b_int[6] = 2*scale - scale = scale
    // b[6] = b[3]-1 = 1, scale, correct.
    // Looks good.

    // Now compute R for each n in 1..=N
    type Key = (Vec<u32>, u8, i64); // (L_sorted, parity, B_scaled)

    let mut counter: HashMap<Key, u128> = HashMap::new();

    for n_val in 1..=n {
        // Get base-3 digits (most significant first)
        let mut digits = Vec::new();
        let mut x = n_val;
        while x > 0 {
            digits.push((x % 3) as u8);
            x /= 3;
        }
        digits.reverse();
        // digits is the base-3 representation, most significant first

        // Find l: last index j before the first '1' digit where digit < 1 (i.e., digit == 0)
        let mut l: i32 = -1;
        for (j, &c) in digits.iter().enumerate() {
            if c == 1 {
                break;
            }
            if c < 1 {
                l = j as i32;
            }
        }

        // Build L: process digits s[:l] in reverse, tracking runs of zeros
        let mut big_l: Vec<u32> = Vec::new();
        if l > -1 {
            let l_idx = l as usize;
            let mut k: u32 = 0;
            for &c in digits[..l_idx].iter().rev() {
                if c < 1 {
                    k += 1;
                } else {
                    big_l.push(k);
                }
            }
        }

        // u_str: keep only digits < 2 (i.e., 0 and 1), interpret as binary
        let mut u_val: usize = 0;
        for &c in &digits {
            if c < 2 {
                u_val = u_val * 2 + c as usize;
            }
        }

        // Count of '2' digits, parity
        let count_2 = digits.iter().filter(|&&c| c == 2).count();
        let parity = (count_2 & 1) as u8;

        // B value (scaled)
        let b_val = if u_val == 0 && digits.iter().all(|&c| c >= 2) {
            // u string was empty or "0" -> the Python does: B[int(u, 2)] if u else 0.0
            // If u_str is empty, b_val = 0
            // But u_val would be 0 if u_str is "0" too. We need to check if u_str is empty.
            // u_str is empty only if all digits are >= 2, which can't happen since base-3
            // digits include 0 and 1 for most numbers. Actually for n_val whose base-3
            // representation is all 2s (like 2, 8, 26, ...), u_str is empty.
            // Let's check: the Python does u = "".join(c for c in s if c < "2")
            // If all chars are "2", u = "", and then B[int(u,2)] if u else 0.0 -> 0.0
            let has_non_two = digits.iter().any(|&c| c < 2);
            if has_non_two {
                b_int[0] // u_val is 0, but there are 0-digits -> u_str = "0...0"
            } else {
                0i64 // u_str is empty
            }
        } else {
            b_int[u_val]
        };

        let key = (big_l, parity, b_val);
        *counter.entry(key).or_insert(0) += 1;
    }

    // Build d counter from pairs
    let entries: Vec<(&Key, &u128)> = counter.iter().collect();
    let mut d: HashMap<(Vec<u32>, u8, i64), u128> = HashMap::new();

    for &(a_key, &ca) in &entries {
        for &(b_key, &cb) in &entries {
            // Merge the L tuples (sorted)
            let mut merged = a_key.0.clone();
            merged.extend_from_slice(&b_key.0);
            merged.sort();

            let parity = a_key.1 ^ b_key.1;
            let b_sum = a_key.2 + b_key.2;

            let combined_key = (merged, parity, b_sum);

            let count = if a_key == b_key {
                ca * (cb + 1)
            } else {
                ca * cb
            };

            *d.entry(combined_key).or_insert(0) += count;
        }
    }

    // Sum v^2 for all values, divide by 4
    let mut result: u128 = 0;
    for &v in d.values() {
        result += v * v;
    }
    result / 4
}

fn main() {
    println!("{}", compute_f(100_000));
}
