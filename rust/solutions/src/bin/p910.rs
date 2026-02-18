// Project Euler 910 — correct Phi-recursion + CRT solver

use std::collections::HashMap;

const MOD: u64 = 1_000_000_000;
const M1: u64 = 512; // 2^9
const M2: u64 = 1_953_125; // 5^9

const A: usize = 12;
const B: u64 = 345_678;
const C: u64 = 9_012_345;
const D: u64 = 678;
const E: u64 = 90;

fn bit_len(n: u64) -> usize {
    64 - n.leading_zeros() as usize
}

fn mul_mod(a: u64, b: u64, m: u64) -> u64 {
    ((a as u128 * b as u128) % m as u128) as u64
}

fn gcd_u64(mut a: u64, mut b: u64) -> u64 {
    while b != 0 {
        let t = a % b;
        a = b;
        b = t;
    }
    a
}

fn mod_pow(mut base: u64, mut exp: u64, m: u64) -> u64 {
    if m == 1 {
        return 0;
    }
    base %= m;
    let mut result = 1u64;
    while exp > 0 {
        if exp & 1 == 1 {
            result = mul_mod(result, base, m);
        }
        base = mul_mod(base, base, m);
        exp >>= 1;
    }
    result
}

fn egcd(a: i128, b: i128) -> (i128, i128, i128) {
    if b == 0 {
        (a, 1, 0)
    } else {
        let (g, x1, y1) = egcd(b, a % b);
        (g, y1, x1 - (a / b) * y1)
    }
}

fn mod_inverse(a: u64, m: u64) -> u64 {
    let (g, x, _) = egcd(a as i128, m as i128);
    assert_eq!(g, 1);
    ((x.rem_euclid(m as i128)) as u64) % m
}

fn crt(x1: u64, m1: u64, x2: u64, m2: u64) -> u64 {
    let q = mod_inverse(m1, m2);
    let delta = (x2 + m2 - (x1 % m2)) % m2;
    let k = mul_mod(delta, q, m2);
    (x1 as u128 + m1 as u128 * k as u128) as u64 % MOD
}

fn g_mod(p: u64, x: u64, m: u64) -> u64 {
    mul_mod(mod_pow(x, p, m), (x + 1) % m, m)
}

fn build_jump_table(table: &[u64], bits: usize) -> Vec<Vec<u64>> {
    let n = table.len();
    let mut jump = vec![vec![0u64; n]; bits];
    jump[0].copy_from_slice(table);
    for bit in 1..bits {
        for i in 0..n {
            let mid = jump[bit - 1][i] as usize;
            jump[bit][i] = jump[bit - 1][mid];
        }
    }
    jump
}

fn apply_jump(jump: &[Vec<u64>], mut steps: u64, mut x: u64) -> u64 {
    let mut bit = 0usize;
    while steps > 0 {
        if steps & 1 == 1 {
            x = jump[bit][x as usize];
        }
        steps >>= 1;
        bit += 1;
    }
    x
}

fn phi_mod_512() -> u64 {
    let modulus = M1;
    let size = modulus as usize;
    let bits = bit_len(B + 1) + 1;

    let mut gc = vec![0u64; size];
    let mut gcp1 = vec![0u64; size];
    for x in 0..size {
        gc[x] = g_mod(C, x as u64, modulus);
        gcp1[x] = g_mod(C + 1, x as u64, modulus);
    }

    let jump_gc = build_jump_table(&gc, bits);
    let mut phi = vec![vec![0u64; size]; A + 1];

    // Phi_0(x) = g_c^(B+1)(g_{c+1}(x))
    for x in 0..size {
        phi[0][x] = apply_jump(&jump_gc, B + 1, gcp1[x]);
    }

    for level in 1..=A {
        let (left, right) = phi.split_at_mut(level);
        let prev = &left[level - 1];
        let curr = &mut right[0];
        let jump_prev = build_jump_table(prev, bits);
        for x in 0..size {
            let s0 = ((x as u64) * prev[x]) % modulus;
            curr[x] = apply_jump(&jump_prev, B, s0);
        }
    }

    phi[A][(D % modulus) as usize]
}

fn modulus_to_k(mut modulus: u64) -> usize {
    let mut k = 0usize;
    while modulus > 1 && modulus % 5 == 0 {
        modulus /= 5;
        k += 1;
    }
    k
}

fn teichmuller_lift(x: u64, modulus: u64, k: usize) -> u64 {
    let mut z = x % modulus;
    for _ in 0..k {
        z = mod_pow(z, 5, modulus);
    }
    z
}

fn unit_decompose(x: u64, modulus: u64, k: usize) -> (u64, u64) {
    let z = teichmuller_lift(x, modulus, k);
    let inv_z = mod_inverse(z, modulus);
    let y = mul_mod(x, inv_z, modulus);
    if k == 0 {
        (z, 0)
    } else {
        (z, (y - 1) / 5)
    }
}

fn binom_mod_small_r(n: u64, r: usize, modulus: u64) -> u64 {
    if r == 0 {
        return 1 % modulus;
    }
    let mut nums: Vec<u64> = (0..r).map(|i| n - i as u64).collect();
    for d in 2..=r as u64 {
        let mut rem = d;
        for num in &mut nums {
            let g = gcd_u64(*num, rem);
            if g > 1 {
                *num /= g;
                rem /= g;
                if rem == 1 {
                    break;
                }
            }
        }
        debug_assert_eq!(rem, 1);
    }
    nums.into_iter()
        .fold(1u64, |acc, x| mul_mod(acc, x % modulus, modulus))
}

fn binom_prefix_array(exp: u64, k: usize, modulus: u64) -> [u64; 10] {
    let mut coeffs = [0u64; 10];
    coeffs[0] = 1 % modulus;
    for r in 1..=k {
        coeffs[r] = binom_mod_small_r(exp, r, modulus);
    }
    coeffs
}

fn one_plus_5u_pow(u: u64, k: usize, coeffs: &[u64; 10], modulus: u64) -> u64 {
    if k == 0 {
        return 1 % modulus;
    }
    let base = (5 * u) % modulus;
    let mut res = 1u64;
    let mut pow_term = base;
    for r in 1..=k {
        res = (res + mul_mod(coeffs[r], pow_term, modulus)) % modulus;
        pow_term = mul_mod(pow_term, base, modulus);
    }
    res
}

struct GCIterator {
    modulus: u64,
    k5: usize,
    jump_cache: Vec<HashMap<u64, u64>>,
    unit_cache: HashMap<u64, (u64, u64)>,
    binom_c: [u64; 10],
    binom_cp1: [u64; 10],
}

impl GCIterator {
    fn new(modulus: u64, bits: usize) -> Self {
        let k5 = modulus_to_k(modulus);
        let binom_c = binom_prefix_array(C, k5, modulus);
        let binom_cp1 = binom_prefix_array(C + 1, k5, modulus);
        let jump_cache = (0..bits).map(|_| HashMap::new()).collect();
        Self {
            modulus,
            k5,
            jump_cache,
            unit_cache: HashMap::new(),
            binom_c,
            binom_cp1,
        }
    }

    fn pow_unit(&mut self, x: u64, plus_one_exp: bool) -> u64 {
        let xm = x % self.modulus;
        if self.modulus == 1 {
            return 0;
        }
        if xm % 5 == 0 {
            return mod_pow(xm, if plus_one_exp { C + 1 } else { C }, self.modulus);
        }

        let (z, u) = if let Some(&pair) = self.unit_cache.get(&xm) {
            pair
        } else {
            let pair = unit_decompose(xm, self.modulus, self.k5);
            self.unit_cache.insert(xm, pair);
            pair
        };

        let exp = if plus_one_exp { C + 1 } else { C };
        let r = exp % 4;
        let z_pow = mod_pow(z, if r == 0 { 4 } else { r }, self.modulus);
        let coeffs = if plus_one_exp {
            self.binom_cp1
        } else {
            self.binom_c
        };
        let u_pow = one_plus_5u_pow(u, self.k5, &coeffs, self.modulus);
        mul_mod(z_pow, u_pow, self.modulus)
    }

    fn g_c(&mut self, x: u64) -> u64 {
        mul_mod(self.pow_unit(x, false), (x + 1) % self.modulus, self.modulus)
    }

    fn g_cp1(&mut self, x: u64) -> u64 {
        mul_mod(self.pow_unit(x, true), (x + 1) % self.modulus, self.modulus)
    }

    fn jump(&mut self, bit: usize, x: u64) -> u64 {
        let xm = x % self.modulus;
        if let Some(&v) = self.jump_cache[bit].get(&xm) {
            return v;
        }
        let v = if bit == 0 {
            self.g_c(xm)
        } else {
            let mid = self.jump(bit - 1, xm);
            self.jump(bit - 1, mid)
        };
        self.jump_cache[bit].insert(xm, v);
        v
    }

    fn iterate_steps(&mut self, mut steps: u64, mut x: u64) -> u64 {
        let mut bit = 0usize;
        while steps > 0 {
            if steps & 1 == 1 {
                x = self.jump(bit, x);
            }
            steps >>= 1;
            bit += 1;
        }
        x
    }

    fn phi0(&mut self, x: u64) -> u64 {
        let y0 = self.g_cp1(x % self.modulus);
        self.iterate_steps(B + 1, y0)
    }
}

struct Phi5Solver {
    modulus: u64,
    g_iter: GCIterator,
    phi_cache: Vec<HashMap<u64, u64>>,
    jump_cache: Vec<Vec<HashMap<u64, u64>>>,
}

impl Phi5Solver {
    fn new(modulus: u64) -> Self {
        let bits = bit_len(B + 1) + 1;
        let g_iter = GCIterator::new(modulus, bits);
        let levels = A + 1;
        let mut phi_cache = Vec::with_capacity(levels);
        let mut jump_cache = Vec::with_capacity(levels);
        for _ in 0..levels {
            phi_cache.push(HashMap::new());
            jump_cache.push((0..bits).map(|_| HashMap::new()).collect());
        }
        Self {
            modulus,
            g_iter,
            phi_cache,
            jump_cache,
        }
    }

    fn phi(&mut self, level: usize, x: u64) -> u64 {
        let xm = x % self.modulus;
        if let Some(&v) = self.phi_cache[level].get(&xm) {
            return v;
        }

        let v = if level == 0 {
            self.g_iter.phi0(xm)
        } else {
            let prev = self.phi(level - 1, xm);
            let s0 = mul_mod(xm, prev, self.modulus);
            self.iterate_phi(level - 1, B, s0)
        };

        self.phi_cache[level].insert(xm, v);
        v
    }

    fn jump_phi(&mut self, level: usize, bit: usize, x: u64) -> u64 {
        let xm = x % self.modulus;
        if let Some(&v) = self.jump_cache[level][bit].get(&xm) {
            return v;
        }
        let v = if bit == 0 {
            self.phi(level, xm)
        } else {
            let mid = self.jump_phi(level, bit - 1, xm);
            self.jump_phi(level, bit - 1, mid)
        };
        self.jump_cache[level][bit].insert(xm, v);
        v
    }

    fn iterate_phi(&mut self, level: usize, mut steps: u64, mut x: u64) -> u64 {
        let mut bit = 0usize;
        while steps > 0 {
            if steps & 1 == 1 {
                x = self.jump_phi(level, bit, x);
            }
            steps >>= 1;
            bit += 1;
        }
        x
    }
}

fn phi_mod_5pow() -> u64 {
    let mut solver = Phi5Solver::new(M2);
    solver.phi(A, D % M2)
}

fn main() {
    debug_assert_eq!(M1 * M2, MOD);

    let v1 = phi_mod_512();
    let v2 = phi_mod_5pow();
    let ans = (crt(v1, M1, v2, M2) + E) % MOD;
    println!("{}", ans);
}
