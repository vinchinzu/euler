// Project Euler 684 - Inverse Digit Sum
// s(n) = smallest number with digit sum n, S(k) = sum s(1..k).
// Answer = sum S(fib(i)) for i=2..90, mod 10^9+7.

const MOD: u64 = 1_000_000_007;

#[inline(always)]
fn power_mod(mut base: u64, mut exp: u64) -> u64 {
    let mut result = 1u64;
    base %= MOD;
    while exp != 0 {
        if exp & 1 != 0 {
            result = result * base % MOD;
        }
        base = base * base % MOD;
        exp >>= 1;
    }
    result
}

fn main() {
    let mut fib_prev = 1u64;
    let mut fib_curr = 1u64;
    let mut ans = 0u64;
    
    for _ in 0..89 {
        let n = fib_curr;
        let r = n % 9;
        let q = n / 9;
        
        let pw = power_mod(10, q);
        let coeff = 6 + r + (r * (r + 1) >> 1);
        let term = coeff * pw % MOD;
        ans = (ans + term + MOD - (6 + n % MOD)) % MOD;
        
        let next = fib_prev + fib_curr;
        fib_prev = fib_curr;
        fib_curr = next;
    }

    println!("{}", ans);
}
