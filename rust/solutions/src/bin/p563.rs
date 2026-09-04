// Project Euler 563 - Robot Welders
//
// Find sum of M(n) for n=2..100, where M(n) is the minimal area that can be
// manufactured as w*h in exactly n variants with h/w <= 11/10.
// Dimensions are 23-smooth numbers.

use rayon::prelude::*;

fn main() {
    let primes: &[i64] = &[2, 3, 5, 7, 11, 13, 17, 19, 23];
    let limit: i64 = 50_230_735;
    let max_area: i64 = 2_293_751_652_192_000;

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
    smooths.sort_unstable();

    let max_w = 47_893_127;
    let n_w = smooths.partition_point(|&x| x <= max_w);

    // Count total pairs for allocation and record row offsets
    let mut row_offsets: Vec<usize> = Vec::with_capacity(n_w + 1);
    row_offsets.push(0);
    for i in 0..n_w {
        let w = smooths[i];
        let h_max = (11 * w / 10).min(max_area / w);
        let end = smooths.partition_point(|&x| x <= h_max);
        let cnt = if end > i { end - i } else { 0 };
        row_offsets.push(row_offsets[i] + cnt);
    }
    let total_pairs = row_offsets[n_w];

    let mut areas: Vec<i64> = Vec::with_capacity(total_pairs);
    unsafe {
        areas.set_len(total_pairs);
    }
    let out_addr = areas.as_mut_ptr() as usize;
    let smooths_slice = smooths.as_slice();
    let offsets_slice = row_offsets.as_slice();

    (0..n_w).into_par_iter().for_each(|i| {
        let start_idx = unsafe { *offsets_slice.get_unchecked(i) };
        let cnt = unsafe { *offsets_slice.get_unchecked(i + 1) } - start_idx;
        if cnt > 0 {
            let w = unsafe { *smooths_slice.get_unchecked(i) };
            let out_ptr = (out_addr as *mut i64).wrapping_add(start_idx);
            let in_slice = unsafe { smooths_slice.get_unchecked(i..i + cnt) };
            for k in 0..cnt {
                unsafe {
                    *out_ptr.add(k) = w * *in_slice.get_unchecked(k);
                }
            }
        }
    });

    areas.par_sort_unstable();

    let mut found_area = [0i64; 101];
    let mut found_count = 0;
    let n = areas.len();
    let ptr = areas.as_ptr();
    let mut i = 0;
    while i < n {
        let val = unsafe { *ptr.add(i) };
        let mut j = i + 1;
        while j < n && unsafe { *ptr.add(j) } == val {
            j += 1;
        }
        let cnt = j - i;
        if cnt >= 2 && cnt <= 100 {
            let entry = &mut found_area[cnt];
            if *entry == 0 {
                *entry = val;
                found_count += 1;
                if found_count == 99 {
                    break;
                }
            }
        }
        i = j;
    }

    let answer: i64 = found_area[2..=100].iter().sum();
    println!("{}", answer);
}
