// Project Euler 342 - Sum of n where phi(n^2) is a cube.
// Recursive backtracking approach.

const MAXN: i64 = 10_000_000_000;
const SQRT_MAXN: usize = 100_001;
const MAX_PHI: usize = 30;

static mut PRIMES: [i32; 10000] = [0; 10000];
static mut NPRIMES: usize = 0;

fn build_sieve() {
    let mut sieve = vec![false; SQRT_MAXN];
    unsafe {
        NPRIMES = 0;
        for i in 2..SQRT_MAXN {
            if !sieve[i] {
                PRIMES[NPRIMES] = i as i32;
                NPRIMES += 1;
                let mut j = i as u64 * i as u64;
                while (j as usize) < SQRT_MAXN {
                    sieve[j as usize] = true;
                    j += i as u64;
                }
            }
        }
    }
}

fn factorize(mut n: i32) -> Vec<(i32, i32)> {
    let mut factors = Vec::new();
    unsafe {
        for i in 0..NPRIMES {
            let p = PRIMES[i];
            if (p as i64) * (p as i64) > n as i64 { break; }
            if n % p == 0 {
                let mut e = 0;
                while n % p == 0 { n /= p; e += 1; }
                factors.push((p, e));
            }
        }
    }
    if n > 1 { factors.push((n, 1)); }
    factors
}

#[derive(Clone)]
struct PhiState {
    p: [i32; MAX_PHI],
    e: [i32; MAX_PHI],
    cnt: usize,
}

impl PhiState {
    fn new() -> Self { PhiState { p: [0; MAX_PHI], e: [0; MAX_PHI], cnt: 0 } }

    fn find(&self, p: i32) -> Option<usize> {
        for i in 0..self.cnt { if self.p[i] == p { return Some(i); } }
        None
    }

    fn remove(&mut self, idx: usize) {
        self.cnt -= 1;
        if idx < self.cnt {
            self.p[idx] = self.p[self.cnt];
            self.e[idx] = self.e[self.cnt];
        }
    }

    fn max_prime(&self) -> i32 {
        let mut mx = 0;
        for i in 0..self.cnt { if self.p[i] > mx { mx = self.p[i]; } }
        mx
    }
}

fn helper(n: i64, phi: &PhiState, max_prime: i32, ans: &mut i64) {
    if phi.cnt == 0 {
        if n > 1 { *ans += n; }
    } else {
        let mx = phi.max_prime();
        let idx = phi.find(mx).unwrap();
        let e_mod = phi.e[idx] % 3;
        let start_e = if e_mod == 1 { 3 } else { 1 };
        add_prime(n, phi, mx, start_e, mx, ans);
    }

    let mx_p = if phi.cnt > 0 { phi.max_prime() } else { 0 };
    unsafe {
        for i in 0..NPRIMES {
            let p = PRIMES[i];
            if p >= max_prime { break; }
            if n * (p as i64) * (p as i64) >= MAXN { break; }
            if n % (p as i64) == 0 { continue; }
            if phi.cnt > 0 && p < mx_p { continue; }
            if phi.find(p).is_some() { continue; }
            add_prime(n, phi, p, 2, max_prime, ans);
        }
    }
}

fn add_prime(n: i64, phi: &PhiState, p: i32, start_e: i32, max_prime_for_recurse: i32, ans: &mut i64) {
    let mut pe: i64 = 1;
    for _ in 0..start_e {
        if pe > MAXN / p as i64 { return; }
        pe *= p as i64;
    }

    let mut e = start_e;
    loop {
        if n > (MAXN - 1) / pe { break; }

        let mut new_phi = phi.clone();

        if let Some(idx) = new_phi.find(p) {
            new_phi.remove(idx);
        }

        let factors = factorize(p - 1);
        let mut good = true;
        for &(q, f) in &factors {
            if !good { break; }
            let qi = new_phi.find(q);
            let old_val = qi.map_or(0, |i| new_phi.e[i]);
            let new_val = old_val + f;
            if new_val % 3 != 0 {
                if let Some(i) = qi {
                    new_phi.e[i] = new_val;
                } else {
                    if new_phi.cnt >= MAX_PHI { good = false; break; }
                    new_phi.p[new_phi.cnt] = q;
                    new_phi.e[new_phi.cnt] = new_val;
                    new_phi.cnt += 1;
                }
                if n % (q as i64) == 0 { good = false; }
            } else if let Some(i) = qi {
                new_phi.remove(i);
            }
        }

        if good {
            helper(n * pe, &new_phi, p, ans);
        }

        // Next: e += 3
        for _ in 0..3 {
            if pe > MAXN / p as i64 { pe = MAXN + 1; break; }
            pe *= p as i64;
        }
        e += 3;
        let _ = e; // suppress unused warning
        if pe > MAXN { break; }
    }
}

fn main() {
    build_sieve();
    let phi = PhiState::new();
    let mut ans: i64 = 0;
    helper(1, &phi, i32::MAX, &mut ans);
    println!("{}", ans);
}
