// Problem 964 - Musical Chairs probability P(7)

use num::BigInt;
use num::BigRational;
use std::collections::HashMap;

/// Represent a partition as a Vec<u32> in descending order.
/// E.g., hook partition (n-m, 1^m) = [n-m, 1, 1, ..., 1]
type Partition = Vec<u32>;

fn hook_partition(n: u32, m: u32) -> Partition {
    if m == 0 {
        vec![n]
    } else {
        let mut p = Vec::with_capacity(1 + m as usize);
        p.push(n - m);
        for _ in 0..m {
            p.push(1);
        }
        p
    }
}

/// Count removal paths from `start` to `target` by successively removing corner boxes.
/// Uses memoization via the provided cache.
fn count_removal_paths(
    start: &Partition,
    target: &Partition,
    cache: &mut HashMap<(Partition, Partition), BigInt>,
) -> BigInt {
    if start == target {
        return BigInt::from(1);
    }

    let start_sum: u32 = start.iter().sum();
    let target_sum: u32 = target.iter().sum();
    if start_sum < target_sum {
        return BigInt::from(0);
    }

    let key = (start.clone(), target.clone());
    if let Some(v) = cache.get(&key) {
        return v.clone();
    }

    let l = start.len();
    let mut total = BigInt::from(0);

    for r in 0..l {
        // Check if row r is a corner: either last row, or strictly larger than next row
        if r == l - 1 || start[r + 1] < start[r] {
            let mut new_part: Vec<u32> = start.clone();
            new_part[r] -= 1;
            if new_part[r] == 0 {
                new_part.remove(r);
            }

            // Check containment: new_part >= target (componentwise)
            if new_part.len() < target.len() {
                continue;
            }
            let mut ok = true;
            for i in 0..new_part.len() {
                if i < target.len() && new_part[i] < target[i] {
                    ok = false;
                    break;
                }
            }
            if !ok {
                continue;
            }

            let sub = count_removal_paths(&new_part, target, cache);
            total += sub;
        }
    }

    cache.insert(key, total.clone());
    total
}

/// Multiplicity: number of ways to reduce partition lambda to (i,)
fn multiplicity(
    lambda: &Partition,
    i: u32,
    cache: &mut HashMap<(Partition, Partition), BigInt>,
) -> BigInt {
    let target = vec![i];
    count_removal_paths(lambda, &target, cache)
}

/// Dimension of hook partition (n-m, 1^m) = C(n-1, m)
fn dim_hook(n: u32, m: u32) -> BigInt {
    // C(n-1, m)
    let mut result = BigInt::from(1);
    let top = n - 1;
    for i in 0..m {
        result = result * BigInt::from(top - i) / BigInt::from(i + 1);
    }
    result
}

fn factorial(n: u32) -> BigInt {
    let mut f = BigInt::from(1);
    for i in 2..=n {
        f *= BigInt::from(i);
    }
    f
}

fn compute_p(k: u32) -> BigRational {
    let n = k * (k - 1) / 2 + 1;
    let nfact = factorial(n);

    let mut cache: HashMap<(Partition, Partition), BigInt> = HashMap::new();
    let mut total = BigRational::new(BigInt::from(0), BigInt::from(1));

    for m in 0..n {
        let lam = hook_partition(n, m);

        // Compute product of multiplicities for i = 1..k
        let mut prod = BigInt::from(1);
        let mut zero = false;
        for i in 1..=k {
            let mi = multiplicity(&lam, i, &mut cache);
            if mi == BigInt::from(0) {
                zero = true;
                break;
            }
            prod *= mi;
        }

        if zero {
            continue;
        }

        let d = dim_hook(n, m);
        // d^(k-1)
        let mut d_pow = BigInt::from(1);
        for _ in 0..(k - 1) {
            d_pow *= &d;
        }

        // sign = (-1)^m
        let sign = if m % 2 == 0 {
            BigInt::from(1)
        } else {
            BigInt::from(-1)
        };

        let term = BigRational::new(sign * prod, d_pow);
        total += term;
    }

    total / BigRational::new(nfact, BigInt::from(1))
}

fn main() {
    let p = compute_p(7);

    // Convert to f64 for scientific notation formatting
    // We need high precision, so let's do the division carefully
    // The answer is ~4.7e-29, so we need to handle very small numbers

    // Use string-based decimal conversion for precision
    let numer = p.numer().clone();
    let denom = p.denom().clone();

    // Compute mantissa and exponent
    // First, find the order of magnitude
    // log10(p) = log10(numer) - log10(denom)
    // We need 10 digits after decimal point in mantissa

    // Strategy: multiply numerator by 10^(precision + safety) and divide by denominator
    // to get an integer, then format

    // First determine the exponent by checking how many digits in numer vs denom
    let numer_abs = if numer < BigInt::from(0) {
        -&numer
    } else {
        numer.clone()
    };
    let is_negative = numer < BigInt::from(0);

    // Find exponent: we want mantissa in [1, 10)
    // Compute numer * 10^60 / denom to get enough digits
    let precision = 60u32;
    let ten = BigInt::from(10);
    let mut scale = BigInt::from(1);
    for _ in 0..precision {
        scale *= &ten;
    }

    let scaled = &numer_abs * &scale / &denom;
    let digits = scaled.to_string();
    let num_digits = digits.len() as i32;

    // scaled = numer/denom * 10^precision
    // If scaled has D digits, then numer/denom ~ scaled / 10^precision ~ 10^(D-1) / 10^precision
    // = 10^(D - 1 - precision)
    // So exponent = D - 1 - precision
    let exponent = num_digits - 1 - precision as i32;

    // Mantissa digits: first 11 digits of `digits` string give us X.YYYYYYYYYY
    // We need 11 significant digits (1 before decimal + 10 after)
    // But we should round properly

    // Take first 12 digits for rounding
    let sig_digits = 11; // 1 + 10 after decimal point
    let round_digits = sig_digits + 1;

    let digit_chars: Vec<u8> = digits.bytes().take(round_digits).collect();

    // Round: check the (sig_digits+1)th digit
    let mut mantissa_digits: Vec<u8> = digit_chars[..sig_digits.min(digit_chars.len())].to_vec();

    if digit_chars.len() > sig_digits && digit_chars[sig_digits] >= b'5' {
        // Round up
        let mut carry = true;
        for d in mantissa_digits.iter_mut().rev() {
            if carry {
                if *d == b'9' {
                    *d = b'0';
                } else {
                    *d += 1;
                    carry = false;
                }
            }
        }
        // If carry propagated all the way, we'd need to handle it
        // but that won't happen for this problem
    }

    // Format as X.YYYYYYYYYY
    let mantissa_str: String = std::iter::once(mantissa_digits[0] as char)
        .chain(std::iter::once('.'))
        .chain(mantissa_digits[1..].iter().map(|&b| b as char))
        .collect();

    let sign_str = if is_negative { "-" } else { "" };
    println!("{}{}e{}", sign_str, mantissa_str, exponent);
}
