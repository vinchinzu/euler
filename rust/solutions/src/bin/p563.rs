// Project Euler 563 - Robot Welders
//
// Find sum of M(n) for n=2..100, where M(n) is the minimal area that can be
// manufactured as w*h in exactly n variants with h/w <= 11/10.
// Dimensions are 23-smooth numbers.

use rayon::prelude::*;

fn main() {
    let primes: &[i64] = &[2, 3, 5, 7, 11, 13, 17, 19, 23];
    let limit: i64 = 500_000_000;

    // Generate all 23-smooth numbers up to limit
    let mut smooths: Vec<i64> = Vec::new();

    fn gen_smooth(n: i64, pi: usize, limit: i64, primes: &[i64], out: &mut Vec<i64>) {
        out.push(n);
        for i in pi..primes.len() {
            let next = n * primes[i];
            if next > limit {
                break;
            }
            gen_smooth(next, i, limit, primes, out);
        }
    }

    gen_smooth(1, 0, limit, primes, &mut smooths);
    smooths.sort();

    // Count total pairs for allocation
    let nsmooths = smooths.len();
    let mut total_pairs: usize = 0;
    for i in 0..nsmooths {
        let w = smooths[i];
        let h_max = if w <= 1_000_000_000 {
            11 * w / 10
        } else {
            w + w / 10
        };
        let start = smooths.partition_point(|&x| x < w);
        let end = smooths.partition_point(|&x| x <= h_max);
        total_pairs += end - start;
    }

    let mut areas: Vec<i64> = Vec::with_capacity(total_pairs);
    for i in 0..nsmooths {
        let w = smooths[i];
        let h_max = if w <= 1_000_000_000 {
            11 * w / 10
        } else {
            w + w / 10
        };
        let start = smooths.partition_point(|&x| x < w);
        let end = smooths.partition_point(|&x| x <= h_max);
        for j in start..end {
            areas.push(w * smooths[j]);
        }
    }

    areas.par_sort_unstable();

    let mut found_area = [0i64; 101];
    let mut i = 0;
    while i < areas.len() {
        let mut j = i + 1;
        while j < areas.len() && areas[j] == areas[i] {
            j += 1;
        }
        let cnt = j - i;
        if cnt >= 2 && cnt <= 100 && found_area[cnt] == 0 {
            found_area[cnt] = areas[i];
        }
        i = j;
    }

    let answer: i64 = found_area[2..=100].iter().sum();
    println!("{}", answer);
}
