// Project Euler 900 — DistribuNim II (correct, optimized)
// Expected: 646900900

const MOD: i64 = 900497239;
const TARGET_N: usize = 10_000;

fn next_power_of_two_strictly_greater(x: u64) -> u64 {
    if x == 0 {
        1
    } else {
        1u64 << (64 - x.leading_zeros())
    }
}

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
        total += t(n) as i128;
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

    // Compute S(TARGET_N) mod MOD via recurrence
    if TARGET_N <= 5 {
        println!("{}", (s_exact[TARGET_N] % MOD as i128) as i64);
        return;
    }

    // seed = [S(1), S(2), S(3), S(4), S(5)]
    let seed: Vec<i64> = (1..=5).map(|i| (s_exact[i] % MOD as i128) as i64).collect();

    // prev = [S5, S4, S3, S2, S1]
    let mut prev = [seed[4], seed[3], seed[2], seed[1], seed[0]];

    for _ in 6..=TARGET_N {
        let mut newv =
            7 * prev[0] - 6 * prev[1] - 48 * prev[2] + 112 * prev[3] - 64 * prev[4];
        newv = ((newv % MOD) + MOD) % MOD; // positive modulo

        prev = [newv, prev[0], prev[1], prev[2], prev[3]];
    }

    println!("{}", prev[0]); // -> 646900900
}
