// Project Euler Problem 954
// Count numbers up to 10^13 that are not divisible by 7 and remain
// not divisible by 7 after swapping any two digits.
//
// Digit DP: for each length L (1..13), count L-digit numbers (no leading zeros)
// where mod 7 != 0 and no swap of two digits makes it divisible by 7.
//
// A swap of digits at positions i,j changes the number by
// (d_i - d_j) * (10^{L-1-i} - 10^{L-1-j}).
// After swap, number n' = n + (d_i - d_j) * (10^{p_j} - 10^{p_i}) where p_k = L-1-k.
// Wait, swapping digits at positions i and j:
// n' = n - d_i*10^{p_i} - d_j*10^{p_j} + d_i*10^{p_j} + d_j*10^{p_i}
//    = n + (d_j - d_i)*(10^{p_i} - 10^{p_j})
// For i < j, p_i > p_j, so 10^{p_i} - 10^{p_j} > 0.
// n' ≡ 0 (mod 7) iff n ≡ (d_i - d_j)*(10^{p_i} - 10^{p_j}) (mod 7)
//
// We only count numbers where is_bad=false and mod7 != 0.
// When is_bad becomes true, we prune (don't recurse further - saves time).

fn main() {
    let mut pow10_mod7 = [0i32; 14];
    pow10_mod7[0] = 1;
    for i in 1..=13 {
        pow10_mod7[i] = (pow10_mod7[i - 1] * 10) % 7;
    }

    let mut mod_inverse_table = [0i32; 7];
    for x in 1..7 {
        for y in 1..7 {
            if (x * y) % 7 == 1 {
                mod_inverse_table[x] = y as i32;
                break;
            }
        }
    }

    // diff_table[L][i][j] for i < j
    let mut diff_table = [[[0i32; 14]; 14]; 14];
    for l in 1..=13 {
        for i in 0..l {
            for j in (i + 1)..l {
                let p_i = l - 1 - i;
                let p_j = l - 1 - j;
                // (10^{p_i} - 10^{p_j}) mod 7 = 10^{p_j} * (10^{p_i-p_j} - 1) mod 7
                let val = (pow10_mod7[p_j] as i32 * ((pow10_mod7[p_i - p_j] as i32 - 1 + 7) % 7)) % 7;
                diff_table[l][i][j] = val;
            }
        }
    }

    let mut grand_total: i64 = 0;

    for cur_l in 1..=13usize {
        let mut total_count: i64 = 0;
        let mut digits = [0i32; 14];

        // Recursive DFS using explicit stack to avoid deep recursion overhead
        // State: (pos, mod7, digit_to_place_range_start)
        // Actually, recursive is fine since depth is at most 13.
        fn dfs(
            pos: usize,
            mod7: i32,
            cur_l: usize,
            digits: &mut [i32; 14],
            total_count: &mut i64,
            pow10_mod7: &[i32; 14],
            mod_inverse_table: &[i32; 7],
            diff_table: &[[[i32; 14]; 14]; 14],
        ) {
            if pos == cur_l {
                if mod7 != 0 {
                    *total_count += 1;
                }
                return;
            }

            let start_d = if pos == 0 { 1 } else { 0 };

            for d in start_d..=9i32 {
                let new_mod = (mod7 + d * pow10_mod7[cur_l - 1 - pos]) % 7;
                let mut is_bad = false;

                // Check if placing digit d at position pos creates a bad swap
                // with any previous position k
                for k in 0..pos {
                    let diff_val = diff_table[cur_l][k][pos];
                    if diff_val == 0 {
                        // Swapping positions k and pos doesn't change value mod 7
                        // (only when p_i ≡ p_j, which means positions have same power mod 7)
                        // n' ≡ n (mod 7), so if n ≡ 0 then n' ≡ 0, else n' !≡ 0
                        // This doesn't create a new divisibility, skip
                        // Wait: diff_val == 0 means (10^{p_i} - 10^{p_j}) ≡ 0 mod 7
                        // So n' ≡ n mod 7 regardless of digits. Not bad.
                        // But wait, the C code checks if new_mod == 0 here.
                        // If new_mod == 0 then n ≡ 0 mod 7, so n' ≡ 0 mod 7 too.
                        // But we're already filtering mod7 != 0 at the leaf.
                        // So this case doesn't create "extra" badness beyond mod7==0.
                        // Actually: if diff_val == 0 and new_mod == 0, the swap makes
                        // n' ≡ 0, but n ≡ 0 too, so the number itself is already div by 7.
                        // We filter those at the leaf. No need for is_bad here.
                        continue;
                    } else {
                        // n' ≡ 0 mod 7 iff (d_k - d) * diff_val ≡ n (mod 7)
                        // i.e., (d_k - d) ≡ n * inv(diff_val) (mod 7)
                        // Actually let me re-derive:
                        // n' = n + (d_k_at_pos_k - d_at_pos_pos) * diff_coeff
                        // Wait, swapping positions k and pos: the digit at k goes to pos, digit at pos goes to k.
                        // n' = n + (digits[k] - d) * (10^{p_k} - 10^{p_pos})
                        // where p_k = L-1-k > p_pos = L-1-pos (since k < pos)
                        // So diff_val = (10^{p_k} - 10^{p_pos}) mod 7 = diff_table[L][k][pos]
                        // n' ≡ 0 mod 7 iff new_mod + (digits[k] - d) * diff_val ≡ 0 mod 7
                        // iff (digits[k] - d) * diff_val ≡ -new_mod mod 7
                        // iff (digits[k] - d) ≡ (-new_mod) * inv(diff_val) mod 7
                        let inv_d = mod_inverse_table[diff_val as usize];
                        let target = (((7 - new_mod % 7) % 7) * inv_d) % 7;
                        let actual = ((digits[k] - d) % 7 + 7) % 7;
                        if actual == target {
                            is_bad = true;
                            break;
                        }
                    }
                }

                if !is_bad {
                    digits[pos] = d;
                    dfs(
                        pos + 1,
                        new_mod,
                        cur_l,
                        digits,
                        total_count,
                        pow10_mod7,
                        mod_inverse_table,
                        diff_table,
                    );
                }
                // If is_bad, we prune: don't count this branch at all
            }
        }

        dfs(
            0,
            0,
            cur_l,
            &mut digits,
            &mut total_count,
            &pow10_mod7,
            &mod_inverse_table,
            &diff_table,
        );

        grand_total += total_count;
    }

    println!("{}", grand_total);
}
