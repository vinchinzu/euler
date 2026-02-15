// Project Euler 792 - Too Many Twos
// u(n) = n + 2 + v2(T(n)) using precomputed odd factorial table mod 2^33.

fn main() {
    const TABLE_SIZE: usize = 1 << 27;
    const B_EFF: u32 = 33;
    const MASK_EFF: u64 = (1u64 << B_EFF) - 1;
    let n_val = 10000;

    // Precompute table of cumulative products of odd numbers mod 2^33
    let mut table = vec![1u64; TABLE_SIZE];
    for i in 1..TABLE_SIZE {
        if i & 1 == 1 {
            table[i] = (table[i - 1].wrapping_mul(i as u64)) & MASK_EFF;
        } else {
            table[i] = table[i - 1];
        }
    }

    let mod_inv_ull = |a: u64| -> u64 {
        let mut x = 1u64;
        for _ in 0..40 {
            x = (x.wrapping_mul(2u64.wrapping_sub(a.wrapping_mul(x)))) & MASK_EFF;
        }
        x
    };

    let v2_factorial = |n: i64| -> i64 {
        n - (n as u64).count_ones() as i64
    };

    let oddpart_factorial = |mut n: i64| -> u64 {
        let mut result = 1u64;
        while n > 0 {
            result = (result.wrapping_mul(table[(n as usize) & (TABLE_SIZE - 1)])) & MASK_EFF;
            n >>= 1;
        }
        result
    };

    let ncr_mod = |a: i64, b: i64| -> u64 {
        if b < 0 || b > a { return 0; }
        let num = oddpart_factorial(a);
        let den = (oddpart_factorial(b).wrapping_mul(oddpart_factorial(a - b))) & MASK_EFF;
        let inv_den = mod_inv_ull(den);
        let odd_part = (num.wrapping_mul(inv_den)) & MASK_EFF;
        let exp2 = v2_factorial(a) - v2_factorial(b) - v2_factorial(a - b);
        if exp2 >= B_EFF as i64 || exp2 < 0 { return 0; }
        (odd_part << exp2 as u32) & MASK_EFF
    };

    let mut ans: i64 = 0;

    for m in 1..=n_val {
        let n: i64 = (m as i64) * (m as i64) * (m as i64);
        let mut res: u64 = 0;
        let mut found = false;
        for k in 0..B_EFF {
            let term = ncr_mod(2 * n + 1, n + k as i64 + 1);
            if k % 2 == 0 {
                res = (res.wrapping_add(term)) & MASK_EFF;
            } else {
                res = (res.wrapping_sub(term).wrapping_add(1u64 << B_EFF)) & MASK_EFF;
            }
            if res & 1 == 1 {
                ans += n + 2 + k as i64;
                found = true;
                break;
            }
            res >>= 1;
        }
        if !found {
            ans += n + 2 + B_EFF as i64;
        }
    }

    println!("{}", ans);
}
