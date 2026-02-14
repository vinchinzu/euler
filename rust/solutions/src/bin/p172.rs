// Project Euler 172: Numbers with few repeated digits
fn main() {
    const N_DIGITS: usize = 18;
    const BASE: usize = 4;
    const NUM_STATES: usize = 1_048_576; // 4^10

    let mut powers = [0usize; 10];
    powers[0] = 1;
    for d in 1..10 {
        powers[d] = powers[d - 1] * BASE;
    }

    let mut dp = vec![0i64; NUM_STATES];
    let mut ndp = vec![0i64; NUM_STATES];
    dp[0] = 1;

    for pos in 0..N_DIGITS {
        ndp.iter_mut().for_each(|x| *x = 0);

        for state in 0..NUM_STATES {
            if dp[state] == 0 { continue; }

            for digit in 0..10usize {
                if pos == 0 && digit == 0 { continue; }
                let count = (state / powers[digit]) % BASE;
                if count >= 3 { continue; }

                let new_state = state + powers[digit];
                ndp[new_state] += dp[state];
            }
        }

        std::mem::swap(&mut dp, &mut ndp);
    }

    let total: i64 = dp.iter().sum();
    println!("{}", total);
}
