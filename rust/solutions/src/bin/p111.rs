// Project Euler 111 - Primes with Runs
// For 10-digit primes, find S(10,d) for each digit d and sum them.
use euler_utils::miller_rabin;

const N: usize = 10;

fn gen_combos(n: usize, k: usize) -> Vec<Vec<usize>> {
    let mut result = Vec::new();
    let mut combo = vec![0usize; k];
    fn helper(n: usize, k: usize, start: usize, depth: usize, combo: &mut Vec<usize>, result: &mut Vec<Vec<usize>>) {
        if depth == k {
            result.push(combo[..k].to_vec());
            return;
        }
        for i in start..n {
            combo[depth] = i;
            helper(n, k, i + 1, depth + 1, combo, result);
        }
    }
    helper(n, k, 0, 0, &mut combo, &mut result);
    result
}

fn main() {
    let mut total_s_sum: i64 = 0;

    for d in 0..=9u8 {
        let mut s_n_d: i64 = 0;

        for k_repeats in (1..=N).rev() {
            let num_other = N - k_repeats;
            let mut current_sum: i64 = 0;
            let mut current_count = 0;

            let combos = gen_combos(N, k_repeats);

            let mut cand = Vec::new();
            for x in 0..=9u8 {
                if x != d { cand.push(x); }
            }

            for combo in &combos {
                let mut is_d = [false; 10];
                for &pos in combo.iter() {
                    is_d[pos] = true;
                }
                let other_pos: Vec<usize> = (0..N).filter(|i| !is_d[*i]).collect();

                if num_other == 0 {
                    if d == 0 { continue; }
                    let mut num: u64 = 0;
                    for _ in 0..N {
                        num = num * 10 + d as u64;
                    }
                    if miller_rabin(num) {
                        current_sum += num as i64;
                        current_count += 1;
                    }
                } else {
                    let total_other: usize = 9usize.pow(num_other as u32);
                    for t in 0..total_other {
                        let mut digits = [0u8; 10];
                        for &pos in combo.iter() {
                            digits[pos] = d;
                        }
                        let mut tmp = t;
                        for i in 0..num_other {
                            digits[other_pos[i]] = cand[tmp % 9];
                            tmp /= 9;
                        }
                        if digits[0] == 0 { continue; }
                        let mut num: u64 = 0;
                        for i in 0..N {
                            num = num * 10 + digits[i] as u64;
                        }
                        if miller_rabin(num) {
                            current_sum += num as i64;
                            current_count += 1;
                        }
                    }
                }
            }

            if current_count > 0 {
                s_n_d = current_sum;
                break;
            }
        }

        total_s_sum += s_n_d;
    }

    println!("{}", total_s_sum);
}
