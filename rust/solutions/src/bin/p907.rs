// Problem 907: Stacking Cups
//
// S(n) = number of ways to build a single tower using all n cups.
// S(4)=12, S(8)=58, S(20)=5560. Find S(10^7) mod 10^9+7.
//
// Linear recurrence (verified for n >= 10):
// S(n) = 2*S(n-1) - 3*S(n-2) + 5*S(n-3) - 4*S(n-4) + 4*S(n-5) - 3*S(n-6) + S(n-7) - S(n-8)
//
// Characteristic polynomial: (x-1)(x^2+1)^2(x^3-x^2-1)

fn main() {
    const MOD: u64 = 1_000_000_007;
    const N: usize = 10_000_000;

    // Base cases S(1) through S(9)
    let base: [u64; 10] = [0, 2, 2, 6, 12, 16, 22, 36, 58, 82];

    if N <= 9 {
        println!("{}", base[N] % MOD);
        return;
    }

    // Recurrence coefficients: S(n) = c1*S(n-1) + c2*S(n-2) + ... + c8*S(n-8)
    // c = [2, -3, 5, -4, 4, -3, 1, -1]
    // Using modular arithmetic (add MOD to handle negative coefficients)
    let coeffs: [(u64, bool); 8] = [
        (2, true),   // +2
        (3, false),  // -3
        (5, true),   // +5
        (4, false),  // -4
        (4, true),   // +4
        (3, false),  // -3
        (1, true),   // +1
        (1, false),  // -1
    ];

    // Sliding window of last 8 values
    let mut window: [u64; 8] = [
        base[2] % MOD, // s[2]
        base[3] % MOD, // s[3]
        base[4] % MOD, // s[4]
        base[5] % MOD, // s[5]
        base[6] % MOD, // s[6]
        base[7] % MOD, // s[7]
        base[8] % MOD, // s[8]
        base[9] % MOD, // s[9]
    ];

    for _n in 10..=N {
        let mut val = 0u64;
        for (i, &(c, positive)) in coeffs.iter().enumerate() {
            let term = c % MOD * window[7 - i] % MOD;
            if positive {
                val = (val + term) % MOD;
            } else {
                val = (val + MOD - term) % MOD;
            }
        }
        // Shift window
        for i in 0..7 {
            window[i] = window[i + 1];
        }
        window[7] = val;
    }

    println!("{}", window[7]);
}
