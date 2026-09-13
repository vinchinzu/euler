// Project Euler 900 — DistribuNim II (matrix exponentiation)
// Expected: 646900900

use euler_utils::ModMatrix;

const MOD: i64 = 900497239;
const TARGET_N: usize = 10_000;

fn next_power_of_two_strictly_greater(x: u64) -> u64 {
    if x == 0 {
        1
    } else {
        1u64 << (64 - x.leading_zeros())
    }
}

#[inline(always)]
fn t(n: usize) -> i64 {
    let n64 = n as u64;
    let p = next_power_of_two_strictly_greater(n64);
    let nn = n as i64;
    let val = -nn * nn - (nn & 1);
    ((val % p as i64) + p as i64) % (p as i64)
}

fn exact_s_up_to(nmax: usize) -> Vec<i128> {
    let mut s = vec![0i128; nmax + 1];
    let mut total = 0i128;
    let mut next_cut = 2usize;
    let mut k = 1;
    let limit = 1usize << nmax;
    
    for n in 1..=limit {
        let n64 = n as u64;
        let p = 1u64 << (64 - n64.leading_zeros());
        let nn = n as i64;
        let val = -nn * nn - (nn & 1);
        let t_val = ((val % p as i64) + p as i64) % (p as i64);
        
        total += t_val as i128;
        if n == next_cut {
            s[k] = total;
            k += 1;
            next_cut <<= 1;
            if k > nmax {
                break;
            }
        }
    }
    s
}

fn main() {
    // Build exact small values + verification (fast, ~32 k iterations)
    let s_exact = exact_s_up_to(15);

    assert_eq!(t(1), 0);
    assert_eq!(t(2), 0);
    assert_eq!(t(3), 2);
    assert_eq!(s_exact[10], 361_522);

    // Verify recurrence
    for n in 6..=15 {
        let lhs = s_exact[n];
        let rhs = 7 * s_exact[n - 1]
            - 6 * s_exact[n - 2]
            - 48 * s_exact[n - 3]
            + 112 * s_exact[n - 4]
            - 64 * s_exact[n - 5];
        assert_eq!(lhs, rhs);
    }

    // Compute S(TARGET_N) mod MOD via matrix exponentiation
    if TARGET_N <= 5 {
        println!("{}", (s_exact[TARGET_N] % MOD as i128) as i64);
        return;
    }

    let mod_u64 = MOD as u64;

    // Recurrence: S(n) = 7*S(n-1) - 6*S(n-2) - 48*S(n-3) + 112*S(n-4) - 64*S(n-5)
    // Convert negative coefficients to positive modulo equivalents
    let c1 = 7u64;
    let c2 = mod_u64 - 6; // -6 mod MOD
    let c3 = mod_u64 - 48; // -48 mod MOD
    let c4 = 112u64;
    let c5 = mod_u64 - 64; // -64 mod MOD

    // Matrix form: [S(n), S(n-1), S(n-2), S(n-3), S(n-4)]^T = M * [S(n-1), S(n-2), S(n-3), S(n-4), S(n-5)]^T
    let m = ModMatrix::<5>::from_data(
        [
            [c1, c2, c3, c4, c5],
            [1, 0, 0, 0, 0],
            [0, 1, 0, 0, 0],
            [0, 0, 1, 0, 0],
            [0, 0, 0, 1, 0],
        ],
        mod_u64,
    );

    // Initial state: [S(5), S(4), S(3), S(2), S(1)]
    let initial: [u64; 5] = [
        (s_exact[5] % MOD as i128) as u64,
        (s_exact[4] % MOD as i128) as u64,
        (s_exact[3] % MOD as i128) as u64,
        (s_exact[2] % MOD as i128) as u64,
        (s_exact[1] % MOD as i128) as u64,
    ];

    // M^(TARGET_N - 5) * initial gives [S(TARGET_N), S(TARGET_N-1), ...]
    let result = m.pow((TARGET_N - 5) as u64).mul_vec(&initial);

    println!("{}", result[0]); // -> 646900900
}
