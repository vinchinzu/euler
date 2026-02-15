// Project Euler 813 - XOR Power
// Compute K^N in XOR multiplication, K=11, N=8^12*12^8 = 2^52 * 6561

const MAX_DEG: usize = 20000;
const NWORDS: usize = (MAX_DEG + 63) / 64;

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

struct GF2Poly {
    bits: Vec<u64>,
}

impl GF2Poly {
    fn new() -> Self {
        GF2Poly { bits: vec![0u64; NWORDS] }
    }

    fn bit(&self, pos: usize) -> bool {
        (self.bits[pos / 64] >> (pos % 64)) & 1 == 1
    }

    fn set(&mut self, pos: usize) {
        self.bits[pos / 64] |= 1u64 << (pos % 64);
    }

    fn max_degree(&self) -> i32 {
        for w in (0..NWORDS).rev() {
            if self.bits[w] != 0 {
                return (w * 64 + 63 - self.bits[w].leading_zeros() as usize) as i32;
            }
        }
        -1
    }

    fn mul(&self, other: &GF2Poly) -> GF2Poly {
        let mut result = GF2Poly::new();
        let max_a = self.max_degree();
        let max_b = other.max_degree();
        if max_a < 0 || max_b < 0 { return result; }

        for i in 0..=max_a as usize {
            if !self.bit(i) { continue; }
            let word_off = i / 64;
            let bit_off = i % 64;
            let max_w_b = max_b as usize / 64;

            if bit_off == 0 {
                for w in 0..=max_w_b {
                    result.bits[w + word_off] ^= other.bits[w];
                }
            } else {
                for w in 0..=max_w_b {
                    result.bits[w + word_off] ^= other.bits[w] << bit_off;
                    if w + word_off + 1 < NWORDS {
                        result.bits[w + word_off + 1] ^= other.bits[w] >> (64 - bit_off);
                    }
                }
            }
        }
        result
    }
}

fn main() {
    // K^(2^52) represented with reduced positions: {0, 1, 3}
    // Raise to power 6561
    let mut base = GF2Poly::new();
    base.set(0);
    base.set(1);
    base.set(3);

    let mut result = GF2Poly::new();
    result.set(0); // 1

    let n = 6561;
    let hb = 64 - (n as u64).leading_zeros() as i32 - 1;

    for bit in (0..=hb).rev() {
        // Square result
        let temp = result.mul(&result);
        result = temp;

        if (n >> bit) & 1 == 1 {
            let temp = result.mul(&base);
            result = temp;
        }
    }

    let m: i64 = 1_000_000_007;
    let base_val = pow_mod(2, 1i64 << 52, m); // 2^(2^52) mod M

    let mut ans: i64 = 0;
    for i in 0..MAX_DEG {
        if result.bit(i) {
            ans = (ans + pow_mod(base_val, i as i64, m)) % m;
        }
    }

    println!("{}", ans);
}
