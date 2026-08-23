// Project Euler 483 - Repeated permutation
// Average f^2(P) over all permutations of {1..350}
//
// Cycle-index recursion over largest prime factor. Cache keys are canonicalized
// by stripping prime powers that can no longer appear in remaining cycles, then
// stored in a packed open-addressing table (Fx-style mix, linear probe).

const N: usize = 350;
const HBITS: usize = 23;
const HSIZE: usize = 1 << HBITS;
const HMASK: usize = HSIZE - 1;

#[inline(always)]
fn gcd_u64(mut a: u64, mut b: u64) -> u64 {
    if a == 0 {
        return b;
    }
    if b == 0 {
        return a;
    }
    let shift = (a | b).trailing_zeros();
    a >>= a.trailing_zeros();
    loop {
        b >>= b.trailing_zeros();
        if a > b {
            core::mem::swap(&mut a, &mut b);
        }
        b -= a;
        if b == 0 {
            break;
        }
    }
    a << shift
}

#[inline(always)]
fn lcm_u64(a: u64, b: u64) -> u64 {
    a / gcd_u64(a, b) * b
}

#[inline(always)]
fn pack_key(max_index: usize, min_k: usize, n: usize) -> u32 {
    ((max_index as u32) << 17) | ((min_k as u32) << 9) | (n as u32)
}

#[inline(always)]
fn mix(pack: u32, lcm: u64) -> u64 {
    let mut x = lcm ^ (pack as u64).wrapping_mul(0x9E3779B97F4A7C15);
    x ^= x >> 32;
    x = x.wrapping_mul(0xBF58476D1CE4E5B9);
    x ^= x >> 29;
    x = x.wrapping_mul(0x94D049BB133111EB);
    x ^= x >> 32;
    x | 1
}

struct Solver {
    primes: Vec<u32>,
    inv_ffact: Vec<f64>,
    by_lpf: Vec<Vec<u16>>,
    hash: Vec<u64>,
    kpack: Vec<u32>,
    klcm: Vec<u64>,
    kval: Vec<f64>,
}

impl Solver {
    #[inline(always)]
    fn lookup(&self, pack: u32, lcm: u64) -> Option<f64> {
        let h = mix(pack, lcm);
        let mut i = h as usize & HMASK;
        loop {
            let eh = unsafe { *self.hash.get_unchecked(i) };
            if eh == 0 {
                return None;
            }
            if eh == h
                && unsafe { *self.kpack.get_unchecked(i) } == pack
                && unsafe { *self.klcm.get_unchecked(i) } == lcm
            {
                return Some(unsafe { *self.kval.get_unchecked(i) });
            }
            i = (i + 1) & HMASK;
        }
    }

    #[inline(always)]
    fn store(&mut self, pack: u32, lcm: u64, val: f64) {
        let h = mix(pack, lcm);
        let mut i = h as usize & HMASK;
        loop {
            let eh = unsafe { *self.hash.get_unchecked(i) };
            if eh == 0 {
                unsafe {
                    *self.hash.get_unchecked_mut(i) = h;
                    *self.kpack.get_unchecked_mut(i) = pack;
                    *self.klcm.get_unchecked_mut(i) = lcm;
                    *self.kval.get_unchecked_mut(i) = val;
                }
                return;
            }
            if eh == h
                && unsafe { *self.kpack.get_unchecked(i) } == pack
                && unsafe { *self.klcm.get_unchecked(i) } == lcm
            {
                unsafe {
                    *self.kval.get_unchecked_mut(i) = val;
                }
                return;
            }
            i = (i + 1) & HMASK;
        }
    }

    fn sum_f2(&mut self, mut max_index: usize, mut min_k: usize, n: usize, mut lcm: u64) -> f64 {
        let mut outer = 1u64;

        // Canonicalize: drop primes that cannot appear in any remaining cycle
        // (min_k * p > n) and absorb their contribution into `outer`.
        loop {
            if n < 2 {
                let lf = lcm as f64;
                let of = outer as f64;
                return lf * lf * of * of * unsafe { *self.inv_ffact.get_unchecked(n) };
            }
            let p = unsafe { *self.primes.get_unchecked(max_index) } as usize;
            if min_k * p <= n {
                break;
            }
            let pu = p as u64;
            while lcm % pu == 0 {
                lcm /= pu;
                outer *= pu;
            }
            if max_index == 0 {
                let lf = lcm as f64;
                let of = outer as f64;
                return lf * lf * of * of * unsafe { *self.inv_ffact.get_unchecked(n) };
            }
            max_index -= 1;
            min_k = 1;
        }

        let outer2 = (outer as f64) * (outer as f64);
        let pack = pack_key(max_index, min_k, n);
        if let Some(v) = self.lookup(pack, lcm) {
            return v * outer2;
        }

        let orig_max = max_index;
        let mut relevant = lcm;
        let mut scale = 1u64;
        let lf = lcm as f64;
        let mut result = lf * lf * unsafe { *self.inv_ffact.get_unchecked(n) };

        let mut idx = max_index;
        loop {
            let p = unsafe { *self.primes.get_unchecked(idx) } as usize;
            let start_k = if idx == orig_max { min_k } else { 1 };
            let start_c = start_k * p;
            let scale2 = (scale as f64) * (scale as f64);

            let nlen = unsafe { self.by_lpf.get_unchecked(idx).len() };
            for li in 0..nlen {
                let c = unsafe { *self.by_lpf.get_unchecked(idx).get_unchecked(li) } as usize;
                if c < start_c {
                    continue;
                }
                if c > n {
                    break;
                }
                let new_lcm = lcm_u64(relevant, c as u64);
                let inv_c = 1.0 / c as f64;
                let mut inv_pow = inv_c;
                let mut remaining = n;
                let mut mult = 1usize;
                let k_next = c / p + 1;
                while remaining >= c {
                    remaining -= c;
                    let sub = self.sum_f2(idx, k_next, remaining, new_lcm);
                    result += sub
                        * scale2
                        * inv_pow
                        * unsafe { *self.inv_ffact.get_unchecked(mult) };
                    inv_pow *= inv_c;
                    mult += 1;
                }
            }

            let pu = p as u64;
            while relevant % pu == 0 {
                relevant /= pu;
                scale *= pu;
            }
            if idx == 0 {
                break;
            }
            idx -= 1;
        }

        self.store(pack, lcm, result);
        result * outer2
    }
}

fn main() {
    let mut is_prime = vec![true; N + 1];
    is_prime[0] = false;
    is_prime[1] = false;
    for i in 2..=N {
        if is_prime[i] {
            let mut j = i * i;
            while j <= N {
                is_prime[j] = false;
                j += i;
            }
        }
    }
    let primes: Vec<u32> = (2..=N).filter(|&i| is_prime[i]).map(|i| i as u32).collect();
    let nprimes = primes.len();

    let mut lpf = vec![0u32; N + 1];
    for &p in &primes {
        let mut i = p as usize;
        while i <= N {
            lpf[i] = p;
            i += p as usize;
        }
    }

    let mut inv_ffact = vec![0.0f64; N + 1];
    inv_ffact[0] = 1.0;
    let mut f = 1.0f64;
    for i in 1..=N {
        f *= i as f64;
        inv_ffact[i] = 1.0 / f;
    }

    let mut by_lpf = vec![Vec::new(); nprimes];
    let mut pidx = vec![0usize; N + 1];
    for (i, &p) in primes.iter().enumerate() {
        pidx[p as usize] = i;
    }
    for c in 2..=N {
        by_lpf[pidx[lpf[c] as usize]].push(c as u16);
    }

    let mut solver = Solver {
        primes,
        inv_ffact,
        by_lpf,
        hash: vec![0u64; HSIZE],
        kpack: vec![0u32; HSIZE],
        klcm: vec![0u64; HSIZE],
        kval: vec![0f64; HSIZE],
    };

    let ans = solver.sum_f2(nprimes - 1, 1, N, 1);

    let s = format!("{:.9e}", ans);
    let s: String = s.chars().filter(|&c| c != '+').collect();
    println!("{}", s);
}
