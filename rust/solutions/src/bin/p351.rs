// Project Euler 351: Hexagonal Orchards
// Hidden = 3*n*(n+1) - 6*sum(phi(k) for k=1..n), n=10^8.

const LIMIT: usize = 100_000_000;

fn main() {
    let n = LIMIT;

    // Allocate phi array
    let mut phi = vec![0u32; n + 1];
    for i in 0..=n {
        phi[i] = i as u32;
    }

    // Sieve to compute totient
    for p in 2..=n {
        if phi[p] == p as u32 {
            // p is prime
            phi[p] = (p - 1) as u32;
            let mut j = 2 * p;
            while j <= n {
                phi[j] -= phi[j] / p as u32;
                j += p;
            }
        }
    }

    // Sum phi
    let totient_sum: i64 = phi[1..=n].iter().map(|&x| x as i64).sum();

    let hidden = 3i64 * n as i64 * (n as i64 + 1) - 6 * totient_sum;
    println!("{hidden}");
}
