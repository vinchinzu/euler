// Problem 889 - Rational Blancmange
//
// Compute F(k, t, r) = (2^{2k} - 1) * T( (2^t + 1)^r / (2^k + 1) )
// where T(x) = sum_{n>=0} s(2^n x)/2^n and s(y) = distance from y to nearest integer.
//
// Answer: F(10^18 + 31, 10^14 + 31, 62) mod 1_000_062_031

const MOD: u64 = 1_000_062_031;

/// Fast modular exponentiation. MOD < 2^30 so u64 multiplication is safe.
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

/// Compute bit-length of C(r, r/2) to verify the sparse expansion is valid.
/// We compute C(62, 31) bit-length. C(62,31) is roughly 9.16e17, which is about 60 bits.
/// Since t = 10^14 + 31 >> 60, the blocks don't overlap.
fn max_coeff_bitlen(r: u64) -> u64 {
    // Compute C(r, r/2) and find its bit length
    // For r=62, C(62,31) fits in u128 (it's about 9.16e17 ~ 60 bits)
    let half = r / 2;
    let c = comb_u128(r, half);
    128 - c.leading_zeros() as u64
}

/// Compute C(n, k) as u128 (for small n like 62, this is fine)
fn comb_u128(n: u64, k: u64) -> u128 {
    if k > n {
        return 0;
    }
    let k = k.min(n - k) as u128;
    let n = n as u128;
    let mut result = 1u128;
    for i in 0..k {
        result = result * (n - i) / (i + 1);
    }
    result
}

/// Bit positions of N = (2^t + 1)^r without building N.
/// Uses binomial expansion: (2^t + 1)^r = sum_{u=0..r} C(r,u) * 2^{t*u}.
fn bit_positions_of_n_sparse(t: u64, r: u64) -> Vec<u64> {
    let mcb = max_coeff_bitlen(r);
    assert!(
        t > mcb,
        "t too small for sparse expansion without carries"
    );

    let mut positions: Vec<u64> = Vec::new();
    for u in 0..=r {
        let c = comb_u128(r, u);
        let base = t * u;
        // Get set bits of c
        let mut cv = c;
        while cv > 0 {
            let lsb = cv & cv.wrapping_neg();
            let b = lsb.trailing_zeros() as u64;
            positions.push(base + b);
            cv -= lsb;
        }
    }

    positions.sort_unstable();

    // Sanity check: no duplicate positions
    for i in 1..positions.len() {
        assert_ne!(
            positions[i], positions[i - 1],
            "duplicate bit position produced"
        );
    }

    positions
}

/// Brute computation of F(k,t,r) mod m.
/// Uses the identity: F = sum_{j=0..k-1} d_j * 2^{k-j}
/// where d_j = min(m_j, Q - m_j) and m_j = (2^j * N) mod Q, Q = 2^k + 1.
fn brute_f_mod(k: u64, t: u64, r: u64, m: u64) -> u64 {
    let q = (1u128 << k) + 1;
    let n_mod = {
        let base = ((1u128 << t) + 1) % q;
        // pow(base, r, q) using u128
        let mut result = 1u128;
        let mut b = base;
        let mut e = r;
        while e > 0 {
            if e & 1 == 1 {
                result = result * b % q;
            }
            b = b * b % q;
            e >>= 1;
        }
        result
    };

    let inv2 = pow_mod(2, m - 2, m); // mod inverse of 2
    let mut weight = pow_mod(2, k, m); // 2^{k-j} starting at j=0

    let mut mj = n_mod;
    let mut ans = 0u64;
    for _j in 0..k {
        let d = if mj <= q - mj { mj } else { q - mj };
        let d_mod = (d % m as u128) as u64;
        ans = (ans + d_mod * weight % m) % m;
        weight = weight * inv2 % m;
        mj = mj * 2 % q;
    }

    ans
}

/// Fast computation for the large instance.
/// Requires that k is larger than the highest set bit of N = (2^t+1)^r.
fn fast_f_mod(k: u64, t: u64, r: u64, m: u64) -> u64 {
    let positions = bit_positions_of_n_sparse(t, r);
    let max_pos = *positions.last().unwrap();
    assert!(
        k > max_pos + 1,
        "fast solver requires k > bit_length(N)"
    );

    let pow2_k = pow_mod(2, k, m);

    // Precompute 2^p (mod m) for each set bit position p
    let vals_low: Vec<u64> = positions.iter().map(|&p| pow_mod(2, p, m)).collect();
    // 2^{k+p} = 2^k * 2^p
    let vals_high: Vec<u64> = vals_low.iter().map(|&v| pow2_k * v % m).collect();

    let n = positions.len();
    let mut prefix_low = vec![0u64; n + 1];
    let mut prefix_high = vec![0u64; n + 1];
    for i in 0..n {
        prefix_low[i + 1] = (prefix_low[i] + vals_low[i]) % m;
        prefix_high[i + 1] = (prefix_high[i] + vals_high[i]) % m;
    }

    let total_low = prefix_low[n];

    // Base sum assuming d_j = (B_j - C_j) for all j
    // Contribution per bit p: (k-p)*2^{k+p} - p*2^p
    let mut ans = 0u64;
    for i in 0..n {
        let p = positions[i];
        let v_low = vals_low[i];
        let v_high = vals_high[i];
        // (k - p) mod m  -- k and p are both positive, k > p
        let k_minus_p = (k - p) % m;
        let term_high = k_minus_p * (v_high % m) % m;
        let p_mod = p % m;
        let term_low = p_mod * (v_low % m) % m;
        // ans += term_high - term_low (mod m)
        ans = (ans + term_high + m - term_low) % m;
    }

    // Corrections: for j = k-p0-1 (p0>0), the leading bit forces d_j = Q - (B_j-C_j)
    for i in 1..n {
        let _p0 = positions[i];
        // bits <= p0 are in B; bits > p0 are in C
        let sum_high_le = prefix_high[i + 1];
        let sum_low_gt = (total_low + m - prefix_low[i + 1]) % m;
        let s = (sum_high_le + m - sum_low_gt) % m;

        let pow2_p0_plus1 = vals_low[i] * 2 % m; // 2^{p0+1}
        // (2^k + 1) * 2^{p0+1}
        let q_times = (pow2_k * pow2_p0_plus1 % m + pow2_p0_plus1) % m;

        let delta = (q_times + m - 2 * s % m) % m;
        ans = (ans + delta) % m;
    }

    ans % m
}

/// Compute F(k,t,r) modulo m.
fn f_mod(k: u64, t: u64, r: u64, m: u64) -> u64 {
    if k <= 1500 && t <= 2000 {
        brute_f_mod(k, t, r, m)
    } else {
        fast_f_mod(k, t, r, m)
    }
}

fn main() {
    // Validate against test cases from the problem statement
    debug_assert_eq!(f_mod(3, 1, 1, MOD), 42);
    debug_assert_eq!(f_mod(13, 3, 3, MOD), 23_093_880);
    debug_assert_eq!(f_mod(103, 13, 6, MOD), 878_922_518);

    let k = 1_000_000_000_000_000_000u64 + 31;
    let t = 100_000_000_000_000u64 + 31;
    let r = 62;

    println!("{}", f_mod(k, t, r, MOD));
}
