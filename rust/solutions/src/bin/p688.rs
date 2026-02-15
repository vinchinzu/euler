// Project Euler 688 - Piles of Plates
// Iterate k, compute contribution using closed forms.

const MOD: i64 = 1_000_000_007;

fn main() {
    let big_n: i64 = 10_000_000_000_000_000;
    let inv2 = (MOD + 1) / 2;
    let mut n = big_n;
    let mut ans = 0i64;
    let mut k = 1i64;

    while n > 0 {
        let limit = (n / k) % MOD;
        let term1 = (k % MOD * limit % MOD * ((limit - 1 + MOD) % MOD) % MOD) as i128
            * inv2 as i128 % MOD as i128;
        let nk = n % k;
        let term2 = ((nk + 1) % MOD) as i128 * limit as i128 % MOD as i128;
        ans = ((ans as i128 + term1 + term2) % MOD as i128) as i64;
        n -= k;
        k += 1;
    }

    println!("{}", ans);
}
