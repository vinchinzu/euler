// Problem 950 - Pirate Treasure
//
// Compute sum_{k=1..6} T(10^16, 10^k+1, 1/sqrt(10^k+1))
// and print the last 9 digits.
//
// All arithmetic on p = 1/sqrt(D) is done with exact integer operations.
// The running total is kept mod 10^9 to avoid overflow.

const MOD: u64 = 1_000_000_000;

/// Integer square root of a u128 value using Newton's method.
fn isqrt_u128(n: u128) -> u128 {
    if n <= 1 {
        return n;
    }
    let bits = 128 - n.leading_zeros();
    let mut x = 1u128 << ((bits + 1) / 2);
    loop {
        let x1 = (x + n / x) / 2;
        if x1 >= x {
            break;
        }
        x = x1;
    }
    // Correct off-by-one.
    while x * x > n {
        x -= 1;
    }
    x
}

/// Return floor(d / sqrt(big_d)) for integers d >= 0, big_d >= 1.
/// Uses isqrt(d^2 / big_d) as initial guess, then corrects.
fn floor_div_sqrt(d: u64, big_d: u64) -> u64 {
    if d == 0 {
        return 0;
    }
    let dd = d as u128 * d as u128;
    let big_d128 = big_d as u128;
    let mut t = isqrt_u128(dd / big_d128);
    // Correct potential off-by-one errors.
    while (t + 1) * (t + 1) * big_d128 <= dd {
        t += 1;
    }
    while t * t * big_d128 > dd {
        t -= 1;
    }
    t as u64
}

/// Return ceil(d / sqrt(big_d)) for d >= 0, non-square big_d.
/// Since sqrt(big_d) is irrational, d/sqrt(big_d) is never integer for d > 0.
fn ceil_div_sqrt(d: u64, big_d: u64) -> u64 {
    if d == 0 {
        return 0;
    }
    floor_div_sqrt(d, big_d) + 1
}

/// Compute n*(n+1)/2 mod MOD without overflow.
/// Since n can be up to ~10^16, we use the identity:
///   If n is even: (n/2) * (n+1) mod MOD
///   If n is odd:  n * ((n+1)/2) mod MOD
/// Each factor mod MOD fits in u64, and their product fits in u64 too (< 10^18).
fn tri_mod(n: u64) -> u64 {
    if n % 2 == 0 {
        ((n / 2) % MOD) * ((n + 1) % MOD) % MOD
    } else {
        (n % MOD) * (((n + 1) / 2) % MOD) % MOD
    }
}

/// For p < 1, sum_{n=1..min(N,2C+2)} c(n) mod MOD.
/// c(n) = max(C - floor((n-1)/2), 0) for n = 1..2C+2.
fn initial_prefix_sum_mod(n: u64, c: u64) -> u64 {
    if n == 0 || c == 0 {
        return 0;
    }
    let limit = 2 * c + 2;
    let mut m = n.min(limit);
    m = m.min(2 * c);
    if m == 0 {
        return 0;
    }

    let half = m / 2; // number of full pairs
    // s = 2 * (half * c - half*(half-1)/2)
    // Use u128 to compute this mod MOD safely.
    let hc = (half as u128) * (c as u128); // up to ~10^6 * 10^6 = 10^12, fits u64 too
    let tri = (half as u128) * ((half - 1) as u128) / 2; // half*(half-1)/2
    let inner = (hc - tri) % MOD as u128;
    let mut s = (2 * inner) % MOD as u128;
    if m % 2 == 1 {
        s = (s + (c - half) as u128) % MOD as u128;
    }
    s as u64
}

/// Exact version for small test cases (no mod reduction).
fn initial_prefix_sum_exact(n: u64, c: u64) -> u64 {
    if n == 0 || c == 0 {
        return 0;
    }
    let limit = 2 * c + 2;
    let mut m = n.min(limit);
    m = m.min(2 * c);
    if m == 0 {
        return 0;
    }
    let half = m / 2;
    let mut s = 2 * (half * c - half * (half - 1) / 2);
    if m % 2 == 1 {
        s += c - half;
    }
    s
}

/// Find next position x > l where the most senior pirate survives.
fn next_reset(l: u64, c: u64, big_d: u64) -> u64 {
    if c == 0 {
        return 2 * l;
    }

    let mut t: u64 = 1;
    while t <= c {
        let y = c / t;
        // x = 2*l - 2*y
        // Since l >= 2*c+2 and y <= c, x = 2*l - 2*y >= 2*(2c+2) - 2c = 2c+4 > 0.
        // But we also need d = x - l = l - 2*y > 0, i.e., l > 2*y.
        if l > 2 * y {
            let x = 2 * l - 2 * y;
            let d = x - l; // = l - 2*y
            let s = ceil_div_sqrt(d, big_d);
            if c / s == y {
                return x;
            }
        }
        // Jump to next t that changes c//t.
        t = (c / y) + 1;
    }

    // y = 0 case (k > C)
    2 * l
}

/// Compute T(N, C, 1/sqrt(D)) mod MOD.
fn t_func(n: u64, c: u64, big_d: u64) -> u64 {
    if n == 0 {
        return 0;
    }

    let start_reset = 2 * c + 2;
    if n <= start_reset {
        return initial_prefix_sum_mod(n, c);
    }

    let mut total = initial_prefix_sum_mod(start_reset, c);
    let mut l = start_reset;
    let mut c_l: u64 = 0; // at n=2C+2, c=0

    while l < n {
        let x = next_reset(l, c, big_d);
        if x > n {
            // Truncated final cascade: n = L+1..N
            // cnt = N - L terms, each contributing (cL + k) for k=1..cnt
            // sum = cnt * cL + cnt*(cnt+1)/2
            let cnt = n - l;
            let contrib = add_cascade_mod(cnt, c_l);
            total = (total + contrib) % MOD;
            break;
        }

        let d = x - l;
        if d > 1 {
            // n = L+1..x-1: cnt = d-1 terms contributing (cL+1)..(cL+d-1)
            // sum = (d-1)*cL + (d-1)*d/2
            let cnt = d - 1;
            let contrib = add_cascade_mod(cnt, c_l);
            total = (total + contrib) % MOD;
        }

        // At n=x, compute c(x).
        let required_votes = (x + 1) / 2; // ceil(x/2)
        let free_votes = d;
        let need_bribes = if required_votes > free_votes {
            required_votes - free_votes
        } else {
            0
        };

        let s = ceil_div_sqrt(d, big_d);
        let cost = need_bribes * s;
        c_l = c - cost;

        total = (total + c_l % MOD) % MOD;
        l = x;
    }

    total
}

/// Compute (cnt * c_l + cnt*(cnt+1)/2) mod MOD.
/// cnt can be up to ~10^16, c_l up to ~10^6.
fn add_cascade_mod(cnt: u64, c_l: u64) -> u64 {
    // cnt * c_l mod MOD: use u128
    let part1 = ((cnt as u128) * (c_l as u128)) % MOD as u128;
    // cnt*(cnt+1)/2 mod MOD
    let part2 = tri_mod(cnt) as u128;
    ((part1 + part2) % MOD as u128) as u64
}

/// Exact T for small test cases.
fn t_func_exact(n: u64, c: u64, big_d: u64) -> u64 {
    if n == 0 {
        return 0;
    }

    let start_reset = 2 * c + 2;
    if n <= start_reset {
        return initial_prefix_sum_exact(n, c);
    }

    let mut total = initial_prefix_sum_exact(start_reset, c);
    let mut l = start_reset;
    let mut c_l: u64 = 0;

    while l < n {
        let x = next_reset(l, c, big_d);
        if x > n {
            let cnt = n - l;
            total += cnt * c_l + cnt * (cnt + 1) / 2;
            break;
        }

        let d = x - l;
        if d > 1 {
            let cnt = d - 1;
            total += cnt * c_l + cnt * (cnt + 1) / 2;
        }

        let required_votes = (x + 1) / 2;
        let free_votes = d;
        let need_bribes = if required_votes > free_votes {
            required_votes - free_votes
        } else {
            0
        };

        let s = ceil_div_sqrt(d, big_d);
        let cost = need_bribes * s;
        c_l = c - cost;
        total += c_l;
        l = x;
    }

    total
}

fn main() {
    // Verify against known test cases.
    debug_assert_eq!(t_func_exact(30, 3, 3), 190);
    debug_assert_eq!(t_func_exact(50, 3, 31), 385);
    debug_assert_eq!(t_func_exact(1000, 101, 101), 142427);

    let n: u64 = 10_u64.pow(16);
    let mut acc: u64 = 0;
    for k in 1..=6_u32 {
        let c = 10_u64.pow(k) + 1;
        acc = (acc + t_func(n, c, c)) % MOD;
    }
    println!("{:09}", acc);
}
