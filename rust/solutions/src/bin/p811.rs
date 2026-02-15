// Project Euler 811 - Recursive Binary Function
// H(t, r) via bignum binary expansion of (2^t + 1)^r

const MAX_WORDS: usize = 128;

fn pow_mod(mut base: i64, mut exp: i64, modulus: i64) -> i64 {
    let mut result: i64 = 1;
    base = base.rem_euclid(modulus);
    while exp > 0 {
        if exp & 1 == 1 {
            result = (result as i128 * base as i128 % modulus as i128) as i64;
        }
        base = (base as i128 * base as i128 % modulus as i128) as i64;
        exp >>= 1;
    }
    result
}

fn mod_inv(a: i64, modulus: i64) -> i64 {
    pow_mod(a, modulus - 2, modulus)
}

struct BigNum {
    words: [u64; MAX_WORDS],
}

impl BigNum {
    fn new() -> Self {
        BigNum { words: [0u64; MAX_WORDS] }
    }

    fn add_at(&mut self, val: u64, bit_shift: usize) {
        if val == 0 { return; }
        let word_off = bit_shift / 64;
        let bit_off = bit_shift % 64;

        let lo = if bit_off == 0 { val } else { val << bit_off };
        let hi = if bit_off == 0 { 0 } else { val >> (64 - bit_off) };

        // Add lo
        let mut idx = word_off;
        let (sum, mut carry) = self.words[idx].overflowing_add(lo);
        self.words[idx] = sum;
        idx += 1;

        // Add hi + carry
        let (s1, c1) = hi.overflowing_add(carry as u64);
        let (s2, c2) = self.words[idx].overflowing_add(s1);
        self.words[idx] = s2;
        carry = c1 || c2;
        idx += 1;

        while carry && idx < MAX_WORDS {
            let (s, c) = self.words[idx].overflowing_add(1);
            self.words[idx] = s;
            carry = c;
            idx += 1;
        }
    }

    fn bit(&self, pos: usize) -> bool {
        let word = pos / 64;
        let bit = pos % 64;
        if word >= MAX_WORDS { return false; }
        (self.words[word] >> bit) & 1 == 1
    }

    fn highest_bit(&self) -> i32 {
        for w in (0..MAX_WORDS).rev() {
            if self.words[w] != 0 {
                return (w * 64 + 63 - self.words[w].leading_zeros() as usize) as i32;
            }
        }
        -1
    }
}

fn h(t: usize, r: usize, m: i64) -> i64 {
    let mut num = BigNum::new();

    let mut binom: u64 = 1;
    for i in 0..=r {
        num.add_at(binom, t * i);
        if i < r {
            binom = binom * (r - i) as u64 / (i + 1) as u64;
        }
    }

    let hb = num.highest_bit();
    if hb < 0 { return 1; }

    let mut result: i64 = 1;
    let mut mult: i64 = 1;
    let mut gap_len: i64 = 0;

    for pos in (0..=hb as usize).rev() {
        if num.bit(pos) {
            if gap_len > 0 {
                result = (result as i128 * pow_mod(mult, gap_len, m) as i128 % m as i128) as i64;
            }
            mult = (5 * mult + 3) % m;
            gap_len = 0;
        } else {
            gap_len += 1;
        }
    }

    result
}

fn main() {
    let n: i64 = 100_000_000_000_000 + 31; // 10^14 + 31
    let k: usize = 62;
    let m: i64 = 1_000_062_031;

    let v1 = h(k + 1, k, m);
    let v2 = h(k + 2, k, m);
    let ratio = (v2 as i128 * mod_inv(v1, m) as i128 % m as i128) as i64;

    let answer = (v1 as i128 * pow_mod(ratio, n - k as i64 - 1, m) as i128 % m as i128) as i64;
    println!("{}", answer);
}
