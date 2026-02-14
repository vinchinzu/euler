// Project Euler 376: Nontransitive sets of dice
use std::collections::HashMap;

const N_PIPS: usize = 30;
const K_SIDES: usize = 6;
const HALF_K_SQ: i32 = 18;

fn main() {
    // Precompute binomial coefficients
    let mut binom = vec![vec![0i64; N_PIPS + 2]; N_PIPS + 2];
    for i in 0..=N_PIPS + 1 {
        binom[i][0] = 1;
        for j in 1..=i {
            binom[i][j] = binom[i - 1][j - 1] + binom[i - 1][j];
        }
    }

    let mut memo: HashMap<u64, i64> = HashMap::new();

    fn pack_state(max_pip: i32, r1: i32, r2: i32, r3: i32, l1: i32, l2: i32, l3: i32) -> u64 {
        ((max_pip as u64) << 40) | ((r1 as u64) << 34) | ((r2 as u64) << 28)
            | ((r3 as u64) << 22) | ((l1 as u64) << 11) | ((l2 as u64) << 5) | (l3 as u64)
    }

    fn helper(
        max_pip: i32, rem1: i32, rem2: i32, rem3: i32,
        loss1: i32, loss2: i32, loss3: i32,
        binom: &[Vec<i64>], memo: &mut HashMap<u64, i64>,
    ) -> i64 {
        if loss1 >= HALF_K_SQ || loss2 >= HALF_K_SQ || loss3 >= HALF_K_SQ { return 0; }
        if rem1 == 0 && rem2 == 0 && rem3 == 0 {
            return binom[N_PIPS][max_pip as usize];
        }
        let key = pack_state(max_pip, rem1, rem2, rem3, loss1, loss2, loss3);
        if let Some(&v) = memo.get(&key) { return v; }

        let mut total: i64 = 0;
        for s1 in 0..=rem1 {
            for s2 in 0..=rem2 {
                for s3 in 0..=rem3 {
                    if s1 + s2 + s3 > 0 {
                        total += helper(
                            max_pip + 1,
                            rem1 - s1, rem2 - s2, rem3 - s3,
                            loss1 + rem1 * s2,
                            loss2 + rem2 * s3,
                            loss3 + rem3 * s1,
                            binom, memo,
                        );
                    }
                }
            }
        }
        memo.insert(key, total);
        total
    }

    let result = helper(0, K_SIDES as i32, K_SIDES as i32, K_SIDES as i32, 0, 0, 0, &binom, &mut memo) / 3;
    println!("{result}");
}
