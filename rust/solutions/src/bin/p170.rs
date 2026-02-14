const ALL_DIGITS_MASK: u32 = 0x3FF;

fn popcount10(mask: u32) -> u32 {
    mask.count_ones()
}

fn digit_info(number: i64) -> Option<(u32, u32)> {
    if number <= 0 { return None; }
    let mut mask = 0u32;
    let mut length = 0u32;
    let mut n = number;
    while n > 0 {
        let d = (n % 10) as u32;
        let bit = 1 << d;
        if mask & bit != 0 { return None; }
        mask |= bit;
        n /= 10;
        length += 1;
    }
    Some((mask, length))
}

struct NR {
    value: i32,
    used_mask: u32,
}

fn gen_numbers(mask: u32, target_len: u32) -> Vec<NR> {
    let mut result = Vec::new();
    let mut stk: Vec<(u32, i32, u32)> = Vec::new();
    stk.push((0, 0, 0));

    while let Some((depth, value, used)) = stk.pop() {
        if depth == target_len {
            result.push(NR { value, used_mask: used });
            continue;
        }
        for digit in 0..=9u32 {
            let bit = 1 << digit;
            if mask & bit == 0 { continue; }
            if used & bit != 0 { continue; }
            if depth == 0 && target_len > 1 && digit == 0 { continue; }
            stk.push((depth + 1, value * 10 + digit as i32, used | bit));
        }
    }
    result
}

fn num_to_str(mut n: i64) -> String {
    if n == 0 { return "0".to_string(); }
    let mut digits = Vec::new();
    while n > 0 {
        digits.push((b'0' + (n % 10) as u8) as char);
        n /= 10;
    }
    digits.reverse();
    digits.into_iter().collect()
}

fn main() {
    let base_length = 2u32;
    let mut best = String::new();

    let bases = gen_numbers(ALL_DIGITS_MASK, base_length);

    for bi in &bases {
        let base = bi.value;
        let base_mask = bi.used_mask;
        if base == 0 { continue; }

        let remaining_mask = ALL_DIGITS_MASK ^ base_mask;
        let total_remaining = popcount10(remaining_mask);
        if total_remaining != 10 - base_length { continue; }

        for len1 in 1..total_remaining {
            let len2 = total_remaining - len1;
            let expected_len1 = len1 + 1;
            let expected_len2 = len2 + 1;

            let mults1 = gen_numbers(remaining_mask, len1);

            for mi in &mults1 {
                let mult1 = mi.value;
                let mask1 = mi.used_mask;
                let next_remaining = remaining_mask ^ mask1;
                if popcount10(next_remaining) != len2 { continue; }

                let prod1 = base as i64 * mult1 as i64;
                let Some((mask_prod1, len_prod1)) = digit_info(prod1) else { continue };
                if len_prod1 != expected_len1 { continue; }

                let mults2 = gen_numbers(next_remaining, len2);

                for mj in &mults2 {
                    let mult2 = mj.value;
                    let prod2 = base as i64 * mult2 as i64;
                    let Some((mask_prod2, len_prod2)) = digit_info(prod2) else { continue };
                    if len_prod2 != expected_len2 { continue; }
                    if mask_prod1 & mask_prod2 != 0 { continue; }
                    if mask_prod1 | mask_prod2 != ALL_DIGITS_MASK { continue; }

                    let s1 = num_to_str(prod1);
                    let s2 = num_to_str(prod2);
                    let candidate = format!("{}{}", s1, s2);

                    if best.is_empty() || candidate > best {
                        best = candidate;
                    }
                }
            }
        }
    }
    println!("{}", best);
}
