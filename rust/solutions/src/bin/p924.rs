// Problem 924 - Larger Digit Permutation II
//
// B(n) = smallest number larger than n formed by rearranging digits of n, or 0.
// a_0 = 0, a_n = a_{n-1}^2 + 2.
// U(N) = sum_{n=1..N} B(a_n).
// Compute U(10^16) mod 10^9+7.
//
// Strategy: U(N) = sum(a_n mod MOD) + sum(B(a_n) - a_n mod MOD).
// The delta (B(a_n) - a_n) for n >= 6 depends only on the last 10 digits of a_n,
// which cycle with period 8 * 5^8 = 3_125_000 modulo 10^10.
// For the single "bad" position per cycle (last 10 digits non-increasing),
// we look at 11 digits where the sub-cycle has period 5.

const MOD: u64 = 1_000_000_007;

/// Next lexicographic permutation of `digs` in-place. Returns true if advanced.
fn next_perm_inplace(digs: &mut [u8]) -> bool {
    let n = digs.len();
    if n < 2 {
        return false;
    }
    let mut i = n - 2;
    loop {
        if digs[i] < digs[i + 1] {
            break;
        }
        if i == 0 {
            return false;
        }
        i -= 1;
    }
    let mut j = n - 1;
    while digs[j] <= digs[i] {
        j -= 1;
    }
    digs.swap(i, j);
    digs[i + 1..].reverse();
    true
}

/// B(n): next digit permutation as a number, or 0.
fn big_b(n: u128) -> u128 {
    if n == 0 {
        return 0;
    }
    // Extract digits
    let mut digs = Vec::new();
    let mut tmp = n;
    while tmp > 0 {
        digs.push((tmp % 10) as u8);
        tmp /= 10;
    }
    digs.reverse();
    if !next_perm_inplace(&mut digs) {
        return 0;
    }
    let mut y: u128 = 0;
    for &d in &digs {
        y = y * 10 + d as u128;
    }
    y
}

/// Fixed-width next permutation: treat x as exactly k decimal digits (leading zeros allowed).
/// Returns Some(permuted_int) or None.
fn next_perm_fixed(x: u64, k: usize, buf: &mut [u8]) -> Option<u64> {
    let mut t = x;
    for i in (0..k).rev() {
        buf[i] = (t % 10) as u8;
        t /= 10;
    }

    // Find pivot
    if k < 2 {
        return None;
    }
    let mut i = k - 2;
    loop {
        if buf[i] < buf[i + 1] {
            break;
        }
        if i == 0 {
            return None;
        }
        i -= 1;
    }

    let mut j = k - 1;
    while buf[j] <= buf[i] {
        j -= 1;
    }
    buf.swap(i, j);
    buf[i + 1..k].reverse();

    let mut y: u64 = 0;
    for idx in 0..k {
        y = y * 10 + buf[idx] as u64;
    }
    Some(y)
}

/// Compute sum_{n=1..N} (a_n mod MOD) mod MOD using cycle detection.
fn sum_a_mod(big_n: u64) -> u64 {
    let mut x: u64 = 0;
    let mut states: Vec<u64> = vec![0]; // states[i] = a_i mod MOD
    let mut seen = std::collections::HashMap::new();
    seen.insert(0u64, 0usize);

    let (mu, lam);
    loop {
        let nxt = ((x as u128 * x as u128 + 2) % MOD as u128) as u64;
        let idx = states.len();
        if let Some(&prev) = seen.get(&nxt) {
            mu = prev;
            lam = idx - prev;
            break;
        }
        seen.insert(nxt, idx);
        states.push(nxt);
        x = nxt;
    }

    // Build prefix sums
    let mut pref: Vec<u64> = vec![0; states.len()];
    for i in 1..states.len() {
        pref[i] = (pref[i - 1] + states[i]) % MOD;
    }

    if big_n < states.len() as u64 {
        return pref[big_n as usize];
    }

    let base_before = if mu > 0 { pref[mu - 1] } else { 0 };
    let cycle_sum = (pref[mu + lam - 1] + MOD - base_before) % MOD;

    let cycle_terms = big_n - mu as u64 + 1;
    let full = cycle_terms / lam as u64;
    let rem = (cycle_terms % lam as u64) as usize;

    let mut total = (base_before + (full % MOD) * cycle_sum % MOD) % MOD;
    if rem > 0 {
        total = (total + (pref[mu + rem - 1] + MOD - base_before)) % MOD;
    }
    total
}

/// Exact contribution of delta_n = B(a_n) - a_n (mod MOD) for small n (1..=min(N,5)).
fn delta_small(big_n: u64) -> u64 {
    let mut a: u128 = 0;
    let mut s: u64 = 0;
    let limit = std::cmp::min(big_n, 5);
    for _ in 0..limit {
        a = a * a + 2;
        let b = big_b(a);
        // b - a could be negative if b==0, handle with MOD
        let delta = if b >= a {
            ((b - a) % MOD as u128) as u64
        } else {
            // b == 0, so delta = -a mod MOD
            (MOD - (a % MOD as u128) as u64) % MOD
        };
        s = (s + delta) % MOD;
    }
    s
}

/// For n >= 6, compute sum of deltas from the 10-digit cycle, plus identify the "bad" position.
/// Returns (sum_delta10, first_bad_n, step).
fn delta10_and_bad(big_n: u64) -> (u64, Option<u64>, u64) {
    if big_n <= 5 {
        return (0, None, 0);
    }

    let k: usize = 10;
    let m: u64 = 10_000_000_000; // 10^10
    // cycle length modulo 10^k = 8 * 5^(k-2) = 8 * 5^8 = 3_125_000
    let step: u64 = 8 * 390_625; // 8 * 5^8 = 3_125_000

    // Compute a_6 mod 10^10
    let mut x: u64 = 0;
    for _ in 0..6 {
        x = ((x as u128 * x as u128 + 2) % m as u128) as u64;
    }
    let start = x;

    let total_terms = big_n - 5; // n = 6..N
    let q = total_terms / step;
    let r = total_terms % step;

    let mut buf = [0u8; 10];
    let mut cycle_sum: u64 = 0;
    let mut rem_sum: u64 = 0;
    let mut bad_step: Option<u64> = None;

    for i in 1..=step {
        if let Some(y) = next_perm_fixed(x, k, &mut buf) {
            // delta = y - x, but y > x always when next_perm succeeds for fixed-width
            let d = if y >= x {
                (y - x) % MOD
            } else {
                // This shouldn't happen for fixed-width next perm, but be safe
                (MOD - (x - y) % MOD) % MOD
            };
            cycle_sum = (cycle_sum + d) % MOD;
            if i <= r {
                rem_sum = (rem_sum + d) % MOD;
            }
        } else {
            assert!(bad_step.is_none(), "More than one bad position in 10-digit cycle");
            bad_step = Some(i);
            // Contributes 0 to delta10
        }
        x = ((x as u128 * x as u128 + 2) % m as u128) as u64;
    }

    assert_eq!(x, start, "Must return to start after one cycle");
    assert!(bad_step.is_some(), "Expected one bad position");

    let first_bad_n = bad_step.unwrap() + 5; // i=1 corresponds to n=6
    let total = ((q % MOD) * cycle_sum % MOD + rem_sum) % MOD;
    (total, Some(first_bad_n), step)
}

/// Handle the "bad" indices where last 10 digits are non-increasing.
/// At these, the pivot is the 11th digit, so delta = next_perm(last11) - last11.
/// The bad indices form n = first_bad_n + t*step, and mod 10^11 the sub-cycle has period 5.
fn delta_bad_11(big_n: u64, first_bad_n: Option<u64>, step: u64) -> u64 {
    let first_bad_n = match first_bad_n {
        Some(f) if big_n >= f => f,
        _ => return 0,
    };

    let m11: u128 = 100_000_000_000; // 10^11
    let targets: Vec<u64> = (0..5).map(|t| first_bad_n + t * step).collect();
    let max_n = targets[4];

    // Simulate a_n mod 10^11 up to max_n
    let mut x: u128 = 0;
    let mut vals = [0u64; 5];
    let mut idx = 0;
    for n in 1..=max_n {
        x = (x * x + 2) % m11;
        if n == targets[idx] {
            vals[idx] = x as u64;
            idx += 1;
            if idx == 5 {
                break;
            }
        }
    }
    assert_eq!(idx, 5);

    let mut buf = [0u8; 11];
    let mut deltas = [0u64; 5];
    for i in 0..5 {
        let v = vals[i];
        let y = next_perm_fixed(v, 11, &mut buf)
            .expect("Expected 11-digit next perm to exist for bad positions");
        deltas[i] = if y >= v {
            (y - v) % MOD
        } else {
            (MOD - (v - y) % MOD) % MOD
        };
    }

    // Count how many bad indices <= N
    let t_count = 1 + (big_n - first_bad_n) / step;
    let base = t_count / 5;
    let rem = t_count % 5;

    let mut total: u64 = 0;
    for i in 0..5 {
        let c = base + if (i as u64) < rem { 1 } else { 0 };
        total = (total + (c % MOD) * deltas[i] % MOD) % MOD;
    }
    total
}

fn solve(big_n: u64) -> u64 {
    let s_a = sum_a_mod(big_n);
    let d_small = delta_small(big_n);
    let (d10, first_bad_n, step) = delta10_and_bad(big_n);
    let d_bad = delta_bad_11(big_n, first_bad_n, step);

    (s_a + d_small + d10 + d_bad) % MOD
}

fn main() {
    // Verification asserts
    debug_assert_eq!(big_b(245), 254);
    debug_assert_eq!(big_b(542), 0);

    println!("{}", solve(10_000_000_000_000_000));
}
