// Project Euler Problem 806: Nim on Towers of Hanoi
// Compute f(n) modulo 1_000_000_007

const MOD: i64 = 1_000_000_007;
const INV2: i64 = (MOD + 1) / 2;

fn mod_pow(mut a: i64, mut e: i64, m: i64) -> i64 {
    let mut r = 1i64;
    while e > 0 {
        if e & 1 == 1 {
            r = r * a % m;
        }
        a = a * a % m;
        e >>= 1;
    }
    r
}

struct CoefficientComputer {
    fact: Vec<i64>,
    inv: Vec<i64>,
    invfact: Vec<i64>,
    pow2: Vec<i64>,
    cache: std::collections::HashMap<(i64, i64, i64), i64>,
}

impl CoefficientComputer {
    fn new(nmax: usize) -> Self {
        let fact = Self::precompute_factorials(nmax);
        let inv = Self::precompute_inverses(nmax);
        let invfact = Self::precompute_invfact(nmax, &fact, &inv);
        let pow2 = Self::build_pow2(nmax);

        CoefficientComputer {
            fact,
            inv,
            invfact,
            pow2,
            cache: std::collections::HashMap::new(),
        }
    }

    fn precompute_factorials(nmax: usize) -> Vec<i64> {
        let mut fact = vec![1i64; nmax + 1];
        for i in 1..=nmax {
            fact[i] = fact[i - 1] * i as i64 % MOD;
        }
        fact
    }

    fn precompute_inverses(nmax: usize) -> Vec<i64> {
        let mut inv = vec![0i64; nmax + 1];
        inv[1] = 1;
        for i in 2..=nmax {
            inv[i] = (MOD - (MOD / i as i64) * inv[(MOD % i as i64) as usize] % MOD) % MOD;
        }
        inv
    }

    fn precompute_invfact(nmax: usize, fact: &[i64], inv: &[i64]) -> Vec<i64> {
        let mut invfact = vec![1i64; nmax + 1];
        invfact[nmax] = mod_pow(fact[nmax], MOD - 2, MOD);
        for i in (1..=nmax).rev() {
            invfact[i - 1] = invfact[i] * i as i64 % MOD;
        }
        invfact
    }

    fn build_pow2(nmax: usize) -> Vec<i64> {
        let mut p = vec![1i64; nmax + 1];
        for i in 1..=nmax {
            p[i] = p[i - 1] * 2 % MOD;
        }
        p
    }

    fn denom_coeff(&mut self, a: i64, b: i64, c: i64) -> i64 {
        let key = (a, b, c);
        if let Some(&val) = self.cache.get(&key) {
            return val;
        }

        if a < 0 || b < 0 || c < 0 {
            self.cache.insert(key, 0);
            return 0;
        }

        // Parity constraint
        if ((a ^ b) & 1) != 0 || ((a ^ c) & 1) != 0 {
            self.cache.insert(key, 0);
            return 0;
        }

        let minabc = a.min(b).min(c);
        let mut i = a & 1;
        if i > minabc {
            self.cache.insert(key, 0);
            return 0;
        }

        let mut a = (a - i) / 2;
        let mut b = (b - i) / 2;
        let mut c = (c - i) / 2;
        let mut m = (a + b + c + i) as usize;

        let mut term = self.pow2[i as usize];
        term = term * self.fact[m] % MOD;
        term = term * self.invfact[i as usize] % MOD;
        term = term * self.invfact[a as usize] % MOD;
        term = term * self.invfact[b as usize] % MOD;
        term = term * self.invfact[c as usize] % MOD;

        let mut ans = 0i64;

        loop {
            ans += term;
            if ans >= MOD {
                ans -= MOD;
            }

            let i2 = i + 2;
            if i2 > minabc {
                break;
            }

            // ratio = 4*a*b*c / (m*(i+1)*(i+2))
            let mut ratio = 4 * a % MOD;
            ratio = ratio * b % MOD;
            ratio = ratio * c % MOD;
            ratio = ratio * self.inv[m] % MOD;
            ratio = ratio * self.inv[(i + 1) as usize] % MOD;
            ratio = ratio * self.inv[i2 as usize] % MOD;

            term = term * ratio % MOD;

            i = i2;
            a -= 1;
            b -= 1;
            c -= 1;
            m -= 1;
        }

        self.cache.insert(key, ans);
        ans
    }

    fn full_coeff(&mut self, a: i64, b: i64, c: i64) -> i64 {
        let mut res = self.denom_coeff(a, b, c);
        res = (res + self.denom_coeff(a - 1, b, c)) % MOD;
        res = (res + self.denom_coeff(a, b, c - 1)) % MOD;
        res = (res + self.denom_coeff(a - 1, b - 1, c)) % MOD;
        res = (res + self.denom_coeff(a, b - 1, c - 1)) % MOD;
        res = (res - self.denom_coeff(a, b - 2, c)) % MOD;
        if res < 0 {
            res += MOD;
        }
        res
    }
}

fn xor_zero_triples(n: i64) -> Vec<(i64, i64, i64)> {
    if n & 1 != 0 {
        return Vec::new();
    }

    let mut bits = Vec::new();
    let mut x = n;
    let mut p = 0;
    while x > 0 {
        if x & 1 != 0 {
            bits.push(p);
        }
        x >>= 1;
        p += 1;
    }

    let mut triples = vec![(0i64, 0i64, 0i64)];
    for p in bits {
        if p == 0 {
            continue;
        }
        let v = 1i64 << (p - 1);
        let mut new_list = Vec::with_capacity(triples.len() * 3);
        for (a, b, c) in &triples {
            new_list.push((a + v, b + v, *c));
            new_list.push((a + v, *b, c + v));
            new_list.push((*a, b + v, c + v));
        }
        triples = new_list;
    }
    triples
}

fn count_losing_positions(n: i64, coeff_comp: &mut CoefficientComputer) -> i64 {
    if n & 1 != 0 {
        return 0;
    }
    let mut total = 0i64;
    for (a, b, c) in xor_zero_triples(n) {
        total = (total + coeff_comp.full_coeff(a, b, c)) % MOD;
    }
    total
}

fn solve(n: i64) -> i64 {
    if n & 1 != 0 {
        return 0;
    }
    let mut coeff_comp = CoefficientComputer::new((n + 5) as usize);
    let k = count_losing_positions(n, &mut coeff_comp);
    let val = (mod_pow(2, n, MOD) - 1 + MOD) % MOD;
    k * val % MOD * INV2 % MOD
}

fn main() {
    // Test values
    assert_eq!(solve(4), 30);
    assert_eq!(solve(10), 67518);

    println!("{}", solve(100000));
}
