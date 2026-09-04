// Project Euler 413 - One-child Numbers
use rayon::prelude::*;

use euler_utils::gcd;

const BASE: usize = 10;

#[inline(always)]
fn hash_u64(x: u64) -> usize {
    ((x ^ (x >> 21)).wrapping_mul(0x517cc1b727220a95)) as usize
}

struct FastMap {
    keys: Vec<u64>,
    vals: Vec<i64>,
    occupied: Vec<u32>,
    mask: usize,
}

impl FastMap {
    fn with_capacity(cap: usize) -> Self {
        let size = cap.next_power_of_two();
        Self {
            keys: vec![u64::MAX; size],
            vals: vec![0; size],
            occupied: Vec::with_capacity(size / 2),
            mask: size - 1,
        }
    }

    #[inline(always)]
    fn clear(&mut self) {
        for &idx in &self.occupied {
            self.keys[idx as usize] = u64::MAX;
        }
        self.occupied.clear();
    }

    #[inline(always)]
    fn insert_add(&mut self, key: u64, val: i64) {
        let mut idx = hash_u64(key) & self.mask;
        loop {
            let k = self.keys[idx];
            if k == key {
                self.vals[idx] += val;
                return;
            }
            if k == u64::MAX {
                self.keys[idx] = key;
                self.vals[idx] = val;
                self.occupied.push(idx as u32);
                return;
            }
            idx = (idx + 1) & self.mask;
        }
    }
}

fn count_for_d_even(d: usize) -> i64 {
    let half = d / 2;
    let mask_bits = 2 * half;
    let parity_shift = mask_bits;
    let hits_shift = mask_bits + 1;
    let num_states = 1 << (hits_shift + 1);

    // Precompute single digit tables
    let mut hit_from_single = [0u32; 10];
    let mut sj_shift = [0usize; 10];
    let mut sj_clear = [0u32; 10];
    let added_c = [1u32, 2, 2, 2];

    for digit in 0..10 {
        let np = (digit & 1) as usize;
        let sj = ((digit % d) - np) / 2;
        sj_shift[digit] = 2 * sj;
        sj_clear[digit] = !(3u32 << (2 * sj));
        if np == 0 && sj == 0 {
            hit_from_single[digit] = 1;
        }
    }

    let low_len = 5.min(half);
    let low_size = 1 << (2 * low_len);
    let high_len = half.saturating_sub(5);
    let high_size = 1 << (2 * high_len);

    let mut trans_low = vec![0u32; 10 * 2 * low_size];
    let mut trans_high = vec![0u32; 10 * 2 * high_size];

    for digit in 0..10 {
        let np = (digit & 1) as usize;
        for parity in 0..2 {
            let base_idx_low = (digit * 2 + parity) * low_size;
            for low in 0..low_size {
                let mut mask = 0u32;
                let mut hit = 0u32;
                let mut temp = low;
                for j in 0..low_len {
                    let c = (temp & 3) as u32;
                    temp >>= 2;
                    if c > 0 {
                        let r = parity + 2 * j;
                        let nr = (r * 10 + digit) % d;
                        let nj = (nr - np) / 2;
                        if np == 0 && nj == 0 {
                            hit += c;
                        }
                        mask |= c << (2 * nj);
                    }
                }
                trans_low[base_idx_low + low] = (hit << 18) | mask;
            }

            let base_idx_high = (digit * 2 + parity) * high_size;
            for high in 0..high_size {
                let mut mask = 0u32;
                let mut hit = 0u32;
                let mut temp = high;
                for k in 0..high_len {
                    let j = 5 + k;
                    let c = (temp & 3) as u32;
                    temp >>= 2;
                    if c > 0 {
                        let r = parity + 2 * j;
                        let nr = (r * 10 + digit) % d;
                        let nj = (nr - np) / 2;
                        if np == 0 && nj == 0 {
                            hit += c;
                        }
                        mask |= c << (2 * nj);
                    }
                }
                trans_high[base_idx_high + high] = (hit << 18) | mask;
            }
        }
    }

    let mut dp = vec![0i64; num_states];
    let mut dp2 = vec![0i64; num_states];
    let mut occ1 = Vec::with_capacity(num_states.min(100_000));
    let mut occ2 = Vec::with_capacity(num_states.min(100_000));

    dp[0] = 1;
    occ1.push(0u32);

    let low_mask_filter = (1 << (2 * low_len)) - 1;

    for pos in 0..d {
        for &idx in &occ2 {
            dp2[idx as usize] = 0;
        }
        occ2.clear();

        let start_digit = if pos == 0 { 1 } else { 0 };

        for &key in &occ1 {
            let val = dp[key as usize];
            let hits = key >> hits_shift;
            let parity = ((key >> parity_shift) & 1) as usize;
            let mask = key & ((1 << mask_bits) - 1);

            let low = (mask & low_mask_filter) as usize;
            let high = (mask >> (2 * low_len)) as usize;

            let p_offset_low = parity * low_size + low;
            let p_offset_high = parity * high_size + high;

            for digit in start_digit..10 {
                let entry_low = trans_low[digit * (2 * low_size) + p_offset_low];
                let entry_high = trans_high[digit * (2 * high_size) + p_offset_high];

                let hit_prev = (entry_low >> 18) + (entry_high >> 18);
                let new_hits = hits + hit_prev + hit_from_single[digit];
                if new_hits > 1 {
                    continue;
                }

                let mapped = (entry_low & 0x3FFFF) | (entry_high & 0x3FFFF);
                let shift = sj_shift[digit];
                let cur_c = ((mapped >> shift) & 3) as usize;
                let new_mask = (mapped & sj_clear[digit]) | (added_c[cur_c] << shift);

                let next_parity = (digit & 1) as u32;
                let new_key = (new_hits << hits_shift) | (next_parity << parity_shift) | new_mask;
                let nk = new_key as usize;
                if dp2[nk] == 0 {
                    occ2.push(new_key);
                }
                dp2[nk] += val;
            }
        }

        std::mem::swap(&mut dp, &mut dp2);
        std::mem::swap(&mut occ1, &mut occ2);
    }

    let mut ans = 0i64;
    for &key in &occ1 {
        let hits = key >> hits_shift;
        if hits == 1 {
            ans += dp[key as usize];
        }
    }
    ans
}

fn count_for_d_g1(d: usize) -> i64 {
    let low_len = 9.min(d.saturating_sub(1));
    let low_size = 1 << low_len;
    let high_len = (d.saturating_sub(1)).saturating_sub(9);
    let high_size = 1 << high_len;

    let mut trans_low = vec![0u32; 10 * low_size];
    let mut trans_high = vec![0u32; 10 * high_size];
    let mut single_produces_zero = [false; 10];
    let mut single_key_bit = [0u32; 10];

    for digit in 0..10 {
        let rem = digit % d;
        single_produces_zero[digit] = rem == 0;
        single_key_bit[digit] = if rem > 0 { 1u32 << (rem - 1) } else { 0 };

        let base_low = digit * low_size;
        for low in 0..low_size {
            let mut key_mask = 0u32;
            let mut hit = 0u32;
            let mut temp = low;
            for j in 0..low_len {
                if (temp & 1) != 0 {
                    let r = 1 + j;
                    let nr = (r * 10 + digit) % d;
                    if nr == 0 {
                        hit = 1;
                    } else {
                        key_mask |= 1 << (nr - 1);
                    }
                }
                temp >>= 1;
            }
            trans_low[base_low + low] = (hit << 19) | key_mask;
        }

        let base_high = digit * high_size;
        for high in 0..high_size {
            let mut key_mask = 0u32;
            let mut hit = 0u32;
            let mut temp = high;
            for k in 0..high_len {
                if (temp & 1) != 0 {
                    let r = 10 + k;
                    let nr = (r * 10 + digit) % d;
                    if nr == 0 {
                        hit = 1;
                    } else {
                        key_mask |= 1 << (nr - 1);
                    }
                }
                temp >>= 1;
            }
            trans_high[base_high + high] = (hit << 19) | key_mask;
        }
    }

    let num_states = 1 << d;
    let mut dp = vec![0i64; num_states];
    let mut dp2 = vec![0i64; num_states];
    let mut occ1 = Vec::with_capacity(num_states.min(150_000));
    let mut occ2 = Vec::with_capacity(num_states.min(150_000));

    dp[0] = 1;
    occ1.push(0u32);

    let low_filter = (1 << low_len) - 1;
    let high_filter = (1 << high_len) - 1;
    let key_mask_filter = (1u32 << (d - 1)) - 1;

    for pos in 0..d {
        for &idx in &occ2 {
            dp2[idx as usize] = 0;
        }
        occ2.clear();

        let start_digit = if pos == 0 { 1 } else { 0 };

        for &key in &occ1 {
            let val = dp[key as usize];
            let hits = key >> (d - 1);

            let low = (key & low_filter) as usize;
            let high = ((key >> low_len) & high_filter) as usize;

            for digit in start_digit..10 {
                let entry_low = trans_low[digit * low_size + low];
                let entry_high = trans_high[digit * high_size + high];

                let hit_prev = (entry_low | entry_high) >> 19;
                let produces_zero = hit_prev != 0 || single_produces_zero[digit];

                let new_hits = if produces_zero { hits + 1 } else { hits };
                if new_hits > 1 {
                    continue;
                }

                let mapped_key = (entry_low | entry_high) & key_mask_filter;
                let new_key = (new_hits << (d - 1)) | mapped_key | single_key_bit[digit];
                let nk_usize = new_key as usize;
                if dp2[nk_usize] == 0 {
                    occ2.push(new_key);
                }
                dp2[nk_usize] += val;
            }
        }
        std::mem::swap(&mut dp, &mut dp2);
        std::mem::swap(&mut occ1, &mut occ2);
    }

    let mut ans = 0i64;
    for &key in &occ1 {
        let hits = key >> (d - 1);
        if hits == 1 {
            ans += dp[key as usize];
        }
    }
    ans
}

fn count_for_d(d: usize) -> i64 {
    let g = gcd(d as u64, BASE as u64) as usize;
    if g == 1 {
        return count_for_d_g1(d);
    }
    if (d & 3) == 2 && d % 5 != 0 {
        return count_for_d_even(d);
    }
    let cap = 2;

    // Precompute transitions (d = 5, 15)
    let mut next_r = [[0usize; 20]; 10];
    let mut digit_rem = [0usize; 10];
    for digit in 0..10 {
        digit_rem[digit] = digit % d;
        for r in 0..d {
            next_r[digit][r] = (r * 10 + digit) % d;
        }
    }

    let cap_size = 4096;
    let mut dp = FastMap::with_capacity(cap_size);
    let mut dp2 = FastMap::with_capacity(cap_size);
    dp.insert_add(0, 1);

    for pos in 0..d {
        dp2.clear();

        for i in 0..dp.occupied.len() {
            let idx = dp.occupied[i] as usize;
            let key = dp.keys[idx];
            let val = dp.vals[idx];

            let old_hits = ((key >> 62) & 1) as usize;

            let mut active_r = [0usize; 20];
            let mut active_c = [0u8; 20];
            let mut num_active = 0;
            for r in 0..d {
                let c = ((key >> (2 * r)) & 3) as u8;
                if c > 0 {
                    active_r[num_active] = r;
                    active_c[num_active] = c;
                    num_active += 1;
                }
            }

            let start_digit = if pos == 0 { 1 } else { 0 };
            for digit in start_digit..BASE {
                let mut new_counts = [0u8; 20];
                for k in 0..num_active {
                    let r = active_r[k];
                    let nr = next_r[digit][r];
                    new_counts[nr] += active_c[k];
                }
                new_counts[digit_rem[digit]] += 1;

                let new_hits = old_hits + (new_counts[0] as usize);
                if new_hits > 1 {
                    continue;
                }

                let mut new_key = (new_hits as u64) << 62;
                for r in 0..d {
                    let c = new_counts[r].min(cap);
                    new_key |= (c as u64) << (2 * r);
                }

                dp2.insert_add(new_key, val);
            }
        }

        std::mem::swap(&mut dp, &mut dp2);
    }

    let mut d_ans: i64 = 0;
    for &idx in &dp.occupied {
        let key = dp.keys[idx as usize];
        let val = dp.vals[idx as usize];
        let hits = ((key >> 62) & 1) as usize;
        if hits == 1 {
            d_ans += val;
        }
    }
    d_ans
}

fn main() {
    let n_max = 19;
    let total_ans: i64 = (1..=n_max).into_par_iter().map(count_for_d).sum();
    println!("{}", total_ans);
}
