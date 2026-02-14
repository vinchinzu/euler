// Project Euler 284: Steady Squares in base 14

const MAXN: usize = 10001;

struct BigNum {
    digits: Vec<i32>, // base-14 digits, d[0] is least significant
}

impl BigNum {
    fn new(v: i32) -> Self {
        let mut digits = vec![0i32; MAXN];
        if v == 0 {
            return BigNum { digits };
        }
        let mut val = v;
        let mut i = 0;
        while val > 0 {
            digits[i] = val % 14;
            val /= 14;
            i += 1;
        }
        BigNum { digits }
    }

    fn mul(b: &BigNum, c: &BigNum, maxlen: usize) -> BigNum {
        let mut tmp = vec![0i32; maxlen];
        for i in 0..maxlen {
            if b.digits[i] == 0 { continue; }
            let mut carry = 0i64;
            for j in 0..maxlen {
                if i + j >= maxlen { break; }
                let s = tmp[i + j] as i64 + b.digits[i] as i64 * c.digits[j] as i64 + carry;
                tmp[i + j] = (s % 14) as i32;
                carry = s / 14;
            }
            let mut k = i + maxlen.min(c.digits.len());
            while carry != 0 && k < maxlen {
                let s = tmp[k] as i64 + carry;
                tmp[k] = (s % 14) as i32;
                carry = s / 14;
                k += 1;
            }
        }
        let mut digits = vec![0i32; MAXN];
        digits[..maxlen].copy_from_slice(&tmp[..maxlen]);
        BigNum { digits }
    }

    fn hensel_step(b: &BigNum, maxlen: usize) -> BigNum {
        // b^2 * (3 - 2*b) mod 14^maxlen
        let b2 = BigNum::mul(b, b, maxlen);

        // factor = 3 - 2*b mod 14^maxlen
        let mut factor = BigNum::new(0);
        factor.digits[0] = 3;
        let mut borrow = 0i32;
        for i in 0..maxlen {
            let mut val = factor.digits[i] - 2 * b.digits[i] - borrow;
            if val < 0 {
                let neg = -val;
                let borrows_needed = (neg + 13) / 14;
                val += borrows_needed * 14;
                borrow = borrows_needed;
            } else {
                borrow = 0;
            }
            factor.digits[i] = val % 14;
        }

        BigNum::mul(&b2, &factor, maxlen)
    }
}

fn main() {
    let n = 10000usize;

    // Compute a: start with 8
    let mut a = BigNum::new(8);
    let mut prec = 1usize;
    while prec < n {
        let new_prec = (prec * 2).min(n);
        let tmp = BigNum::hensel_step(&a, new_prec);
        a = tmp;
        prec = new_prec;
    }

    // Compute b: start with 7
    let mut b = BigNum::new(7);
    prec = 1;
    while prec < n {
        let new_prec = (prec * 2).min(n);
        let tmp = BigNum::hensel_step(&b, new_prec);
        b = tmp;
        prec = new_prec;
    }

    // Compute digit sum contributions
    let mut result: i64 = 1; // For trivial steady square: 1

    // Process a
    {
        let d = &a.digits;
        let mut suffix = 0i64;
        let mut total = 0i64;
        for i in (0..n).rev() {
            if d[i] != 0 { suffix += 1; }
            total += d[i] as i64 * suffix;
        }
        result += total;
    }

    // Process b
    {
        let d = &b.digits;
        let mut suffix = 0i64;
        let mut total = 0i64;
        for i in (0..n).rev() {
            if d[i] != 0 { suffix += 1; }
            total += d[i] as i64 * suffix;
        }
        result += total;
    }

    // Convert result to base 14 and print
    let hex_chars: &[u8] = b"0123456789abcd";
    let mut buf = Vec::new();
    let mut v = result;
    while v > 0 {
        buf.push(hex_chars[(v % 14) as usize]);
        v /= 14;
    }
    buf.reverse();
    let s: String = buf.iter().map(|&c| c as char).collect();
    println!("{}", s);
}
