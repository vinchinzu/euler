// Project Euler 464 - Mobius function and balanced pairs
//
// Count pairs 1 <= a <= b <= N where the counts of mu(n)=1 and mu(n)=-1
// in [a,b] are approximately balanced.

const NN: usize = 20_000_000;
const K: i32 = 100;

#[inline(always)]
fn bit_add(bit: &mut [i32], mut idx: usize, n: usize) {
    idx += 1;
    while idx <= n {
        // SAFETY: idx in 1..=n and bit.len() >= n + 1
        unsafe {
            *bit.get_unchecked_mut(idx) += 1;
        }
        idx += idx & idx.wrapping_neg();
    }
}

#[inline(always)]
fn bit_sum(bit: &[i32], mut idx: usize) -> i32 {
    idx += 1;
    let mut s = 0i32;
    while idx > 0 {
        // SAFETY: idx starts at a 1-based in-range index and strictly decreases
        unsafe {
            s += *bit.get_unchecked(idx);
        }
        idx -= idx & idx.wrapping_neg();
    }
    s
}

fn mobius(n: usize) -> Vec<i8> {
    let mut mu = vec![0i8; n + 1];
    let mut spf = vec![0u32; n + 1];
    let mut primes = Vec::with_capacity(1_280_000);
    mu[1] = 1;
    for i in 2..=n {
        if spf[i] == 0 {
            spf[i] = i as u32;
            primes.push(i as u32);
            mu[i] = -1;
        }
        let spi = unsafe { *spf.get_unchecked(i) };
        for &p in &primes {
            if p > spi || p as usize > n / i {
                break;
            }
            let v = i * (p as usize);
            unsafe {
                *spf.get_unchecked_mut(v) = p;
                *mu.get_unchecked_mut(v) = if p == spi {
                    0
                } else {
                    -*mu.get_unchecked(i)
                };
            }
        }
    }
    mu
}

/// One Fenwick pass: subtract intervals that violate the `sign` inequality.
/// `delta[mu+1]` is the walk step for this sign. Counts fit in i32 (NN = 2e7).
fn fenwick_violations(mu: &[i8], delta: [i32; 3], l: i32, tree_size: usize) -> i64 {
    let mut bit = vec![0i32; tree_size + 2];
    let mut f = 0i32;
    let mut sub = 0i64;
    for b in 1..=NN {
        bit_add(&mut bit, (f + l) as usize, tree_size);
        // SAFETY: mu.len() == NN + 1
        let m = unsafe { *mu.get_unchecked(b) };
        f += delta[(m + 1) as usize];
        // Insertions are in-range, so the tree total equals b.
        sub += b as i64 - bit_sum(&bit, (f + l) as usize) as i64;
    }
    sub
}

fn main() {
    let mu = mobius(NN);
    let l = K * (NN as f64).sqrt() as i32;
    let tree_size = NN + l as usize + 2;
    let ans0 = (NN as i64) * (NN as i64 + 1) / 2;
    // sign = +1: mu=1 => +K, mu=-1 => -(K-1); sign = -1 swaps those.
    let d_pos = [-(K - 1), 0, K];
    let d_neg = [K, 0, -(K - 1)];
    let (s1, s2) = rayon::join(
        || fenwick_violations(&mu, d_pos, l, tree_size),
        || fenwick_violations(&mu, d_neg, l, tree_size),
    );
    println!("{}", ans0 - s1 - s2);
}
