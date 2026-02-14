// Project Euler 171: Sum of digits whose digit-square-sum is a perfect square

const DIGITS: usize = 20;
const MAX_SUM: usize = DIGITS * 81;
const MOD: i64 = 1_000_000_000;

fn main() {
    let mut pow10 = [0i64; DIGITS + 1];
    pow10[0] = 1;
    for i in 1..=DIGITS {
        pow10[i] = (pow10[i - 1] * 10) % MOD;
    }

    let mut is_square = vec![false; MAX_SUM + 1];
    for s in 0..=MAX_SUM {
        let root = (s as f64).sqrt() as usize;
        for r in root.saturating_sub(1)..=root + 1 {
            if r * r == s {
                is_square[s] = true;
                break;
            }
        }
    }

    let mut count_prev = vec![0i64; MAX_SUM + 1];
    let mut count_cur = vec![0i64; MAX_SUM + 1];
    let mut sum_prev = vec![0i64; MAX_SUM + 1];
    let mut sum_cur = vec![0i64; MAX_SUM + 1];

    count_prev[0] = 1;

    for length in 0..DIGITS {
        count_cur.iter_mut().for_each(|x| *x = 0);
        sum_cur.iter_mut().for_each(|x| *x = 0);

        for s in 0..=MAX_SUM {
            let cnt = count_prev[s];
            if cnt == 0 { continue; }
            let cur_sum = sum_prev[s];
            let factor = pow10[length];

            for d in 0..=9i64 {
                let ns = s + (d * d) as usize;
                if ns > MAX_SUM { break; }

                count_cur[ns] = (count_cur[ns] + cnt) % MOD;
                let added = (cnt % MOD * (d % MOD) % MOD * factor) % MOD;
                let total = (cur_sum + added) % MOD;
                sum_cur[ns] = (sum_cur[ns] + total) % MOD;
            }
        }

        std::mem::swap(&mut count_prev, &mut count_cur);
        std::mem::swap(&mut sum_prev, &mut sum_cur);
    }

    let mut result: i64 = 0;
    for s in 0..=MAX_SUM {
        if is_square[s] {
            result = (result + sum_prev[s]) % MOD;
        }
    }

    println!("{}", result);
}
