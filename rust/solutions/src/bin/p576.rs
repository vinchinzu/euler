// Project Euler 576 - Irrational Jumps

use euler_utils::primes_up_to;
use rayon::prelude::*;

#[derive(Clone, Copy)]
struct JumpPos {
    frac: f64,
    step: u32,
    prime_idx: u8,
}

#[inline]
fn frac_part(x: f64) -> f64 {
    x - x.floor()
}

#[derive(Clone, Copy)]
struct MonoDeque {
    buf: [(u32, u32); 16],
    head: u32,
    tail: u32,
    pushed: u32,
    popped: u32,
}

impl MonoDeque {
    const fn new() -> Self {
        Self {
            buf: [(0, 0); 16],
            head: 0,
            tail: 0,
            pushed: 0,
            popped: 0,
        }
    }
}

struct WindowTracker {
    deques: [MonoDeque; 25],
    cur_mins: [u32; 25],
    sqrt_inv: [f64; 25],
    cur_total: f64,
}

impl WindowTracker {
    fn new(sqrt_inv: [f64; 25]) -> Self {
        Self {
            deques: [MonoDeque::new(); 25],
            cur_mins: [0; 25],
            sqrt_inv,
            cur_total: 0.0,
        }
    }

    #[inline(always)]
    fn push(&mut self, pi: usize, step: u32) {
        let dq = &mut self.deques[pi];
        let idx = dq.pushed;
        dq.pushed += 1;
        if dq.popped == idx {
            dq.head = 0;
            dq.tail = 1;
            dq.buf[0] = (idx, step);
            self.cur_mins[pi] = step;
            self.cur_total += step as f64 * self.sqrt_inv[pi];
        } else {
            let old_min = self.cur_mins[pi];
            if step < old_min {
                dq.head = 0;
                dq.tail = 1;
                dq.buf[0] = (idx, step);
                self.cur_mins[pi] = step;
                self.cur_total += (step as f64 - old_min as f64) * self.sqrt_inv[pi];
            } else {
                while dq.tail > dq.head && dq.buf[((dq.tail - 1) & 15) as usize].1 >= step {
                    dq.tail -= 1;
                }
                dq.buf[(dq.tail & 15) as usize] = (idx, step);
                dq.tail += 1;
            }
        }
    }

    #[inline(always)]
    fn pop(&mut self, pi: usize) {
        let dq = &mut self.deques[pi];
        let idx = dq.popped;
        dq.popped += 1;
        if dq.popped == dq.pushed {
            dq.head = 0;
            dq.tail = 0;
            let old_min = self.cur_mins[pi];
            self.cur_total -= old_min as f64 * self.sqrt_inv[pi];
            self.cur_mins[pi] = 0;
        } else if dq.buf[(dq.head & 15) as usize].0 == idx {
            dq.head += 1;
            let new_min = dq.buf[(dq.head & 15) as usize].1;
            let old_min = self.cur_mins[pi];
            self.cur_mins[pi] = new_min;
            self.cur_total += (new_min as f64 - old_min as f64) * self.sqrt_inv[pi];
        }
    }

    #[inline(always)]
    fn recompute_total(&mut self) {
        let mut sum = 0.0;
        for i in 0..25 {
            sum += self.cur_mins[i] as f64 * self.sqrt_inv[i];
        }
        self.cur_total = sum;
    }
}

fn main() {
    let n = 100;
    let d = 0.00002;

    let primes = primes_up_to(n);
    let nprimes = primes.len();
    let sqrt_inv: Vec<f64> = primes.iter().map(|&p| (1.0 / p as f64).sqrt()).collect();

    // Independent per-prime generation
    let per_prime: Vec<Vec<JumpPos>> = primes
        .par_iter()
        .enumerate()
        .map(|(pi, &p)| {
            let sqrt_inv_p = (1.0 / p as f64).sqrt();
            let mut tmp: Vec<JumpPos> = Vec::with_capacity(262144);
            let mut sorted: Vec<f64> = Vec::with_capacity(262144);
            let mut i = 0u32;
            loop {
                let total_len = i as f64 * sqrt_inv_p;
                tmp.push(JumpPos {
                    frac: frac_part(total_len),
                    step: i,
                    prime_idx: pi as u8,
                });
                i += 1;
                if i == 65535 || i == 131071 || i == 262143 {
                    sorted.clear();
                    sorted.extend(tmp.iter().map(|j| j.frac));
                    sorted.sort_unstable_by(|a, b| a.to_bits().cmp(&b.to_bits()));
                    let all_within = sorted.windows(2).all(|w| w[1] - w[0] <= d);
                    if all_within {
                        break;
                    }
                }
            }
            tmp
        })
        .collect();

    let total_len: usize = per_prime.iter().map(|v| v.len()).sum();
    let mut all_pos: Vec<JumpPos> = Vec::with_capacity(total_len);
    for v in per_prime {
        all_pos.extend_from_slice(&v);
    }

    all_pos.par_sort_unstable_by_key(|a| a.frac.to_bits());
    let total_count = all_pos.len();
    let mut sqrt_inv_arr = [0.0; 25];
    sqrt_inv_arr.copy_from_slice(&sqrt_inv);

    let chunk_size = 65536;
    let mut ranges: Vec<(usize, usize)> = Vec::new();
    let mut curr = nprimes;
    while curr < total_count {
        let next = (curr + chunk_size).min(total_count);
        ranges.push((curr, next));
        curr = next;
    }

    let ans = ranges
        .par_iter()
        .map(|&(range_start, range_end)| {
            let mut start = range_start;
            while start > 0 && all_pos[range_start].frac - all_pos[start - 1].frac <= d {
                start -= 1;
            }

            let mut tracker = WindowTracker::new(sqrt_inv_arr);
            for idx in start..range_start {
                tracker.push(all_pos[idx].prime_idx as usize, all_pos[idx].step);
            }

            let mut chunk_ans = 0.0f64;
            let mut cur_start = start;
            for end in range_start..range_end {
                let frac_end = all_pos[end].frac;
                while frac_end - all_pos[cur_start].frac > d {
                    tracker.pop(all_pos[cur_start].prime_idx as usize);
                    cur_start += 1;
                }

                if (end & 2047) == 0 {
                    tracker.recompute_total();
                }

                if tracker.cur_total > chunk_ans {
                    chunk_ans = tracker.cur_total;
                }

                let pi = all_pos[end].prime_idx as usize;
                let step = all_pos[end].step;
                tracker.push(pi, step);
            }
            chunk_ans
        })
        .reduce(|| 0.0f64, f64::max);

    println!("{:.4}", ans);
}
