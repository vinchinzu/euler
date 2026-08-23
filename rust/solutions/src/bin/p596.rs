// Project Euler 596 - Number of Lattice Points in a Ball
//
// Find the number of integer quadruples (x,y,z,t) with x^2+y^2+z^2+t^2 <= N^2.
// Uses Jacobi's four square theorem.
//
// sigma2(n) = sum_{k=1}^n k * floor(n/k) = sum_{d=1}^n sigma(d)  (mod MOD)
// Hyperbola identity (one O(sqrt n) loop):
//   sum_{i=1}^s (i * floor(n/i) + tri(floor(n/i))) - s * tri(s)

const MOD: u64 = 1_000_000_007;

/// n_mod = n % MOD, 0 <= n_mod < MOD. Returns n*(n+1)/2 % MOD.
#[inline(always)]
fn tri_from_mod(n_mod: u64) -> u64 {
    let prod = if n_mod & 1 == 0 {
        (n_mod >> 1) * (n_mod + 1)
    } else {
        n_mod * ((n_mod + 1) >> 1)
    };
    prod % MOD
}

/// sum_{k=1}^n k * floor(n/k)  mod MOD
fn sigma2(n: u64) -> u64 {
    let s = n.isqrt();
    let mut acc = 0u64;
    let mut i = 1u64;

    // i < MOD (s = 1e8), so i * (n/i % MOD) fits in u64. Reduce every 64 steps.
    while i + 3 <= s {
        let r0 = (n / i) % MOD;
        let r1 = (n / (i + 1)) % MOD;
        let r2 = (n / (i + 2)) % MOD;
        let r3 = (n / (i + 3)) % MOD;
        acc = acc
            .wrapping_add(i * r0)
            .wrapping_add(tri_from_mod(r0))
            .wrapping_add((i + 1) * r1)
            .wrapping_add(tri_from_mod(r1))
            .wrapping_add((i + 2) * r2)
            .wrapping_add(tri_from_mod(r2))
            .wrapping_add((i + 3) * r3)
            .wrapping_add(tri_from_mod(r3));
        i += 4;
        if i & 63 == 1 {
            acc %= MOD;
        }
    }
    while i <= s {
        let r = (n / i) % MOD;
        acc = acc.wrapping_add(i * r).wrapping_add(tri_from_mod(r));
        i += 1;
    }
    acc %= MOD;

    // s < MOD, so s * tri(s) % MOD = (s % MOD) * tri_from_mod(s % MOD) % MOD
    let sub = (s % MOD) * tri_from_mod(s % MOD) % MOD;
    if acc >= sub { acc - sub } else { acc + MOD - sub }
}

fn main() {
    let n: u64 = 100_000_000; // 10^8
    let n_sq = n * n; // 10^16

    let (s1, s2) = rayon::join(|| sigma2(n_sq), || sigma2(n_sq / 4));

    let ans = (1 + 8 * s1 + 2 * MOD - 32 * s2 % MOD) % MOD;
    println!("{}", ans);
}
