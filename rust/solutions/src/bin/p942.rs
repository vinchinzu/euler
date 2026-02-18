// Problem 942
// TODO: Port the Python solution below to Rust
//
// === Python reference ===
// #!/usr/bin/env python3
// """Project Euler 942: Mersenne's Square Root
//
// We need R(q): the minimal x > 0 such that x^2 \equiv q (mod p) where p = 2^q - 1.
// For the given q, p is a (Mersenne) prime.
//
// Key identity (q an odd prime, q \equiv 1 (mod 4)):
// Let (a/q) be the Legendre symbol. Because p = 2^q - 1 is prime, 2 has exact order q modulo p,
// so 2 is a primitive q-th root of unity in F_p. The quadratic Gauss sum
//
//     G = sum_{a=1..q-1} (a/q) * 2^a    (computed in F_p)
//
// satisfies G^2 \equiv q (mod p). Thus G is a square root of q modulo p (up to sign).
//
// We only need the final answer modulo MOD = 1e9+7, and G is a +/- sum of powers of two.
// So we compute the same sum modulo MOD.
//
// The minimal root is min(G, p-G) as integers in [0,p). For q \equiv 1 (mod 4), the top term
// 2^{q-1} has coefficient (q-1/q) = (−1/q) = +1, so:
//
//     G = 2^{q-1} + L,   where L = sum_{a=1..q-2} (a/q) 2^a.
//
// The sign of L is determined by its highest term 2^{q-2} with coefficient (−2/q).
// If (−2/q) = +1 then L > 0 and G > 2^{q-1}, hence the minimal root is p-G.
// If (−2/q) = −1 then L < 0 and the minimal root is G.
//
// We compute the Legendre symbol table by marking quadratic residues r = i^2 mod q.
// """
// === End Python reference ===

const MOD: u64 = 1_000_000_007;

fn gauss_sum_mod(q: usize, modulo: u64) -> u64 {
    let mut qr = vec![false; q];
    let half = (q - 1) / 2;

    // Mark non-zero quadratic residues mod q using consecutive square differences.
    let mut sq = 1usize;
    let mut delta = 3usize;
    for _ in 0..half {
        qr[sq] = true;
        sq += delta;
        if sq >= q {
            sq -= q;
        }
        delta += 2;
    }

    let mut s = 0u64;
    let mut pow2 = 2u64 % modulo;
    for is_residue in qr.iter().skip(1) {
        if *is_residue {
            s += pow2;
            if s >= modulo {
                s -= modulo;
            }
        } else if s >= pow2 {
            s -= pow2;
        } else {
            s = s + modulo - pow2;
        }

        pow2 <<= 1;
        if pow2 >= modulo {
            pow2 -= modulo;
        }
    }

    s
}

fn legendre_minus_two_prime(q: usize) -> i8 {
    match q & 7 {
        1 | 3 => 1,
        5 | 7 => -1,
        _ => unreachable!("q is odd prime"),
    }
}

fn mod_pow(mut base: u64, mut exp: usize, modulo: u64) -> u64 {
    let mut res = 1u64;
    base %= modulo;
    while exp > 0 {
        if exp & 1 == 1 {
            res = (res as u128 * base as u128 % modulo as u128) as u64;
        }
        base = (base as u128 * base as u128 % modulo as u128) as u64;
        exp >>= 1;
    }
    res
}

fn r_mod(q: usize, modulo: u64) -> u64 {
    assert!(q >= 3 && q % 2 == 1, "q must be an odd prime >= 3");

    let g = gauss_sum_mod(q, modulo);
    let p_mod = (mod_pow(2, q, modulo) + modulo - 1) % modulo;

    if legendre_minus_two_prime(q) == 1 {
        (p_mod + modulo - g) % modulo
    } else {
        g
    }
}

fn main() {
    assert_eq!(r_mod(5, MOD), 6);
    assert_eq!(r_mod(17, MOD), 47_569);

    let q = 74_207_281usize;
    println!("{}", r_mod(q, MOD));
}
