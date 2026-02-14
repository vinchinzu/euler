// Project Euler 596 - Number of Lattice Points in a Ball
//
// Find the number of integer quadruples (x,y,z,t) with x^2+y^2+z^2+t^2 <= N^2.
// Uses Jacobi's four square theorem.

const MOD: i64 = 1_000_000_007;

fn sigma2(n: i64, modulus: i64) -> i64 {
    let mut val: i64 = 0;
    let sq = (n as f64).sqrt() as i64;
    // Adjust for precision
    let sq = {
        let mut s = sq;
        while s * s > n { s -= 1; }
        while (s + 1) * (s + 1) <= n { s += 1; }
        s
    };

    for k in 1..=sq {
        val = (val + ((n / k) % modulus) * (k % modulus)) % modulus;
    }

    for t in 1..sq {
        let low = n / (t + 1) + 1;
        let high = n / t;
        let count = high - low + 1;
        let sum_range = if (low + high) % 2 == 0 {
            (((low + high) / 2) % modulus) * (count % modulus) % modulus
        } else {
            ((low + high) % modulus) * ((count / 2) % modulus) % modulus
        };
        val = (val + sum_range * (t % modulus)) % modulus;
    }

    ((val % modulus) + modulus) % modulus
}

fn main() {
    let n: i64 = 100_000_000; // 10^8
    let n_sq: i64 = n * n; // 10^16

    let s1 = sigma2(n_sq, MOD);
    let s2 = sigma2(n_sq / 4, MOD);

    let ans = ((1 + 8 * s1 % MOD - 32 * s2 % MOD + 2 * MOD) % MOD + MOD) % MOD;

    println!("{}", ans);
}
