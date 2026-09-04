// Project Euler 521 - Smallest Prime Factor Sum
//
// Lucy DP sieve: compute sum of smallest prime factors for 2..N mod M.

use euler_utils::sieve;

const N: i64 = 1_000_000_000_000;
const M: i64 = 1_000_000_000;

fn isqrt(n: i64) -> i64 {
    let mut x = (n as f64).sqrt() as i64;
    while x > 0 && x * x > n { x -= 1; }
    while (x + 1) * (x + 1) <= n { x += 1; }
    x
}

#[derive(Clone, Copy)]
#[repr(C)]
struct SmallVal {
    cnt: u32,
    sum: u32,
}

fn sum_2_to_n(n: i64) -> u32 {
    if n < 2 { return 0; }
    let s = (n as u128 * (n as u128 + 1) / 2 - 1) % M as u128;
    s as u32
}

fn solve() -> u64 {
    let l = isqrt(N);

    let is_prime = sieve(l as usize);
    let primes: Vec<u32> = (2..=l)
        .filter(|&i| is_prime[i as usize])
        .map(|i| i as u32)
        .collect();

    let small_len = (N / l + 2) as usize;
    let big_len = (l + 2) as usize;

    let mut big_cnt = vec![0i64; big_len];
    let mut big_sum = vec![0u32; big_len];
    let mut small = vec![SmallVal { cnt: 0, sum: 0 }; small_len];

    for i in 1..=l {
        big_cnt[i as usize] = N / i - 1;
        big_sum[i as usize] = sum_2_to_n(N / i);
    }
    for i in 1..(N / l) {
        small[i as usize] = SmallVal {
            cnt: (i - 1) as u32,
            sum: sum_2_to_n(i),
        };
    }

    let mut ans: u64 = 0;
    let m_u32 = M as u32;

    for &p_u32 in &primes {
        let p = p_u32 as usize;
        let p_i64 = p_u32 as i64;
        let p2 = p_i64 * p_i64;
        if p2 > N {
            break;
        }
        let p_val = small[p - 1];
        let p_cnt = p_val.cnt;
        let p_sum = p_val.sum;
        let p_mod = (p_u32 % m_u32) as u64;

        let removed = (big_cnt[p] - p_cnt as i64) % M;
        ans = (ans + p_mod * (removed as u64 % M as u64)) % M as u64;

        // Update big arrays
        let max_i = (N / p2).min(l) as usize;
        let i_split = ((l as usize) / p).min(max_i);

        let b_cnt_ptr = big_cnt.as_mut_ptr();
        let b_sum_ptr = big_sum.as_mut_ptr();
        let s_ptr = small.as_ptr();

        let mut idx = p;
        for i in 1..=i_split {
            unsafe {
                let cnt_remove = *b_cnt_ptr.add(idx) - p_cnt as i64;
                *b_cnt_ptr.add(i) -= cnt_remove;
                let cur_s = *b_sum_ptr.add(idx);
                let sum_remove = if cur_s >= p_sum { cur_s - p_sum } else { cur_s + m_u32 - p_sum };
                let term = ((p_mod * sum_remove as u64) % M as u64) as u32;
                let cur = *b_sum_ptr.add(i);
                *b_sum_ptr.add(i) = if cur >= term { cur - term } else { cur + m_u32 - term };
            }
            idx += p;
        }

        let n_div_p = (N / p_i64) as usize;
        let isqrt_n_div_p = (n_div_p as f64).sqrt() as usize;
        let mid_i = isqrt_n_div_p.min(max_i);

        let mut i = i_split + 1;
        if n_div_p <= u32::MAX as usize {
            let n_div_p_u32 = n_div_p as u32;
            while i <= mid_i {
                let q = (n_div_p_u32 / i as u32) as usize;
                unsafe {
                    let val = *s_ptr.add(q);
                    let cnt_remove = (val.cnt - p_cnt) as i64;
                    *b_cnt_ptr.add(i) -= cnt_remove;
                    let cur_s = val.sum;
                    let sum_remove = if cur_s >= p_sum { cur_s - p_sum } else { cur_s + m_u32 - p_sum };
                    let term = ((p_mod * sum_remove as u64) % M as u64) as u32;
                    let cur = *b_sum_ptr.add(i);
                    *b_sum_ptr.add(i) = if cur >= term { cur - term } else { cur + m_u32 - term };
                }
                i += 1;
            }

            let mut q = if i <= max_i { (n_div_p_u32 / i as u32) as usize } else { 0 };
            while i <= max_i {
                let i_last = (n_div_p_u32 / q as u32) as usize;
                let i_end = i_last.min(max_i);

                unsafe {
                    let val = *s_ptr.add(q);
                    let cnt_remove = (val.cnt - p_cnt) as i64;
                    let cur_s = val.sum;
                    let sum_remove = if cur_s >= p_sum { cur_s - p_sum } else { cur_s + m_u32 - p_sum };
                    let term = ((p_mod * sum_remove as u64) % M as u64) as u32;

                    let mut cp = b_cnt_ptr.add(i);
                    let mut sp = b_sum_ptr.add(i);
                    let count = i_end - i + 1;
                    for _ in 0..count {
                        *cp -= cnt_remove;
                        let cur = *sp;
                        *sp = if cur >= term { cur - term } else { cur + m_u32 - term };
                        cp = cp.add(1);
                        sp = sp.add(1);
                    }
                }
                i = i_end + 1;
                q -= 1;
            }
        } else {
            while i <= mid_i {
                let q = n_div_p / i;
                unsafe {
                    let val = *s_ptr.add(q);
                    let cnt_remove = (val.cnt - p_cnt) as i64;
                    *b_cnt_ptr.add(i) -= cnt_remove;
                    let cur_s = val.sum;
                    let sum_remove = if cur_s >= p_sum { cur_s - p_sum } else { cur_s + m_u32 - p_sum };
                    let term = ((p_mod * sum_remove as u64) % M as u64) as u32;
                    let cur = *b_sum_ptr.add(i);
                    *b_sum_ptr.add(i) = if cur >= term { cur - term } else { cur + m_u32 - term };
                }
                i += 1;
            }

            let mut q = if i <= max_i { n_div_p / i } else { 0 };
            while i <= max_i {
                let i_last = n_div_p / q;
                let i_end = i_last.min(max_i);

                unsafe {
                    let val = *s_ptr.add(q);
                    let cnt_remove = (val.cnt - p_cnt) as i64;
                    let cur_s = val.sum;
                    let sum_remove = if cur_s >= p_sum { cur_s - p_sum } else { cur_s + m_u32 - p_sum };
                    let term = ((p_mod * sum_remove as u64) % M as u64) as u32;

                    let mut cp = b_cnt_ptr.add(i);
                    let mut sp = b_sum_ptr.add(i);
                    let count = i_end - i + 1;
                    for _ in 0..count {
                        *cp -= cnt_remove;
                        let cur = *sp;
                        *sp = if cur >= term { cur - term } else { cur + m_u32 - term };
                        cp = cp.add(1);
                        sp = sp.add(1);
                    }
                }
                i = i_end + 1;
                q -= 1;
            }
        }

        // Update small arrays
        if p2 <= N / l {
            let p2_u = p2 as usize;
            let mut end = (N / l - 1) as usize;
            let mut q = end / p;
            let s_mut = small.as_mut_ptr();
            while end >= p2_u {
                let start = (q * p).max(p2_u);
                unsafe {
                    let val = *s_mut.add(q);
                    let cnt_remove = val.cnt - p_cnt;
                    let cur_s = val.sum;
                    let sum_remove = if cur_s >= p_sum { cur_s - p_sum } else { cur_s + m_u32 - p_sum };
                    let term = ((p_mod * sum_remove as u64) % M as u64) as u32;

                    let mut p_item = s_mut.add(start);
                    let count = end - start + 1;
                    for _ in 0..count {
                        (*p_item).cnt -= cnt_remove;
                        let cur = (*p_item).sum;
                        (*p_item).sum = if cur >= term { cur - term } else { cur + m_u32 - term };
                        p_item = p_item.add(1);
                    }
                }
                if start == p2_u {
                    break;
                }
                end = start - 1;
                q -= 1;
            }
        }
    }

    let final_sum = big_sum[1] as u64;
    (ans + final_sum) % M as u64
}

fn main() {
    println!("{}", solve());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_answer() {
        assert_eq!(solve(), 44389811);
    }
}
