// Project Euler 655 - Divisible Palindromes
// Count palindromes up to 32 digits divisible by K=10^7+19.
// First 8 half-digit steps fit in i32 (10^9); then promote to i64.

fn pow_mod(mut base: i64, mut exp: i64, m: i64) -> i64 {
    let mut result = 1i64;
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

fn step_i32(dp: &[i32], new_dp: &mut [i32], ku: usize, shifts: &[usize; 9]) {
    const CHUNK: usize = 8192;
    for chunk_start in (0..ku).step_by(CHUNK) {
        let chunk_end = (chunk_start + CHUNK).min(ku);
        let len = chunk_end - chunk_start;
        let dst = &mut new_dp[chunk_start..chunk_end];
        dst.copy_from_slice(&dp[chunk_start..chunk_end]);
        for &shift in shifts {
            let offset = if chunk_start >= shift {
                chunk_start - shift
            } else {
                chunk_start + ku - shift
            };
            if offset + len <= ku {
                for (d_val, &s_val) in dst.iter_mut().zip(&dp[offset..offset + len]) {
                    *d_val += s_val;
                }
            } else {
                let part1 = ku - offset;
                for (d_val, &s_val) in dst[..part1].iter_mut().zip(&dp[offset..ku]) {
                    *d_val += s_val;
                }
                for (d_val, &s_val) in dst[part1..].iter_mut().zip(&dp[..len - part1]) {
                    *d_val += s_val;
                }
            }
        }
    }
}

fn step_i64(dp: &[i64], new_dp: &mut [i64], ku: usize, shifts: &[usize; 9]) {
    const CHUNK: usize = 8192;
    for chunk_start in (0..ku).step_by(CHUNK) {
        let chunk_end = (chunk_start + CHUNK).min(ku);
        let len = chunk_end - chunk_start;
        let dst = &mut new_dp[chunk_start..chunk_end];
        dst.copy_from_slice(&dp[chunk_start..chunk_end]);
        for &shift in shifts {
            let offset = if chunk_start >= shift {
                chunk_start - shift
            } else {
                chunk_start + ku - shift
            };
            if offset + len <= ku {
                for (d_val, &s_val) in dst.iter_mut().zip(&dp[offset..offset + len]) {
                    *d_val += s_val;
                }
            } else {
                let part1 = ku - offset;
                for (d_val, &s_val) in dst[..part1].iter_mut().zip(&dp[offset..ku]) {
                    *d_val += s_val;
                }
                for (d_val, &s_val) in dst[part1..].iter_mut().zip(&dp[..len - part1]) {
                    *d_val += s_val;
                }
            }
        }
    }
}

fn num_palindromes(num_digits: usize, k: i64) -> i64 {
    let ku = k as usize;
    let half = (num_digits + 1) / 2;
    if half == 0 {
        return 0;
    }
    let inv10 = pow_mod(10, k - 2, k);
    let mut lo = 1i64;
    let mut hi = pow_mod(10, (num_digits - 1) as i64, k);

    let mut dp32 = vec![0i32; ku];
    let mult = if 1 == num_digits { lo } else { (lo + hi) % k };
    for d in 0..10i64 {
        let shift = (mult * d % k) as usize;
        dp32[shift] += 1;
    }
    lo = lo * 10 % k;
    hi = hi * inv10 % k;

    let mut new32 = vec![0i32; ku];
    // After t extra digits, max count is 10^{t+1}. i32 lasts through t=8 (10^9).
    let i32_steps = (half - 1).min(8);
    for i in 1..=i32_steps {
        let mult = if 2 * i + 1 == num_digits {
            lo
        } else {
            (lo + hi) % k
        };
        let shifts: [usize; 9] = std::array::from_fn(|idx| {
            let d = (idx + 1) as i64;
            (mult * d % k) as usize
        });
        step_i32(&dp32, &mut new32, ku, &shifts);
        std::mem::swap(&mut dp32, &mut new32);
        lo = lo * 10 % k;
        hi = hi * inv10 % k;
    }

    if i32_steps + 1 >= half {
        return dp32[0] as i64 - 1;
    }

    let mut dp = vec![0i64; ku];
    for (d, &v) in dp.iter_mut().zip(dp32.iter()) {
        *d = v as i64;
    }
    drop(dp32);
    drop(new32);
    let mut new_dp = vec![0i64; ku];
    for i in (i32_steps + 1)..half {
        let mult = if 2 * i + 1 == num_digits {
            lo
        } else {
            (lo + hi) % k
        };
        let shifts: [usize; 9] = std::array::from_fn(|idx| {
            let d = (idx + 1) as i64;
            (mult * d % k) as usize
        });
        step_i64(&dp, &mut new_dp, ku, &shifts);
        std::mem::swap(&mut dp, &mut new_dp);
        lo = lo * 10 % k;
        hi = hi * inv10 % k;
    }
    dp[0] - 1
}

fn main() {
    let k: i64 = 10_000_019;
    let nn = 32;
    let (ans1, ans2) = rayon::join(
        || num_palindromes(nn - 1, k),
        || num_palindromes(nn, k),
    );
    let ans = ans1 + ans2;
    println!("{}", ans);
}
