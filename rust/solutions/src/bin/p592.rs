// Project Euler 592 - Factorial Trailing Hex Digits
//
// Find the last 12 hexadecimal digits before trailing zeros in (20!)!.
// Uses baby-step/giant-step with polynomial interpolation for
// product of odd numbers mod 2^48.

const NBITS: u32 = 48;
const MOD: u64 = 1u64 << NBITS;
const HALF_MOD: u64 = 1u64 << (NBITS - 1);
const MASK: u64 = MOD - 1;
const POLY_DEG: usize = 26;
const BLOCK_BITS: u32 = 22;
const BLOCK_SIZE: u64 = 1u64 << BLOCK_BITS;

#[inline]
fn mulmod(a: u64, b: u64) -> u64 {
    ((a as u128 * b as u128) & MASK as u128) as u64
}

#[inline]
fn addmod(a: u64, b: u64) -> u64 {
    (a.wrapping_add(b)) & MASK
}

#[inline]
fn submod(a: u64, b: u64) -> u64 {
    (a.wrapping_sub(b)) & MASK
}

fn product_of_odds(r: u64) -> u64 {
    if r <= 1 { return 1; }

    let b = BLOCK_SIZE;

    if r <= b || r <= (POLY_DEG as u64 + 2) {
        let mut result: u64 = 1;
        for j in 0..r {
            result = mulmod(result, (2u64.wrapping_mul(j).wrapping_add(1)) & MASK);
        }
        return result;
    }

    // Step 1: Precompute f(a) for a = 0..POLY_DEG
    let mut f_vals = [0u64; POLY_DEG + 1];
    for a in 0..=POLY_DEG {
        let mut prod: u64 = 1;
        let base = (a as u64).wrapping_mul(b);
        for j in 0..b {
            let odd_num = (2u64.wrapping_mul(base.wrapping_add(j)).wrapping_add(1)) & MASK;
            prod = mulmod(prod, odd_num);
        }
        f_vals[a] = prod;
    }

    // Step 2: Build forward difference table
    let mut deltas = [0u64; POLY_DEG + 1];
    deltas[0] = f_vals[0];
    let mut work = f_vals;
    for k in 0..POLY_DEG {
        for i in 0..POLY_DEG - k {
            work[i] = submod(work[i + 1], work[i]);
        }
        deltas[k + 1] = work[0];
    }

    // Step 3: Evaluate f at a=0..q-1 and accumulate product
    let q = r / b;
    let remainder = r % b;

    let mut d = deltas;
    let mut result: u64 = 1;
    for _ in 0..q {
        result = mulmod(result, d[0]);
        for k in 0..POLY_DEG {
            d[k] = addmod(d[k], d[k + 1]);
        }
    }

    // Step 4: Handle partial last block
    if remainder > 0 {
        let base = q * b;
        for j in 0..remainder {
            let odd_num = (2u64.wrapping_mul(base.wrapping_add(j)).wrapping_add(1)) & MASK;
            result = mulmod(result, odd_num);
        }
    }

    result
}

fn main() {
    // N = 20!
    let n: u64 = 2_432_902_008_176_640_000;

    // Compute odd_part(N!) mod 2^48
    let mut odd_part: u64 = 1;
    let mut cur = n;

    while cur > 1 {
        let r = (cur + 1) / 2;
        let r_red = r & (HALF_MOD - 1);
        let po = product_of_odds(r_red);
        odd_part = mulmod(odd_part, po);
        cur /= 2;
    }

    // Compute v2(N!)
    let mut v2: u64 = 0;
    let mut t = n;
    while t > 1 {
        t /= 2;
        v2 += t;
    }

    // Answer = odd_part * 2^(v2 mod 4) mod 2^48
    let pow2 = 1u64 << (v2 % 4);
    let answer = mulmod(odd_part, pow2);

    println!("{:012X}", answer);
}
