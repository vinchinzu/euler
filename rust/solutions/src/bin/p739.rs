// Project Euler 739 - Summation of Summations
//
// Lucas sequence with binomial coefficient iteration.

const N: usize = 100_000_000;
const M: i64 = 1_000_000_007;

fn main() {
    let mut mod_invs = vec![0i64; N + 1];
    mod_invs[1] = 1;
    for i in 2..=N {
        mod_invs[i] = M - (M / i as i64) * mod_invs[(M % i as i64) as usize] % M;
    }

    let mut lucas = vec![0i64; N + 1];
    lucas[1] = 1;
    lucas[2] = 3;
    for i in 3..=N {
        lucas[i] = (lucas[i - 2] + lucas[i - 1]) % M;
    }

    let mut ncr1: i64 = 1;
    let mut ncr2: i64 = 1;
    let mut ans = lucas[N];

    for k in (2..N).rev() {
        let mult = (2 * N as i64 - 2 - k as i64) * mod_invs[N - k] % M;
        ncr1 = (ncr1 as i128 * mult as i128 % M as i128) as i64;
        ans = (ans + (lucas[k] as i128 * ((ncr1 - ncr2 + M) % M) as i128 % M as i128) as i64)
            % M;
        ncr2 = (ncr2 + ncr1) % M;
    }

    println!("{}", ans);
}
