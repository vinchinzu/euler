// Problem 961 - Removing Digits
//
// Two-player game: players alternate removing a single digit from a positive integer.
// Leading zeros are removed after each move. The player who removes the last nonzero
// digit wins. W(N) = count of positive integers < N where first player wins.
//
// Key insight: only the pattern of zero/nonzero digits matters, not the actual values.
// We represent states as binary patterns and use memoized game theory analysis.

use std::collections::HashMap;

/// Encode a pattern (slice of 0s and 1s, no leading zeros except the single "0" state)
/// into a u32 key: lower bits store the pattern bits, upper bits store length.
/// Length 0 is used for the empty/all-zeros terminal state.
fn encode(pattern: &[u8]) -> u32 {
    let len = pattern.len() as u32;
    let mut bits: u32 = 0;
    for &b in pattern {
        bits = (bits << 1) | (b as u32);
    }
    (len << 18) | bits
}

/// Normalize: strip leading zeros. If all zeros or empty, return empty slice encoding (terminal).
fn normalize_and_encode(pattern: &[u8]) -> u32 {
    // Find first nonzero
    let start = pattern.iter().position(|&b| b != 0);
    match start {
        Some(s) => encode(&pattern[s..]),
        None => 0, // terminal state (all zeros or empty)
    }
}

/// Check if pattern is terminal (no nonzero digits)
fn is_terminal_key(key: u32) -> bool {
    // Terminal if length is 0, or all bits are 0
    let len = key >> 18;
    if len == 0 {
        return true;
    }
    let bits = key & ((1 << 18) - 1);
    bits == 0
}

/// Determine if first player wins from a given state.
/// Uses memoization via HashMap.
fn first_player_wins(key: u32, cache: &mut HashMap<u32, bool>) -> bool {
    if is_terminal_key(key) {
        return false;
    }
    if let Some(&result) = cache.get(&key) {
        return result;
    }

    let len = (key >> 18) as usize;
    let bits = key & ((1 << 18) - 1);

    // Extract pattern into a small buffer
    let mut pattern = [0u8; 18];
    for i in 0..len {
        pattern[i] = ((bits >> (len - 1 - i)) & 1) as u8;
    }

    let mut result = false;
    for i in 0..len {
        // Create new pattern with digit i removed
        let mut new_pattern = [0u8; 18];
        let new_len = len - 1;
        let mut idx = 0;
        for j in 0..len {
            if j == i {
                continue;
            }
            new_pattern[idx] = pattern[j];
            idx += 1;
        }

        // Check if removing this digit leaves all zeros (immediate win)
        let any_nonzero = new_pattern[..new_len].iter().any(|&b| b != 0);
        if !any_nonzero {
            result = true;
            break;
        }

        // Normalize (strip leading zeros) and check opponent's position
        let new_key = normalize_and_encode(&new_pattern[..new_len]);

        if !first_player_wins(new_key, cache) {
            result = true;
            break;
        }
    }

    cache.insert(key, result);
    result
}

fn w(k: usize) -> u128 {
    // W(10^k): count of positive integers < 10^k where first player wins.
    // Sum over digit counts d = 1..k.
    let mut cache: HashMap<u32, bool> = HashMap::new();
    let mut total: u128 = 0;

    for d in 1..=k {
        // Enumerate all binary patterns of length d with first bit = 1.
        // That means we iterate over the remaining d-1 bits.
        let remaining = d - 1;
        let count = 1u32 << remaining; // 2^(d-1) patterns

        let mut d_total: u128 = 0;
        for mask in 0..count {
            // Build pattern: first bit is 1, then remaining bits from mask
            let pattern_bits = (1u32 << remaining) | mask;
            // The full pattern has d bits, stored in the lower d bits of pattern_bits
            let key = ((d as u32) << 18) | pattern_bits;

            if first_player_wins(key, &mut cache) {
                // Count nonzero digits (number of 1-bits in the pattern)
                let ones = pattern_bits.count_ones();
                // Each nonzero position can be any of 1..9, so multiply by 9^ones
                d_total += 9u128.pow(ones);
            }
        }
        total += d_total;
    }

    total
}

fn main() {
    // Verify test cases
    debug_assert_eq!(w(2), 18);
    debug_assert_eq!(w(4), 1656);

    // Compute W(10^18)
    println!("{}", w(18));
}
