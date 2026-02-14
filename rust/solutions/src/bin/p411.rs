// Project Euler 411 - Uphill paths
// Compute S(k^5) for k=1..30, sum them.
// Stations at (2^i mod n, 3^i mod n), find longest increasing subsequence.

fn gcd_fn(mut a: i64, mut b: i64) -> i64 {
    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }
    a
}

fn lcm_fn(a: i64, b: i64) -> i64 {
    a / gcd_fn(a, b) * b
}

fn pow_mod(mut base: u64, mut exp: u64, modulus: u64) -> u64 {
    let mut result = 1u64;
    base %= modulus;
    while exp > 0 {
        if exp & 1 == 1 {
            result = (result as u128 * base as u128 % modulus as u128) as u64;
        }
        base = (base as u128 * base as u128 % modulus as u128) as u64;
        exp >>= 1;
    }
    result
}

fn mult_order(base: i64, modulus: i64) -> i64 {
    if modulus <= 1 {
        return 1;
    }
    if gcd_fn(base, modulus) != 1 {
        return 0;
    }

    // Euler totient
    let mut phi = modulus;
    let mut temp = modulus;
    let mut p = 2i64;
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
            while result % p == 0
                && pow_mod(base as u64, (result / p) as u64, modulus as u64) == 1
            {
                result /= p;
            }
        }
        p += 1;
    }
    if temp > 1 {
        while result % temp == 0
            && pow_mod(base as u64, (result / temp) as u64, modulus as u64) == 1
        {
            result /= temp;
        }
    }
    result
}

fn lis(arr: &[i32]) -> i32 {
    if arr.is_empty() {
        return 0;
    }
    let mut tails: Vec<i32> = Vec::new();
    for &val in arr {
        // bisect_right: first position where tails[pos] > val
        let pos = tails.partition_point(|&t| t <= val);
        if pos == tails.len() {
            tails.push(val);
        } else {
            tails[pos] = val;
        }
    }
    tails.len() as i32
}

fn s(n: i64) -> i64 {
    if n <= 1 {
        return 1;
    }

    let mut e_2 = 0;
    let mut n_2 = n;
    while n_2 % 2 == 0 {
        e_2 += 1;
        n_2 /= 2;
    }

    let mut e_3 = 0;
    let mut n_3 = n;
    while n_3 % 3 == 0 {
        e_3 += 1;
        n_3 /= 3;
    }

    let ord_2 = if n_2 > 1 { mult_order(2, n_2) } else { 1 };
    let ord_3 = if n_3 > 1 { mult_order(3, n_3) } else { 1 };
    let num_stations = std::cmp::max(e_2, e_3) + lcm_fn(ord_2, ord_3);

    // Generate stations (x, y) encoded as u64
    let mut stations: Vec<u64> = Vec::with_capacity(num_stations as usize);
    let (mut x, mut y) = (1u64 % n as u64, 1u64 % n as u64);
    for _ in 0..num_stations {
        stations.push((x << 32) | y);
        x = (x as u128 * 2 % n as u128) as u64;
        y = (y as u128 * 3 % n as u128) as u64;
    }

    // Remove duplicates
    stations.sort_unstable();
    stations.dedup();

    // Extract y-coordinates
    let y_coords: Vec<i32> = stations.iter().map(|&s| (s & 0xFFFFFFFF) as i32).collect();

    lis(&y_coords) as i64
}

fn main() {
    let (n_val, k_val) = (30, 5);
    let mut ans: i64 = 0;
    for k in 1..=n_val {
        let mut n = 1i64;
        for _ in 0..k_val {
            n *= k;
        }
        ans += s(n);
    }
    println!("{}", ans);
}
