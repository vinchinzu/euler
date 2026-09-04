// Problem 999: Alternating Recurrence
// a_1=a_2=a_3=1, a_4=2, and a_n^2 = a_{n+2}a_{n-2} + u a_{n+1}a_{n-1}
// with u=1 (n even) or u=2 (n odd). Find a_{10^18+3} mod 1234567891.
//
// Rescaled elliptic divisibility sequence with doubling identities.

const MOD: u64 = 1_234_567_891;
const INV_TWO: u64 = (MOD + 1) / 2;
const TARGET: u64 = 1_000_000_000_000_000_003;
const BARRETT_M: u128 = 14_941_862_823;

const SMALL_W: [i64; 9] = [0, 1, 2, -4, -32, -192, 3584, 77824, 262144];

#[inline(always)]
fn mul(a: u64, b: u64) -> u64 {
    let prod = a * b;
    let q = ((prod as u128 * BARRETT_M) >> 64) as u64;
    let mut rem = prod - q * MOD;
    if rem >= MOD {
        rem -= MOD;
    }
    rem
}

#[inline(always)]
fn sq(a: u64) -> u64 {
    mul(a, a)
}

#[inline(always)]
fn cube(a: u64) -> u64 {
    mul(mul(a, a), a)
}

#[inline(always)]
fn sub(a: u64, b: u64) -> u64 {
    if a >= b { a - b } else { a + MOD - b }
}

#[inline]
fn mpow(mut base: u64, mut exp: u64) -> u64 {
    let mut result = 1u64;
    while exp > 0 {
        if exp & 1 == 1 {
            result = mul(result, base);
        }
        base = mul(base, base);
        exp >>= 1;
    }
    result
}

fn small_w(index: i64) -> u64 {
    if index < 0 {
        return sub(0, small_w(-index));
    }
    let v = SMALL_W[index as usize];
    if v >= 0 {
        v as u64
    } else {
        (MOD as i64 + v) as u64
    }
}

fn eds_block(n: i64) -> [u64; 8] {
    if n <= 4 {
        let mut out = [0u64; 8];
        for (i, slot) in out.iter_mut().enumerate() {
            *slot = small_w(n - 3 + i as i64);
        }
        return out;
    }

    let middle = n / 2;
    let source = eds_block(middle);
    let source_start = middle - 3;
    let get = |index: i64| source[(index - source_start) as usize];

    let odd = |index: i64| {
        sub(
            mul(get(index + 1), cube(get(index - 1))),
            mul(get(index - 2), cube(get(index))),
        )
    };
    let even = |index: i64| {
        mul(
            mul(get(index), INV_TWO),
            sub(
                mul(get(index + 2), sq(get(index - 1))),
                mul(get(index - 2), sq(get(index + 1))),
            ),
        )
    };

    if n % 2 == 0 {
        [
            odd(middle - 1),
            even(middle - 1),
            odd(middle),
            even(middle),
            odd(middle + 1),
            even(middle + 1),
            odd(middle + 2),
            even(middle + 2),
        ]
    } else {
        [
            even(middle - 1),
            odd(middle),
            even(middle),
            odd(middle + 1),
            even(middle + 1),
            odd(middle + 2),
            even(middle + 2),
            odd(middle + 3),
        ]
    }
}

fn w_mod(n: i64) -> u64 {
    if n < 0 {
        return sub(0, w_mod(-n));
    }
    eds_block(n)[3]
}

fn a_mod(n: u64) -> u64 {
    let sign_neg = !matches!(n % 4, 1 | 2);
    let exp = (((n as u128 * n as u128) / 4) % (MOD as u128 - 1)) as u64;
    let inverse_scale = mpow(INV_TWO, exp);
    let val = mul(w_mod(n as i64), inverse_scale);
    if sign_neg { sub(0, val) } else { val }
}

fn main() {
    debug_assert_eq!(a_mod(1), 1);
    debug_assert_eq!(a_mod(2), 1);
    debug_assert_eq!(a_mod(3), 1);
    debug_assert_eq!(a_mod(4), 2);
    debug_assert_eq!(a_mod(13), 23321);
    debug_assert_eq!(a_mod(1003), 231_906_014);
    println!("{}", a_mod(TARGET));
}

#[cfg(test)]
mod tests {
    use super::a_mod;

    #[test]
    fn samples() {
        assert_eq!(a_mod(1), 1);
        assert_eq!(a_mod(2), 1);
        assert_eq!(a_mod(3), 1);
        assert_eq!(a_mod(4), 2);
        assert_eq!(a_mod(13), 23321);
        assert_eq!(a_mod(1003), 231_906_014);
    }
}
