// Project Euler 411 - Uphill paths
// Compute S(k^5) for k=1..30, sum them.
// Stations at (2^i mod n, 3^i mod n); LIS of y after sorting by x.

use rayon::prelude::*;

fn gcd_fn(mut a: u64, mut b: u64) -> u64 {
    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }
    a
}

fn lcm_fn(a: u64, b: u64) -> u64 {
    a / gcd_fn(a, b) * b
}

// n = k^5 <= 30^5 < 2^32, so m*m fits in u64.
#[inline]
fn pow_mod(mut base: u64, mut exp: u64, m: u64) -> u64 {
    let mut result = 1u64;
    base %= m;
    while exp > 0 {
        if exp & 1 == 1 {
            result = result * base % m;
        }
        base = base * base % m;
        exp >>= 1;
    }
    result
}

fn mult_order(base: u64, modulus: u64) -> u64 {
    if modulus <= 1 {
        return 1;
    }
    if gcd_fn(base, modulus) != 1 {
        return 0;
    }

    let mut phi = modulus;
    let mut temp = modulus;
    let mut p = 2u64;
    while p * p <= temp {
        if temp % p == 0 {
            phi -= phi / p;
            while temp % p == 0 {
                temp /= p;
            }
        }
        p += 1;
    }
    if temp > 1 {
        phi -= phi / temp;
    }

    let mut result = phi;
    temp = phi;
    p = 2;
    while p * p <= temp {
        if temp % p == 0 {
            while temp % p == 0 {
                temp /= p;
            }
            while result % p == 0 && pow_mod(base, result / p, modulus) == 1 {
                result /= p;
            }
        }
        p += 1;
    }
    if temp > 1 {
        while result % temp == 0 && pow_mod(base, result / temp, modulus) == 1 {
            result /= temp;
        }
    }
    result
}

fn lis_y(stations: &[u64]) -> i64 {
    let mut tails: Vec<u32> = Vec::new();
    for &s in stations {
        let val = s as u32;
        if let Some(&last) = tails.last() {
            if last <= val {
                tails.push(val);
                continue;
            }
        } else {
            tails.push(val);
            continue;
        }
        let mut lo = 0usize;
        let mut hi = tails.len();
        while lo < hi {
            let mid = lo + ((hi - lo) >> 1);
            // SAFETY: lo < hi <= len ⇒ mid < len
            let t = unsafe { *tails.get_unchecked(mid) };
            if t <= val {
                lo = mid + 1;
            } else {
                hi = mid;
            }
        }
        // SAFETY: last > val ⇒ upper_bound is in 0..len
        unsafe {
            *tails.get_unchecked_mut(lo) = val;
        }
    }
    tails.len() as i64
}

fn s(n: u32) -> i64 {
    if n <= 1 {
        return 1;
    }
    let n64 = n as u64;

    let mut e_2 = 0u32;
    let mut n_2 = n;
    while n_2 % 2 == 0 {
        e_2 += 1;
        n_2 /= 2;
    }

    let mut e_3 = 0u32;
    let mut n_3 = n;
    while n_3 % 3 == 0 {
        e_3 += 1;
        n_3 /= 3;
    }

    let ord_2 = if n_2 > 1 { mult_order(2, n_2 as u64) } else { 1 };
    let ord_3 = if n_3 > 1 { mult_order(3, n_3 as u64) } else { 1 };
    let num_stations = (e_2.max(e_3) as u64 + lcm_fn(ord_2, ord_3)) as usize;

    let mut stations: Vec<u64> = Vec::with_capacity(num_stations);
    let ptr = stations.as_mut_ptr();
    let mut x = 1u32;
    let mut y = 1u32;
    for i in 0..num_stations {
        // SAFETY: i < capacity; every slot is written before set_len
        unsafe {
            ptr.add(i).write(((x as u64) << 32) | y as u64);
        }
        // 2x < 2n, 3y < 3n, both fit u64; n <= 30^5 so at most two subtracts for *3
        let t = (x as u64) << 1;
        x = (if t >= n64 { t - n64 } else { t }) as u32;
        let mut t = y as u64 * 3;
        if t >= n64 {
            t -= n64;
        }
        if t >= n64 {
            t -= n64;
        }
        y = t as u32;
    }
    // SAFETY: 0..num_stations written
    unsafe {
        stations.set_len(num_stations);
    }

    if stations.len() > 100_000 {
        stations.par_sort_unstable();
    } else {
        stations.sort_unstable();
    }
    stations.dedup();

    lis_y(&stations)
}

fn main() {
    let ans: i64 = (1..31)
        .into_par_iter()
        .rev()
        .with_max_len(1)
        .map(|k| s((k as u32).pow(5)))
        .sum();
    println!("{}", ans);
}
