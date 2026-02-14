// Project Euler 569 - Mountain Range Peaks
//
// A mountain range with slopes of prime lengths, alternating up/down 45 degrees.
// P(k) = number of peaks visible from peak k. Find sum P(k) for k=1..N.

use euler_utils::primes_up_to;

const N: usize = 2_500_000;

fn main() {
    let prime_sieve_limit = 90_000_000;
    let primes = primes_up_to(prime_sieve_limit);

    if primes.len() < 2 * N {
        eprintln!("Not enough primes: {} < {}", primes.len(), 2 * N);
        return;
    }

    let mut peaks_x = vec![0i64; N];
    let mut peaks_y = vec![0i64; N];

    // Visible peak storage
    let mut vis_data: Vec<i32> = Vec::with_capacity(30_000_000);
    let mut vis_start = vec![0usize; N];
    let mut vis_count = vec![0usize; N];

    let mut x: i64 = 0;
    let mut y: i64 = 0;
    let mut ans: i64 = 0;

    // Cross product: (p2-p1) x (p3-p2)
    let cross = |p1x: i64, p1y: i64, p2x: i64, p2y: i64, p3x: i64, p3y: i64| -> i64 {
        let dx1 = p2x - p1x;
        let dy1 = p2y - p1y;
        let dx2 = p3x - p2x;
        let dy2 = p3y - p2y;
        dx1 * dy2 - dy1 * dx2
    };

    for i in 0..N {
        x += primes[2 * i] as i64;
        y += primes[2 * i] as i64;
        peaks_x[i] = x;
        peaks_y[i] = y;
        x += primes[2 * i + 1] as i64;
        y -= primes[2 * i + 1] as i64;

        vis_start[i] = vis_data.len();
        let mut count = 0usize;
        let mut j = i as i32 - 1;

        while j >= 0 {
            vis_data.push(j);
            count += 1;

            if j == 0 {
                break;
            }

            let prev_vis_start = vis_start[j as usize];
            let prev_count = vis_count[j as usize];
            let mid_x = peaks_x[j as usize];
            let mid_y = peaks_y[j as usize];
            let base_x = peaks_x[i];
            let base_y = peaks_y[i];

            let mut left = 0;
            let mut right = prev_count;
            while left < right {
                let mid_idx = (left + right) / 2;
                let peak_idx = vis_data[prev_vis_start + mid_idx] as usize;
                let turn_val = cross(
                    peaks_x[peak_idx], peaks_y[peak_idx],
                    mid_x, mid_y,
                    base_x, base_y,
                );
                if turn_val < 0 {
                    left = mid_idx + 1;
                } else {
                    right = mid_idx;
                }
            }

            if left >= prev_count {
                break;
            }
            j = vis_data[prev_vis_start + left];
        }

        vis_count[i] = count;
        ans += count as i64;
    }

    println!("{}", ans);
}
