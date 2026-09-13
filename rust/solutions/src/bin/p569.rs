// Project Euler 569 - Mountain Range Peaks
//
// A mountain range with slopes of prime lengths, alternating up/down 45 degrees.
// P(k) = number of peaks visible from peak k. Find sum P(k) for k=1..N.

const N: usize = 2_500_000;

#[inline]
fn primes_odd_sieve(limit: usize) -> Vec<usize> {
    let n_odds = (limit + 1) / 2;
    let mut odd = vec![true; n_odds];
    odd[0] = false;
    let mut i = 1usize;
    while {
        let p = 2 * i + 1;
        p * p <= limit
    } {
        if odd[i] {
            let p = 2 * i + 1;
            let mut j = (p * p) / 2;
            while j < n_odds {
                odd[j] = false;
                j += p;
            }
        }
        i += 1;
    }
    let mut primes = Vec::with_capacity(n_odds / 5);
    primes.push(2);
    for i in 1..n_odds {
        if odd[i] {
            let p = 2 * i + 1;
            if p <= limit {
                primes.push(p);
            }
        }
    }
    primes
}

#[inline(always)]
fn cross_product(p1x: i64, p1y: i64, mid_x: i64, mid_y: i64, base_x: i64, base_y: i64) -> i64 {
    let dx1 = mid_x - p1x;
    let dy1 = mid_y - p1y;
    let dx2 = base_x - mid_x;
    let dy2 = base_y - mid_y;
    dx1 * dy2 - dy1 * dx2
}

fn main() {
    let prime_sieve_limit = 90_000_000;
    let primes = primes_odd_sieve(prime_sieve_limit);

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

    for i in 0..N {
        x += primes[2 * i] as i64;
        y += primes[2 * i] as i64;
        peaks_x[i] = x;
        peaks_y[i] = y;
        x += primes[2 * i + 1] as i64;
        y -= primes[2 * i + 1] as i64;

        let vis_base = vis_data.len();
        vis_start[i] = vis_base;
        let mut count = 0usize;
        let mut j = i.wrapping_sub(1);

        let base_x = peaks_x[i];
        let base_y = peaks_y[i];

        while j < N {
            vis_data.push(j as i32);
            count += 1;

            if j == 0 {
                break;
            }

            let prev_vis_start = vis_start[j];
            let prev_count = vis_count[j];
            let mid_x = peaks_x[j];
            let mid_y = peaks_y[j];

            let mut left = 0usize;
            let mut right = prev_count;
            
            while left < right {
                let mid_idx = (left + right) >> 1;
                // SAFETY: mid_idx < prev_count, prev_vis_start + mid_idx < vis_data.len()
                let peak_idx = unsafe {
                    *vis_data.get_unchecked(prev_vis_start + mid_idx) as usize
                };
                // SAFETY: peak_idx < j < N
                let (p1x, p1y) = unsafe {
                    (*peaks_x.get_unchecked(peak_idx), *peaks_y.get_unchecked(peak_idx))
                };
                
                let turn_val = cross_product(p1x, p1y, mid_x, mid_y, base_x, base_y);
                
                if turn_val < 0 {
                    left = mid_idx + 1;
                } else {
                    right = mid_idx;
                }
            }

            if left >= prev_count {
                break;
            }
            j = vis_data[prev_vis_start + left] as usize;
        }

        vis_count[i] = count;
        ans += count as i64;
    }

    println!("{}", ans);
}
