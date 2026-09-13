// Project Euler 882
// Compute G[i] values using bit manipulation and floor-based search.

fn main() {
    let n = 100_000usize;
    let mut g = vec![0.0f64; n + 1];
    let mut total = 0.0f64;

    for i in 1..=n {
        let mut low = 0.0f64;
        let mut high = f64::MAX;
        let i_u64 = i as u64;
        let bits = 64 - i_u64.leading_zeros();
        
        for j in 0..bits {
            let mask = 1usize << j;
            let remaining = (i >> (j + 1) << j) + (i & (mask - 1));
            
            // SAFETY: remaining < i, and i <= n, so remaining < n+1 (array size)
            let g_remaining = unsafe { *g.get_unchecked(remaining) };
            
            if (i & mask) != 0 {
                if g_remaining > low { low = g_remaining; }
            } else {
                if g_remaining < high { high = g_remaining; }
            }
        }
        
        let mut d = 1.0f64;
        let mut gi = 0.0f64;
        while gi <= low || gi >= high {
            gi = (low / d + 1.0).floor() * d;
            d *= 0.5;
        }
        
        // SAFETY: i is in range 1..=n, so i < n+1 (array size)
        unsafe { *g.get_unchecked_mut(i) = gi; }
        total += i as f64 * gi;
    }

    println!("{}", total.ceil() as i64);
}
