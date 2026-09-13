// Project Euler 569 - Mountain Range Peaks
//
// A mountain range with slopes of prime lengths, alternating up/down 45 degrees.
// P(k) = number of peaks visible from peak k. Find sum P(k) for k=1..N.

const N: usize = 2_500_000;

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

fn main() {
    let prime_sieve_limit = 90_000_000;
    let primes = primes_odd_sieve(prime_sieve_limit);

    if primes.len() < 2 * N {
        eprintln!("Not enough primes: {} < {}", primes.len(), 2 * N);
        return;
    }

    // Interleaved peak coordinates for cache locality
    let mut peaks = vec![0i64; 2 * N];

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
        let i2 = 2 * i;
        peaks[i2] = x;
        peaks[i2 + 1] = y;
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

            let j_usize = j as usize;
            let prev_vis_start = vis_start[j_usize];
            let prev_count = vis_count[j_usize];
            
            let j2 = 2 * j_usize;
            // SAFETY: j_usize < i < N, so j2 and j2+1 < 2*N
            let (mid_x, mid_y) = unsafe {
                (*peaks.get_unchecked(j2), *peaks.get_unchecked(j2 + 1))
            };
            let (base_x, base_y) = unsafe {
                (*peaks.get_unchecked(i2), *peaks.get_unchecked(i2 + 1))
            };

            let mut left = 0;
            let mut right = prev_count;
            while left < right {
                let mid_idx = (left + right) / 2;
                // SAFETY: mid_idx < prev_count, prev_vis_start + mid_idx < vis_data.len()
                let peak_idx = unsafe {
                    *vis_data.get_unchecked(prev_vis_start + mid_idx) as usize
                };
                let p2 = 2 * peak_idx;
                // SAFETY: peak_idx < j_usize < N, so p2 and p2+1 < 2*N
                let (p1x, p1y) = unsafe {
                    (*peaks.get_unchecked(p2), *peaks.get_unchecked(p2 + 1))
                };
                
                // Inline cross product: (mid - p1) x (base - mid)
                let dx1 = mid_x - p1x;
                let dy1 = mid_y - p1y;
                let dx2 = base_x - mid_x;
                let dy2 = base_y - mid_y;
                let turn_val = dx1 * dy2 - dy1 * dx2;
                
                if turn_val < 0 {
                    left = mid_idx + 1;
                } else {
                    right = mid_idx;
                }
            }

            if left >= prev_count {
                break;
            }
            // SAFETY: left < prev_count, prev_vis_start + left < vis_data.len()
            j = unsafe { *vis_data.get_unchecked(prev_vis_start + left) };
        }

        vis_count[i] = count;
        ans += count as i64;
    }

    println!("{}", ans);
}
