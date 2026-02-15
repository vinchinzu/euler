// Project Euler 720 - Unpredictable Permutations
//
// Build elements[] and ranks[] arrays for N=25 (2^25 entries), then compute
// answer using factorial-weighted differences.

const N: usize = 25;
const BIG_M: i64 = 1_000_000_007;
const L: usize = 1 << N;

fn main() {
    let mut elements = vec![0i64; L];
    let mut ranks = vec![0i64; L];

    elements[0] = 1;
    elements[1] = 3;
    elements[2] = 2;
    elements[3] = 4;
    ranks[0] = 1;
    ranks[1] = 2;
    ranks[2] = 2;
    ranks[3] = 4;

    let mut i = 4;
    while i < L {
        for j in 0..i {
            ranks[i + j] = ranks[j] + elements[j];
            elements[i + j] = 2 * elements[j];
            elements[j] = 2 * elements[j] - 1;
        }
        elements[i - 1] = 2;
        elements[i] = 2 * i as i64 - 1;
        ranks[i - 1] = 2;
        ranks[i] = i as i64 + 1;
        i *= 2;
    }

    // Precompute factorials mod M
    let mut factorials = vec![0i64; L];
    factorials[0] = 1;
    for i in 1..L {
        factorials[i] = factorials[i - 1] * (i as i64) % BIG_M;
    }

    let mut ans: i64 = 1;
    for i in 0..L {
        let diff = elements[i] - ranks[i];
        ans = (ans as i128 + factorials[L - 1 - i] as i128 * diff as i128 % BIG_M as i128) as i64 % BIG_M;
        if ans < 0 {
            ans += BIG_M;
        }
    }

    println!("{}", ans);
}
