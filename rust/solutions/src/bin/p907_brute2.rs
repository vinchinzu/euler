// Brute force for Problem 907: Stacking Cups
// Tower: linear sequence from bottom (index 0) to top (index n-1).
// Each position: (cup_size, orientation) where orientation is Up or Down.
//
// Adjacent rules (below, above):
// 1. Nesting (|size diff|=1, same orientation):
//    - Both Up: below is bigger (s_below = s_above + 1)
//    - Both Down: below is smaller (s_below = s_above - 1)
// 2. Base-to-base (|size diff|=2): below Down, above Up
// 3. Rim-to-rim (|size diff|=2): below Up, above Down
//
// Restriction: C_k can't have rim-to-rim connections with BOTH C_{k-2} and C_{k+2}.

fn main() {
    for n in 1..=12 {
        let count = count_towers(n);
        println!("S({}) = {}", n, count);
    }
}

fn count_towers(n: usize) -> u64 {
    let mut perm: Vec<usize> = (1..=n).collect();
    let mut total = 0u64;
    loop {
        // Try all orientation combinations
        for mask in 0..(1u32 << n) {
            let orient: Vec<bool> = (0..n).map(|i| (mask >> i) & 1 == 1).collect(); // true = Up, false = Down
            if is_valid_tower(&perm, &orient) {
                total += 1;
            }
        }
        if !next_permutation(&mut perm) {
            break;
        }
    }
    total
}

fn is_valid_tower(perm: &[usize], orient: &[bool]) -> bool {
    let n = perm.len();

    // Check adjacency rules
    for i in 0..n - 1 {
        let s_below = perm[i];
        let s_above = perm[i + 1];
        let o_below = orient[i];
        let o_above = orient[i + 1];
        let diff = (s_below as i32 - s_above as i32).unsigned_abs() as usize;

        let valid = if diff == 1 && o_below == o_above {
            // Nesting: same orientation
            if o_below {
                // Both Up: below is bigger
                s_below == s_above + 1
            } else {
                // Both Down: below is smaller
                s_below + 1 == s_above
            }
        } else if diff == 2 && !o_below && o_above {
            // Base-to-base: below Down, above Up
            true
        } else if diff == 2 && o_below && !o_above {
            // Rim-to-rim: below Up, above Down
            true
        } else {
            false
        };

        if !valid {
            return false;
        }
    }

    // Check restriction: C_k can't have rim-to-rim with both C_{k-2} and C_{k+2}
    // For each cup C_k, find its rim-to-rim neighbors
    for i in 0..n {
        let k = perm[i];
        let mut rim_with_k_minus_2 = false;
        let mut rim_with_k_plus_2 = false;

        // Check below neighbor (i-1)
        if i > 0 {
            let s_other = perm[i - 1];
            let diff = (k as i32 - s_other as i32).unsigned_abs() as usize;
            if diff == 2 {
                // Below is at i-1, above is at i
                // Rim-to-rim: below Up, above Down -> orient[i-1]=Up, orient[i]=Down
                if orient[i - 1] && !orient[i] {
                    if s_other == k + 2 { rim_with_k_plus_2 = true; }
                    if k >= 3 && s_other == k - 2 { rim_with_k_minus_2 = true; }
                }
                // Also: below is at i-1, above is at i
                // Rim-to-rim: below=C_k is at i-1... no, C_k is at position i, not i-1.
                // The connection at (i-1, i) has below = perm[i-1], above = perm[i] = k.
                // If this is rim-to-rim: orient[i-1]=Up, orient[i]=Down
                // Then perm[i-1] is below (Up) and C_k is above (Down).
                // C_k is participating in rim-to-rim (as the top cup, upside-down).
            }
        }

        // Check above neighbor (i+1)
        if i + 1 < n {
            let s_other = perm[i + 1];
            let diff = (k as i32 - s_other as i32).unsigned_abs() as usize;
            if diff == 2 {
                // Below is at i, above is at i+1
                // Rim-to-rim: below Up, above Down -> orient[i]=Up, orient[i+1]=Down
                if orient[i] && !orient[i + 1] {
                    if s_other == k + 2 { rim_with_k_plus_2 = true; }
                    if k >= 3 && s_other == k - 2 { rim_with_k_minus_2 = true; }
                }
            }
        }

        if rim_with_k_minus_2 && rim_with_k_plus_2 {
            return false;
        }
    }

    true
}

fn next_permutation(arr: &mut Vec<usize>) -> bool {
    let n = arr.len();
    if n <= 1 { return false; }
    let mut i = n - 2;
    while arr[i] >= arr[i + 1] {
        if i == 0 { return false; }
        i -= 1;
    }
    let mut j = n - 1;
    while arr[j] <= arr[i] { j -= 1; }
    arr.swap(i, j);
    arr[i + 1..].reverse();
    true
}
