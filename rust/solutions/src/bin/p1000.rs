// Problem 1000: Max And / Max Xor Sum / Unreachable Nim, then
// M(0)=I(1000), M(1)=X(1000), M(2)=C(1000), M(k)=M(k-1)M(k-2)M(k-3).
// Find M(1000) mod 10^9+7.

const MOD: u64 = 1_000_000_007;

fn ones_in_bit_to_n(n: i32, bit: u32) -> u64 {
    if n < 0 {
        return 0;
    }
    let half_period = 1i32 << bit;
    let period = half_period << 1;
    let value_count = n as i64 + 1;
    let full = value_count / period as i64;
    let rem = value_count % period as i64;
    (full * half_period as i64 + (rem - half_period as i64).max(0)) as u64
}

fn max_and(n: i32) -> u64 {
    if n <= 0 {
        return 0;
    }
    let mut total = 0u64;
    let bits = 32 - n.leading_zeros();
    for bit in 0..bits {
        let count = ones_in_bit_to_n(n, bit);
        let smaller = count / 2;
        let larger = count - smaller;
        total += (1u64 << bit) * smaller * larger;
    }
    total
}

fn max_xor_sum(n: i32) -> u64 {
    if n <= 1 {
        return 0;
    }
    let n = n as usize;
    let vertex_bits = (usize::BITS - n.leading_zeros()).max(1);
    let payload_bits = 2 * vertex_bits;
    let vertex_mask = (1usize << vertex_bits) - 1;

    let mut squares = [0u64; 1001];
    for v in 0..=n {
        squares[v] = (v as u64) * (v as u64);
    }

    let total_edges = n * (n - 1) / 2;

    // Fused generation and Radix sort pass 1 (lower 10 bits of weight)
    let mut count0 = [0usize; 1024];
    for left in 1..n {
        let left_sq = squares[left];
        for right in (left + 1)..=n {
            let weight = left_sq ^ squares[right];
            count0[(weight & 0x3FF) as usize] += 1;
        }
    }

    let mut offset0 = [0usize; 1024];
    let mut acc = 0;
    for i in 0..1024 {
        offset0[i] = acc;
        acc += count0[i];
    }

    let mut tmp = vec![0u64; total_edges];
    for left in 1..n {
        let left_sq = squares[left];
        let left_shift = (left as u64) << vertex_bits;
        for right in (left + 1)..=n {
            let weight = left_sq ^ squares[right];
            let b = (weight & 0x3FF) as usize;
            let val = (weight << payload_bits) | left_shift | (right as u64);
            tmp[offset0[b]] = val;
            offset0[b] += 1;
        }
    }

    // Radix sort pass 2 (upper 10 bits of weight)
    let mut count1 = [0usize; 1024];
    for &x in tmp.iter() {
        let b = ((x >> (payload_bits + 10)) & 0x3FF) as usize;
        count1[b] += 1;
    }
    let mut offset1 = [0usize; 1024];
    acc = 0;
    for i in 0..1024 {
        offset1[i] = acc;
        acc += count1[i];
    }
    let mut packed = vec![0u64; total_edges];
    for &x in tmp.iter() {
        let b = ((x >> (payload_bits + 10)) & 0x3FF) as usize;
        packed[offset1[b]] = x;
        offset1[b] += 1;
    }

    let mut best = vec![0u64; n + 1];
    let mut pending = vec![0u64; n + 1];
    let mut touched: Vec<usize> = Vec::with_capacity(n + 1);
    let mut index = 0;
    while index < packed.len() {
        let weight = packed[index] >> payload_bits;
        touched.clear();
        while index < packed.len() && packed[index] >> payload_bits == weight {
            let p = packed[index];
            let left = ((p >> vertex_bits) as usize) & vertex_mask;
            let right = (p as usize) & vertex_mask;

            let cand_left = best[right] + weight;
            if cand_left > pending[left] {
                if pending[left] == 0 {
                    touched.push(left);
                }
                pending[left] = cand_left;
            }
            let cand_right = best[left] + weight;
            if cand_right > pending[right] {
                if pending[right] == 0 {
                    touched.push(right);
                }
                pending[right] = cand_right;
            }
            index += 1;
        }
        for &v in &touched {
            if pending[v] > best[v] {
                best[v] = pending[v];
            }
            pending[v] = 0;
        }
    }
    *best.iter().max().unwrap()
}

const EVEN_XOR: [u8; 4] = [0b000, 0b011, 0b101, 0b110];

fn count_unreachable_with_highest_xor_bit(limit: i32, pivot: u32) -> u64 {
    let bit_count = 32 - limit.leading_zeros();
    let mut counts = [0u64; 8];
    counts[0b111] = 1;

    for bit in (0..bit_count).rev() {
        let patterns: &[u8] = if bit > pivot {
            &EVEN_XOR
        } else if bit == pivot {
            &[0b111]
        } else {
            &[0, 1, 2, 3, 4, 5, 6, 7]
        };
        let limit_bit = ((limit >> bit) & 1) as u8;
        let mut next = [0u64; 8];
        for tight in 0..8u8 {
            let ways = counts[tight as usize];
            if ways == 0 {
                continue;
            }
            for &pattern in patterns {
                let mut next_tight = 0u8;
                let mut valid = true;
                for pile in 0..3 {
                    let pile_mask = 1 << pile;
                    if tight & pile_mask != 0 {
                        let chosen = (pattern >> pile) & 1;
                        if chosen > limit_bit {
                            valid = false;
                            break;
                        }
                        if chosen == limit_bit {
                            next_tight |= pile_mask;
                        }
                    }
                }
                if valid {
                    next[next_tight as usize] += ways;
                }
            }
        }
        counts = next;
    }
    counts.iter().sum()
}

fn count_unreachable_nim(n: i32) -> u64 {
    if n <= 0 {
        return 0;
    }
    let limit = n - 1;
    if limit < 0 {
        return 0;
    }
    let bit_count = (32 - limit.leading_zeros()).max(1);
    (0..bit_count)
        .map(|pivot| count_unreachable_with_highest_xor_bit(limit, pivot))
        .sum()
}

fn meta(last: usize, i: u64, x: u64, c: u64) -> u64 {
    let mut vals = [i % MOD, x % MOD, c % MOD];
    if last < 3 {
        return vals[last];
    }
    for _ in 3..=last {
        let next = {
            let a = vals[0] as u128;
            let b = vals[1] as u128;
            let c = vals[2] as u128;
            ((a * b % MOD as u128) * c % MOD as u128) as u64
        };
        vals[0] = vals[1];
        vals[1] = vals[2];
        vals[2] = next;
    }
    vals[2]
}

fn main() {
    let i = max_and(1000);
    let x = max_xor_sum(1000);
    let c = count_unreachable_nim(1000);
    let ans = meta(1000, i, x, c);
    println!("{}", ans);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn samples() {
        assert_eq!(max_and(10), 50);
        assert_eq!(max_xor_sum(4), 71);
        assert_eq!(max_xor_sum(10), 702);
        assert_eq!(count_unreachable_nim(10), 123);
        let i = max_and(1000);
        let x = max_xor_sum(1000);
        let c = count_unreachable_nim(1000);
        assert_eq!(meta(4, i, x, c), 457_587_170);
    }
}
