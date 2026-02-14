// Project Euler 167: Investigating Ulam sequences
use std::collections::HashMap;

fn get_ulam_k(v: i32, k: i64) -> i64 {
    let mut terms: Vec<i32> = Vec::new();
    let mut sum_counts: Vec<u8> = vec![0; 65536];

    let mut add_term = |terms: &mut Vec<i32>, sum_counts: &mut Vec<u8>, val: i32| {
        let need = terms.last().map_or(0, |&last| (last + val) as usize + 1);
        if need > sum_counts.len() {
            sum_counts.resize(need.next_power_of_two(), 0);
        }
        for &t in terms.iter() {
            let s = (t + val) as usize;
            if sum_counts[s] < 2 {
                sum_counts[s] += 1;
            }
        }
        terms.push(val);
    };

    add_term(&mut terms, &mut sum_counts, 2);
    add_term(&mut terms, &mut sum_counts, v);

    // Bit array for odd flags
    let mut bits: Vec<u8> = vec![0; 8192];
    let set_bit = |bits: &mut Vec<u8>, idx: usize| {
        let byte = idx >> 3;
        if byte >= bits.len() {
            bits.resize((byte + 1).next_power_of_two(), 0);
        }
        bits[byte] |= 1 << (idx & 7);
    };
    let get_bit = |bits: &[u8], idx: i32| -> i32 {
        if idx < 0 { return 0; }
        let byte = (idx as usize) >> 3;
        if byte >= bits.len() { return 0; }
        ((bits[byte] >> (idx as usize & 7)) & 1) as i32
    };

    set_bit(&mut bits, ((v - 1) / 2) as usize);

    let mut even_terms = vec![2i32];

    while even_terms.len() < 2 {
        let last = *terms.last().unwrap();
        let mut candidate = last + 1;
        loop {
            if (candidate as usize) < sum_counts.len() && sum_counts[candidate as usize] == 1 {
                let need = terms.last().map_or(0, |&l| (l + candidate) as usize + 1);
                if need > sum_counts.len() {
                    sum_counts.resize(need.next_power_of_two(), 0);
                }
                for &t in terms.iter() {
                    let s = (t + candidate) as usize;
                    if sum_counts[s] < 2 {
                        sum_counts[s] += 1;
                    }
                }
                terms.push(candidate);

                if candidate & 1 == 1 {
                    set_bit(&mut bits, ((candidate - 1) / 2) as usize);
                } else {
                    even_terms.push(candidate);
                }
                break;
            }
            candidate += 1;
        }
    }

    // Continue generating odd Ulam terms until we exhaust even_count < 2 condition
    // Now find more terms to build up the bit pattern
    loop {
        let last = *terms.last().unwrap();
        let mut candidate = last + 1;
        let found;
        loop {
            if (candidate as usize) < sum_counts.len() && sum_counts[candidate as usize] == 1 {
                found = candidate;
                break;
            }
            candidate += 1;
        }
        let candidate = found;
        let need = terms.last().map_or(0, |&l| (l + candidate) as usize + 1);
        if need > sum_counts.len() {
            sum_counts.resize(need.next_power_of_two(), 0);
        }
        for &t in terms.iter() {
            let s = (t + candidate) as usize;
            if sum_counts[s] < 2 {
                sum_counts[s] += 1;
            }
        }
        terms.push(candidate);
        if candidate & 1 == 1 {
            set_bit(&mut bits, ((candidate - 1) / 2) as usize);
        } else {
            even_terms.push(candidate);
            // For U(2,v), there are only 2 even terms
        }

        // We want enough terms to detect the period
        if terms.len() > 500 { break; }
    }

    let e2 = even_terms[1];
    let t = (e2 / 2) as i32;

    let max_index = (*terms.last().unwrap() - 1) / 2;

    // Build initial state
    let state_bits = t.min(64);
    let state_mask: i64 = if state_bits < 64 { (1i64 << state_bits) - 1 } else { -1 };
    let mut i = max_index + 1;
    let mut state: i64 = 0;
    for offset in 0..state_bits {
        let idx2 = i - t + offset;
        let bit = get_bit(&bits, idx2);
        state = (state << 1) | bit as i64;
    }

    let mut state_map: HashMap<i64, i32> = HashMap::new();
    state_map.insert(state, i);

    let period_start;
    let period_length;

    loop {
        let prev_flag = get_bit(&bits, i - 1);
        let shifted_flag = get_bit(&bits, i - t);
        let new_flag = prev_flag ^ shifted_flag;
        if new_flag != 0 {
            set_bit(&mut bits, i as usize);
        }

        state = ((state << 1) & state_mask) | new_flag as i64;
        i += 1;

        if let Some(&found) = state_map.get(&state) {
            period_start = found;
            period_length = i - found;
            break;
        }
        state_map.insert(state, i);
    }

    // Collect prefix terms
    let mut prefix_terms: Vec<i32> = Vec::new();
    for idx2 in 0..period_start {
        if get_bit(&bits, idx2) != 0 {
            prefix_terms.push(2 * idx2 + 1);
        }
    }
    for &e in &even_terms {
        prefix_terms.push(e);
    }
    prefix_terms.sort();

    if k <= prefix_terms.len() as i64 {
        return prefix_terms[(k - 1) as usize] as i64;
    }

    let remaining = k - prefix_terms.len() as i64;

    let mut period_indices: Vec<i32> = Vec::new();
    for offset in 0..period_length {
        if get_bit(&bits, period_start + offset) != 0 {
            period_indices.push(offset);
        }
    }

    let pi_count = period_indices.len() as i64;
    let full_periods = (remaining - 1) / pi_count;
    let rem = ((remaining - 1) % pi_count) as usize;

    let base_index = period_start as i64 + full_periods * period_length as i64;
    let chosen_offset = period_indices[rem] as i64;
    let odd_index = base_index + chosen_offset;

    2 * odd_index + 1
}

fn main() {
    let mut total: i64 = 0;
    for n in 2..=10 {
        let v = 2 * n + 1;
        let val = get_ulam_k(v, 100_000_000_000);
        total += val;
    }
    println!("{}", total);
}
