// Problem 996: Overtakes
// Generating function DP for F(123, 4567891) mod 1234567891.

use euler_utils::{mod_inv, mod_mul};

const MOD: u64 = 1_234_567_891;

fn trim(poly: &mut Vec<u64>) {
    while poly.len() > 1 && *poly.last().unwrap() == 0 {
        poly.pop();
    }
}

fn add_to(dst: &mut Vec<u64>, src: &[u64], modu: u64) {
    if dst.len() < src.len() {
        dst.resize(src.len(), 0);
    }
    for (i, &value) in src.iter().enumerate() {
        dst[i] = (dst[i] + value) % modu;
    }
}

fn mul_one_minus_q(poly: &[u64], modu: u64) -> Vec<u64> {
    let mut out = vec![0u64; poly.len() + 1];
    for (i, &value) in poly.iter().enumerate() {
        out[i] = (out[i] + value) % modu;
        out[i + 1] = (out[i + 1] + modu - value % modu) % modu;
    }
    trim(&mut out);
    out
}

fn mul_poly(a: &[u64], b: &[u64], max_degree: usize, modu: u64) -> Vec<u64> {
    if a.is_empty() || b.is_empty() {
        return vec![0];
    }
    let out_len = (a.len() + b.len() - 2).min(max_degree) + 1;
    let mut out = vec![0u64; out_len];
    for (i, &ai) in a.iter().enumerate() {
        if ai == 0 {
            continue;
        }
        let last_j = (b.len() - 1).min(max_degree.saturating_sub(i));
        for j in 0..=last_j {
            let bj = b[j];
            if bj != 0 {
                out[i + j] = (out[i + j] + mod_mul(ai, bj, modu)) % modu;
            }
        }
    }
    trim(&mut out);
    out
}

/// C(n, k) mod prime via product with modular inverse (k small or reduced).
fn comb_mod(n: u64, k: u64, modu: u64) -> u64 {
    if k > n {
        return 0;
    }
    let k = k.min(n - k);
    let mut res = 1u64;
    for i in 0..k {
        res = mod_mul(res, (n - i) % modu, modu);
        res = mod_mul(res, mod_inv(i + 1, modu).unwrap(), modu);
    }
    res
}

/// C(n, k) as i128 for small test cases without modulus.
fn comb_i128(n: i128, k: i128) -> i128 {
    if k < 0 || k > n {
        return 0;
    }
    let k = k.min(n - k);
    let mut res = 1i128;
    for i in 0..k {
        res = res * (n - i) / (i + 1);
    }
    res
}

fn block_count_mod(length: u64, cost: u64, modu: u64) -> u64 {
    if cost == 0 || 2 * cost < length {
        return 0;
    }
    let total = comb_mod(2 * cost - 1, length - 1, modu);
    let too_large = if cost < length {
        0
    } else {
        comb_mod(cost - 1, length - 1, modu)
    };
    (total + modu - mod_mul(length % modu, too_large, modu)) % modu
}

fn block_numerator(length: usize, modu: u64) -> Vec<u64> {
    let mut coeffs = vec![0u64; length + 1];
    let len = length as u64;
    for j in 0..=length {
        let mut value = 0i64;
        let m = modu as i64;
        for i in 0..=j {
            let sign = if i % 2 == 0 { 1i64 } else { -1i64 };
            let term = mod_mul(
                comb_mod(len, i as u64, modu),
                block_count_mod(len, (j - i) as u64, modu),
                modu,
            ) as i64;
            value = (value + sign * term).rem_euclid(m);
        }
        coeffs[j] = value as u64;
    }
    trim(&mut coeffs);
    coeffs
}

fn numerator_for_all_valid_vectors(n: usize, modu: u64) -> Vec<u64> {
    let mut block_num: Vec<Option<Vec<u64>>> = vec![None; n + 1];
    for length in 2..=n {
        block_num[length] = Some(block_numerator(length, modu));
    }

    let mut total: Vec<Vec<u64>> = vec![Vec::new(); n + 1];
    let mut zero_end: Vec<Vec<u64>> = vec![Vec::new(); n + 1];
    total[0] = vec![1];
    zero_end[0] = vec![1];

    for pos in 0..=n {
        if pos < n && !total[pos].is_empty() {
            let add_zero = mul_one_minus_q(&total[pos], modu);
            add_to(&mut total[pos + 1], &add_zero, modu);
            add_to(&mut zero_end[pos + 1], &add_zero, modu);
        }

        if !zero_end[pos].is_empty() {
            for length in 2..=(n - pos) {
                let product = mul_poly(
                    &zero_end[pos],
                    block_num[length].as_ref().unwrap(),
                    pos + length,
                    modu,
                );
                add_to(&mut total[pos + length], &product, modu);
            }
        }
    }

    std::mem::take(&mut total[n])
}

fn count_tuples_mod(n: usize, k: u64, modu: u64) -> u64 {
    let max_cost = k / 2;
    let numerator = numerator_for_all_valid_vectors(n, modu);

    let mut answer = 0u64;
    for (degree, &coeff) in numerator.iter().enumerate() {
        if coeff == 0 || degree as u64 > max_cost {
            continue;
        }
        let top = max_cost - degree as u64 + n as u64;
        let ways = comb_mod(top, n as u64, modu);
        answer = (answer + mod_mul(coeff, ways, modu)) % modu;
    }
    answer
}

/// Unmodular version for small verification tests only.
fn count_tuples_plain(n: usize, k: u64) -> i128 {
    // Reuse modular construction is hard for exact; recompute with i128 for small n.
    fn block_count(length: i128, cost: i128) -> i128 {
        if cost <= 0 || 2 * cost < length {
            return 0;
        }
        comb_i128(2 * cost - 1, length - 1) - length * if cost < length {
            0
        } else {
            comb_i128(cost - 1, length - 1)
        }
    }

    fn block_num(length: usize) -> Vec<i128> {
        let mut coeffs = vec![0i128; length + 1];
        let len = length as i128;
        for j in 0..=length {
            let mut value = 0i128;
            for i in 0..=j {
                let sign = if i % 2 == 0 { 1i128 } else { -1i128 };
                value += sign * comb_i128(len, i as i128) * block_count(len, (j - i) as i128);
            }
            coeffs[j] = value;
        }
        while coeffs.len() > 1 && *coeffs.last().unwrap() == 0 {
            coeffs.pop();
        }
        coeffs
    }

    let mut block_nums: Vec<Option<Vec<i128>>> = vec![None; n + 1];
    for length in 2..=n {
        block_nums[length] = Some(block_num(length));
    }

    let mut total: Vec<Vec<i128>> = vec![Vec::new(); n + 1];
    let mut zero_end: Vec<Vec<i128>> = vec![Vec::new(); n + 1];
    total[0] = vec![1];
    zero_end[0] = vec![1];

    let mul_omq = |poly: &[i128]| -> Vec<i128> {
        let mut out = vec![0i128; poly.len() + 1];
        for (i, &v) in poly.iter().enumerate() {
            out[i] += v;
            out[i + 1] -= v;
        }
        while out.len() > 1 && *out.last().unwrap() == 0 {
            out.pop();
        }
        out
    };

    let mul_p = |a: &[i128], b: &[i128], max_degree: usize| -> Vec<i128> {
        let out_len = (a.len() + b.len() - 2).min(max_degree) + 1;
        let mut out = vec![0i128; out_len];
        for (i, &ai) in a.iter().enumerate() {
            if ai == 0 {
                continue;
            }
            let last_j = (b.len() - 1).min(max_degree.saturating_sub(i));
            for j in 0..=last_j {
                out[i + j] += ai * b[j];
            }
        }
        while out.len() > 1 && *out.last().unwrap() == 0 {
            out.pop();
        }
        out
    };

    let add_to_i = |dst: &mut Vec<i128>, src: &[i128]| {
        if dst.len() < src.len() {
            dst.resize(src.len(), 0);
        }
        for (i, &v) in src.iter().enumerate() {
            dst[i] += v;
        }
    };

    for pos in 0..=n {
        if pos < n && !total[pos].is_empty() {
            let add_zero = mul_omq(&total[pos]);
            add_to_i(&mut total[pos + 1], &add_zero);
            add_to_i(&mut zero_end[pos + 1], &add_zero);
        }
        if !zero_end[pos].is_empty() {
            for length in 2..=(n - pos) {
                let product = mul_p(
                    &zero_end[pos],
                    block_nums[length].as_ref().unwrap(),
                    pos + length,
                );
                add_to_i(&mut total[pos + length], &product);
            }
        }
    }

    let max_cost = k / 2;
    let mut answer = 0i128;
    for (degree, &coeff) in total[n].iter().enumerate() {
        if coeff == 0 || degree as u64 > max_cost {
            continue;
        }
        let top = max_cost - degree as u64 + n as u64;
        answer += coeff * comb_i128(top as i128, n as i128);
    }
    answer
}

fn main() {
    debug_assert_eq!(count_tuples_plain(3, 4), 8);
    debug_assert_eq!(count_tuples_plain(12, 34), 2_457_178_250);

    println!("{}", count_tuples_mod(123, 4_567_891, MOD));
}
