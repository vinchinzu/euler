// Project Euler 803 - Pseudorandom Sequence
// LCG: a_{n+1} = (25214903917 * a_n + 11) mod 2^48
// Find index of first occurrence of "LuckyText" given sequence starts with "PuzzleOne"

const MASK: u64 = (1u64 << 48) - 1;
const L: u64 = 1u64 << 16;
const MULT: u64 = 25214903917;
const INC: u64 = 11;

fn char_to_code(c: u8) -> u64 {
    if c >= b'a' && c <= b'z' {
        (c - b'a') as u64
    } else {
        (c - b'A') as u64 + 26
    }
}

fn next_val(a: u64) -> u64 {
    (MULT.wrapping_mul(a).wrapping_add(INC)) & MASK
}

fn find_r(codes: &[u64]) -> i32 {
    for r in 0..L as i32 {
        let mut a = r as u64;
        let mut good = true;
        for i in 1..codes.len() {
            a = next_val(a % L);
            if ((a / L + codes[i - 1] - codes[i]) % 4 + 4) % 4 != 0 {
                good = false;
                break;
            }
        }
        if good { return r; }
    }
    -1
}

fn is_substring(mut a: u64, codes: &[u64]) -> bool {
    for &c in codes {
        if (a / L) % 52 != c { return false; }
        a = next_val(a);
    }
    true
}

fn main() {
    let s_str = b"PuzzleOne";
    let t_str = b"LuckyText";
    let s: Vec<u64> = s_str.iter().map(|&c| char_to_code(c)).collect();
    let t: Vec<u64> = t_str.iter().map(|&c| char_to_code(c)).collect();

    // Find starting value a such that sequence starts with S
    let r_s = find_r(&s);
    let mut a = s[0] * L + r_s as u64;
    while !is_substring(a, &s) {
        a += 52 * L;
    }

    // Find remainder for T
    let r_t = find_r(&t);
    let mut ans: i64 = 0;
    while (a % L) as i32 != r_t {
        a = next_val(a);
        ans += 1;
    }

    // Compute coefficients for a_{n+L}
    let mut c0: u64 = 0;
    let mut c1: u64 = 1;
    for _ in 0..L {
        c0 = next_val(c0);
    }
    for _ in 0..L {
        c1 = next_val(c1);
    }

    let step_mult = c1.wrapping_sub(c0) & MASK;
    let step_add = c0;

    // Find substring T
    while !is_substring(a, &t) {
        a = (step_mult.wrapping_mul(a).wrapping_add(step_add)) & MASK;
        ans += L as i64;
    }

    println!("{}", ans);
}
