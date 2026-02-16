// Project Euler 786 - Billiard Ball Bounces
// Mobius function sieve and lattice point counting.
// Optimized: linear Mobius sieve + closed-form inner sum.

fn main() {
    const BIG_N: i64 = 1_000_000_000;
    let l = (3 * BIG_N + 5) / 2;
    let g_limit = (l / 5) as usize;

    // Compute Mobius function using linear sieve (O(n))
    // Each number is visited exactly once via its smallest prime factor.
    let mut mobius = vec![0i8; g_limit + 1];
    let mut smallest_prime = vec![0u32; g_limit + 1];
    let mut primes: Vec<usize> = Vec::with_capacity(g_limit / 10);

    mobius[1] = 1;

    for i in 2..=g_limit {
        if unsafe { *smallest_prime.get_unchecked(i) } == 0 {
            // i is prime
            unsafe { *smallest_prime.get_unchecked_mut(i) = i as u32; }
            primes.push(i);
            unsafe { *mobius.get_unchecked_mut(i) = -1; }
        }
        let mi = unsafe { *mobius.get_unchecked(i) };
        for &p in &primes {
            let ip = i * p;
            if ip > g_limit { break; }
            unsafe { *smallest_prime.get_unchecked_mut(ip) = p as u32; }
            if i % p == 0 {
                // p^2 divides ip, so mobius[ip] = 0 (already initialized to 0)
                break;
            } else {
                unsafe { *mobius.get_unchecked_mut(ip) = -mi; }
            }
        }
    }

    drop(smallest_prime);
    drop(primes);

    // Closed-form lattice point counting
    // sum_{x=1}^{N} floor((t - 5x) / d) where N = (t - d) / 5
    // = (sum_y - sum_residues) / d
    // sum_y = N*t - 5*N*(N+1)/2
    // sum_residues uses periodicity of (t - 5x) mod d with period d (gcd(5,d)=1)
    #[inline(always)]
    fn lattice_count(t: i64, d: i64) -> i64 {
        if t < d + 5 { return 0; }
        let n = (t - d) / 5;
        if n <= 0 { return 0; }

        let sum_y: i128 = n as i128 * t as i128 - 5 * n as i128 * (n as i128 + 1) / 2;

        let q = n / d;
        let r = n % d;
        let cycle_sum = d * (d - 1) / 2;
        let mut partial_sum: i64 = 0;
        for j in 1..=r {
            partial_sum += ((t - 5 * j) % d + d) % d;
        }
        let sum_mod: i128 = q as i128 * cycle_sum as i128 + partial_sum as i128;

        ((sum_y - sum_mod) / d as i128) as i64
    }

    let mut ans: i64 = 0;

    for g in 1..=g_limit {
        // SAFETY: g is in bounds [1, g_limit], mobius has size g_limit+1
        let m = unsafe { *mobius.get_unchecked(g) };
        if m == 0 { continue; }
        let t = l / g as i64;
        let d: i64 = if g % 3 == 0 { 3 } else { 9 };

        let count = lattice_count(t, d);
        ans += m as i64 * count;
    }

    ans *= 4;
    ans += 2;

    println!("{}", ans);
}
