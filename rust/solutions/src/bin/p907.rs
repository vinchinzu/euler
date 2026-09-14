// Problem 907: Stacking Cups
//
// S(n) = number of ways to build a single tower using all n cups.
// S(4)=12, S(8)=58, S(20)=5560. Find S(10^7) mod 10^9+7.
//
// Linear recurrence (verified for n >= 10):
// S(n) = 2*S(n-1) - 3*S(n-2) + 5*S(n-3) - 4*S(n-4) + 4*S(n-5) - 3*S(n-6) + S(n-7) - S(n-8)
//
// Characteristic polynomial: (x-1)(x^2+1)^2(x^3-x^2-1)
//
// Optimization: Use matrix exponentiation to compute S(n) in O(k^3 log n) instead of O(n).

use euler_utils::ModMatrix;

fn main() {
    const MOD: u64 = 1_000_000_007;
    const N: usize = 10_000_000;

    let base: [u64; 10] = [0, 2, 2, 6, 12, 16, 22, 36, 58, 82];

    if N <= 9 {
        println!("{}", base[N] % MOD);
        return;
    }

    let m = MOD;
    
    let matrix_data = [
        [2, m - 3, 5, m - 4, 4, m - 3, 1, m - 1],
        [1, 0, 0, 0, 0, 0, 0, 0],
        [0, 1, 0, 0, 0, 0, 0, 0],
        [0, 0, 1, 0, 0, 0, 0, 0],
        [0, 0, 0, 1, 0, 0, 0, 0],
        [0, 0, 0, 0, 1, 0, 0, 0],
        [0, 0, 0, 0, 0, 1, 0, 0],
        [0, 0, 0, 0, 0, 0, 1, 0],
    ];
    
    let mat = ModMatrix::<8>::from_data(matrix_data, MOD);
    
    let initial_state = [
        base[9] % MOD,
        base[8] % MOD,
        base[7] % MOD,
        base[6] % MOD,
        base[5] % MOD,
        base[4] % MOD,
        base[3] % MOD,
        base[2] % MOD,
    ];
    
    let result = mat.pow((N - 9) as u64).mul_vec(&initial_state);
    
    println!("{}", result[0]);
}
