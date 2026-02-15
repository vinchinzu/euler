// Project Euler 418: Factorisation triples
// Find minimum a+b+c where a*b*c = 43!, a <= b <= c.
// Enumerate factors near cube root, brute force pairs.

const PRIMES: [u64; 14] = [2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43];
const NPRIMES: usize = 14;

fn num_factors_in_factorial(n: u64, p: u64) -> usize {
    let mut count = 0;
    let mut power = p;
    while power <= n {
        count += (n / power) as usize;
        power *= p;
    }
    count
}

// Big number as array of limbs (base 10^9)
const MAXLIMBS: usize = 20;
const LIMB_BASE: u128 = 1_000_000_000;

#[derive(Clone)]
struct BigNum {
    limbs: [u128; MAXLIMBS],
    nlimbs: usize,
}

impl BigNum {
    fn one() -> Self {
        let mut b = BigNum { limbs: [0; MAXLIMBS], nlimbs: 1 };
        b.limbs[0] = 1;
        b
    }

    fn mul_int(&mut self, x: u128) {
        let mut carry: u128 = 0;
        for i in 0..self.nlimbs {
            let v = self.limbs[i] * x + carry;
            self.limbs[i] = v % LIMB_BASE;
            carry = v / LIMB_BASE;
        }
        while carry > 0 {
            self.limbs[self.nlimbs] = carry % LIMB_BASE;
            carry /= LIMB_BASE;
            self.nlimbs += 1;
        }
    }

    fn cmp(&self, other: &BigNum) -> std::cmp::Ordering {
        use std::cmp::Ordering::*;
        if self.nlimbs != other.nlimbs {
            return self.nlimbs.cmp(&other.nlimbs);
        }
        for i in (0..self.nlimbs).rev() {
            if self.limbs[i] != other.limbs[i] {
                return self.limbs[i].cmp(&other.limbs[i]);
            }
        }
        Equal
    }

    fn add(a: &BigNum, b: &BigNum) -> BigNum {
        let n = a.nlimbs.max(b.nlimbs);
        let mut res = BigNum { limbs: [0; MAXLIMBS], nlimbs: n };
        let mut carry: u128 = 0;
        for i in 0..n {
            let va = if i < a.nlimbs { a.limbs[i] } else { 0 };
            let vb = if i < b.nlimbs { b.limbs[i] } else { 0 };
            let s = va + vb + carry;
            res.limbs[i] = s % LIMB_BASE;
            carry = s / LIMB_BASE;
        }
        if carry > 0 {
            res.limbs[n] = carry;
            res.nlimbs = n + 1;
        }
        res
    }

    fn from_exponents(exp: &[usize; NPRIMES]) -> BigNum {
        let mut b = BigNum::one();
        for i in 0..NPRIMES {
            for _ in 0..exp[i] {
                b.mul_int(PRIMES[i] as u128);
            }
        }
        b
    }

    fn print(&self) {
        print!("{}", self.limbs[self.nlimbs - 1]);
        for i in (0..self.nlimbs - 1).rev() {
            print!("{:09}", self.limbs[i]);
        }
    }
}

fn main() {
    let mut fact_exp = [0usize; NPRIMES];
    let mut log_primes = [0.0f64; NPRIMES];
    for i in 0..NPRIMES {
        fact_exp[i] = num_factors_in_factorial(43, PRIMES[i]);
        log_primes[i] = (PRIMES[i] as f64).ln();
    }

    let total_log: f64 = (0..NPRIMES).map(|i| fact_exp[i] as f64 * log_primes[i]).sum();
    let l_log = total_log / 3.0;

    let r = 0.99999f64;
    let lo = l_log + r.ln();
    let hi = l_log - r.ln();

    // Enumerate factors near cube root
    let mut factors: Vec<[usize; NPRIMES]> = Vec::new();

    fn enumerate(
        idx: usize, cur_exp: &mut [usize; NPRIMES], logv: f64,
        lo: f64, hi: f64, fact_exp: &[usize; NPRIMES],
        log_primes: &[f64; NPRIMES], factors: &mut Vec<[usize; NPRIMES]>,
    ) {
        if logv > hi { return; }
        if idx == NPRIMES {
            if logv >= lo {
                factors.push(*cur_exp);
            }
            return;
        }
        for e in 0..=fact_exp[idx] {
            cur_exp[idx] = e;
            enumerate(idx + 1, cur_exp, logv + e as f64 * log_primes[idx],
                      lo, hi, fact_exp, log_primes, factors);
        }
        cur_exp[idx] = 0;
    }

    let mut cur_exp = [0usize; NPRIMES];
    enumerate(0, &mut cur_exp, 0.0, lo, hi, &fact_exp, &log_primes, &mut factors);

    eprintln!("Found {} factors near cube root", factors.len());

    // Find minimum sum
    let mut best_sum = BigNum { limbs: [LIMB_BASE as u128 - 1; MAXLIMBS], nlimbs: MAXLIMBS };

    let nf = factors.len();
    for i in 0..nf {
        for j in 0..nf {
            // Check f1 * f2 divides 43!
            let mut ok = true;
            let mut f3_exp = [0usize; NPRIMES];
            for k in 0..NPRIMES {
                if factors[i][k] + factors[j][k] > fact_exp[k] {
                    ok = false;
                    break;
                }
                f3_exp[k] = fact_exp[k] - factors[i][k] - factors[j][k];
            }
            if !ok { continue; }

            let b1 = BigNum::from_exponents(&factors[i]);
            let b2 = BigNum::from_exponents(&factors[j]);
            let b3 = BigNum::from_exponents(&f3_exp);
            let sum12 = BigNum::add(&b1, &b2);
            let sum123 = BigNum::add(&sum12, &b3);

            if sum123.cmp(&best_sum) == std::cmp::Ordering::Less {
                best_sum = sum123;
            }
        }
    }

    best_sum.print();
    println!();
}
