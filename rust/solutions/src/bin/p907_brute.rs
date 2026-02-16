// Brute force for Problem 907: Stacking Cups
// Model: tower is a linear sequence from bottom to top.
// Each cup has orientation (up/down).
// Adjacent cups must satisfy one of:
//   1. Nesting: C_k inside C_{k+1}. Big cup below, small cup above.
//      Both same orientation.
//   2. Base-to-base: |sizes differ by 2|. Bottom cup upside-down, top cup up.
//   3. Rim-to-rim: |sizes differ by 2|. Bottom cup up, top cup upside-down.
// Restriction: can't have both C_{k+2} and C_{k-2} rim-to-rim on C_k.

fn main() {
    for n in 1..=8 {
        let count = count_towers(n);
        println!("S({}) = {}", n, count);
    }
}

fn count_towers(n: usize) -> u64 {
    let mut perm: Vec<usize> = (1..=n).collect();
    let mut count = 0u64;
    loop {
        for orient_mask in 0..(1u32 << n) {
            let orientations: Vec<bool> = (0..n).map(|i| (orient_mask >> i) & 1 == 1).collect();
            if is_valid_tower(&perm, &orientations) {
                count += 1;
            }
        }
        if !next_permutation(&mut perm) {
            break;
        }
    }
    count
}

fn is_valid_tower(perm: &[usize], orient: &[bool]) -> bool {
    let n = perm.len();
    for i in 0..n - 1 {
        let cup_below = perm[i];
        let cup_above = perm[i + 1];
        let o_below = orient[i];
        let o_above = orient[i + 1];

        let valid = if (cup_below as i32 - cup_above as i32).unsigned_abs() == 1 {
            // Nesting: consecutive sizes. Both same orientation.
            o_below == o_above
        } else if (cup_below as i32 - cup_above as i32).unsigned_abs() == 2 {
            // Size differ by 2: base-to-base or rim-to-rim
            (!o_below && o_above) || (o_below && !o_above)
        } else {
            false
        };

        if !valid {
            return false;
        }
    }

    // Check restriction on rim-to-rim
    check_restriction(perm, orient)
}

fn check_restriction(perm: &[usize], orient: &[bool]) -> bool {
    let n = perm.len();
    // For each cup C_k, find all rim-to-rim connections TO C_k.
    // C_k is right-way-up, neighbor is upside-down, size diff = 2.
    // Can't have both C_{k-2} and C_{k+2} rim-to-rim on C_k.
    for i in 0..n {
        let k = perm[i];
        if !orient[i] { continue; } // C_k must be right-way-up for rim-to-rim

        let mut rim_neighbors: Vec<usize> = Vec::new();

        // Check above
        if i + 1 < n {
            let other = perm[i + 1];
            if !orient[i + 1] && (k as i32 - other as i32).unsigned_abs() == 2 {
                rim_neighbors.push(other);
            }
        }
        // Check below (the cup below C_k connects to C_k)
        if i > 0 {
            let other = perm[i - 1];
            if !orient[i - 1] && (k as i32 - other as i32).unsigned_abs() == 2 {
                // This is: other (upside-down) below, C_k (right-way-up) above.
                // That's actually a BASE-TO-BASE connection (bottom down, top up).
                // NOT rim-to-rim on C_k.
                // Rim-to-rim ON C_k means something is on TOP of C_k.
            }
        }

        // Restriction: rim_neighbors can't contain both k-2 and k+2
        if rim_neighbors.contains(&(k + 2)) && k >= 3 && rim_neighbors.contains(&(k - 2)) {
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
