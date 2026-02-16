// Faster brute force for Problem 907: Stacking Cups
// Use backtracking DFS instead of enumeration.

fn main() {
    for n in 1..=36 {
        let count = count_towers(n);
        println!("S({}) = {}", n, count);
    }
}

fn count_towers(n: usize) -> u64 {
    let mut used = vec![false; n + 1];
    let mut count = 0u64;
    // rim_connections[k] = set of sizes connected to k via rim-to-rim
    // We track this as a bitmask or pair
    let mut rim_connections = vec![[false; 2]; n + 1]; // [has_k-2, has_k+2]

    // Try each starting cup and orientation
    for start_cup in 1..=n {
        for start_orient in [true, false] { // true = Up, false = Down
            used[start_cup] = true;
            dfs(n, &mut used, start_cup, start_orient, 1, &mut rim_connections, &mut count);
            used[start_cup] = false;
        }
    }
    count
}

fn dfs(
    n: usize,
    used: &mut Vec<bool>,
    last_cup: usize,
    last_orient: bool, // true = Up, false = Down
    depth: usize,
    rim_conn: &mut Vec<[bool; 2]>, // rim_conn[k][0] = has rim with k-2, rim_conn[k][1] = has rim with k+2
    count: &mut u64,
) {
    if depth == n {
        *count += 1;
        return;
    }

    // Generate valid next moves from (last_cup, last_orient)
    // From (k, U): can go to (k-1, U), (k-2, D), (k+2, D)
    // From (k, D): can go to (k+1, D), (k-2, U), (k+2, U)

    let k = last_cup;

    if last_orient {
        // From (k, U)
        // Nesting: (k-1, U) if k >= 2
        if k >= 2 && !used[k - 1] {
            used[k - 1] = true;
            dfs(n, used, k - 1, true, depth + 1, rim_conn, count);
            used[k - 1] = false;
        }

        // Rim-to-rim: (k-2, D) or (k+2, D)
        // (k, U) -> (next, D): below is k (Up), above is next (Down), diff 2
        // This is a rim-to-rim connection involving k and next.
        // For the restriction: k has rim with next, and next has rim with k.

        for next in [k.wrapping_sub(2), k + 2] {
            if next >= 1 && next <= n && !used[next] {
                // Check restriction before placing
                // Adding rim connection between k and next
                // k's rim partner: if next = k+2, then k gets a k+2 partner. If next = k-2, k gets a k-2 partner.
                // next's rim partner: if next = k+2, then next=k+2 gets a (k+2)-2=k partner (k-2 slot). If next = k-2, then next=k-2 gets a (k-2)+2=k partner (k+2 slot).

                let (k_slot, next_val, next_slot) = if next == k + 2 {
                    (1, next, 0) // k gets k+2 partner (slot 1), next=(k+2) gets k-2=k partner (slot 0)
                } else {
                    (0, next, 1) // k gets k-2 partner (slot 0), next=(k-2) gets k+2=k partner (slot 1)
                };

                // Check restriction: k can't have both slots filled
                let k_other_slot = 1 - k_slot;
                if rim_conn[k][k_other_slot] {
                    continue; // Would violate restriction
                }
                // Check restriction: next can't have both slots filled
                let next_other_slot = 1 - next_slot;
                if rim_conn[next_val][next_other_slot] {
                    continue;
                }

                used[next_val] = true;
                rim_conn[k][k_slot] = true;
                rim_conn[next_val][next_slot] = true;
                dfs(n, used, next_val, false, depth + 1, rim_conn, count);
                rim_conn[next_val][next_slot] = false;
                rim_conn[k][k_slot] = false;
                used[next_val] = false;
            }
        }
    } else {
        // From (k, D)
        // Nesting: (k+1, D) if k+1 <= n
        if k + 1 <= n && !used[k + 1] {
            used[k + 1] = true;
            dfs(n, used, k + 1, false, depth + 1, rim_conn, count);
            used[k + 1] = false;
        }

        // Base-to-base: (k-2, U) or (k+2, U)
        // (k, D) -> (next, U): below is k (Down), above is next (Up), diff 2
        // This is a base-to-base connection (NOT rim-to-rim), so no restriction.

        for next in [k.wrapping_sub(2), k + 2] {
            if next >= 1 && next <= n && !used[next] {
                used[next] = true;
                dfs(n, used, next, true, depth + 1, rim_conn, count);
                used[next] = false;
            }
        }
    }
}
