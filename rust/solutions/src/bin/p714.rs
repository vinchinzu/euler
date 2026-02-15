// Project Euler 714 - Duodigits
//
// For each k from 1 to 50000, find smallest multiple of k that uses at most
// 2 distinct digits. Uses meet-in-the-middle over digit positions.

use std::f64;

const NN: usize = 50_000;
const B: usize = 10;

fn d_func(k: usize) -> f64 {
    let mut pows_mod = [0i64; 20];
    let mut pows_f = [0.0f64; 20];

    for num_digits in 1.. {
        pows_f[0] = 1.0;
        pows_mod[0] = 1;
        for i in 1..num_digits {
            pows_f[i] = pows_f[i - 1] * B as f64;
            pows_mod[i] = pows_mod[i - 1] * B as i64 % k as i64;
        }

        let n = 1usize << num_digits;

        let mut nums = vec![0.0f64; n * B];
        let mut mods = vec![0i64; n * B];

        for bitset in 1..n {
            let i = bitset.trailing_zeros() as usize;
            let prev_bitset = bitset - (bitset & bitset.wrapping_neg());
            let num = nums[prev_bitset * B + 1] + pows_f[i];
            let md = mods[prev_bitset * B + 1] + pows_mod[i];
            for d in 1..B {
                nums[bitset * B + d] = d as f64 * num;
                mods[bitset * B + d] = d as i64 * md;
            }
        }

        let mut best = f64::MAX;
        for bitset in 0..n / 2 {
            for d1 in 0..B {
                for d2 in 1..B {
                    let num = nums[bitset * B + d1] + nums[(n - 1 - bitset) * B + d2];
                    let md = mods[bitset * B + d1] + mods[(n - 1 - bitset) * B + d2];
                    if num < best && md % k as i64 == 0 {
                        best = num;
                    }
                }
            }
        }

        if best < f64::MAX {
            return best;
        }
    }
    unreachable!()
}

fn main() {
    let mut ans: f64 = 0.0;
    for k in 1..=NN {
        ans += d_func(k);
    }
    println!("{:.12e}", ans);
}
