// Project Euler 290: Digital Signature
// DP over digits with state (diff, carry)

const N: usize = 18;
const K: usize = 137;
const B: usize = 10;
const DIFF_OFFSET: i32 = (B * N) as i32;
const DIFF_SIZE: usize = 2 * B * N + 1;

fn sum_digits(mut n: i32) -> i32 {
    let mut s = 0;
    while n > 0 {
        s += n % 10;
        n /= 10;
    }
    s
}

fn main() {
    let mut counts = vec![vec![0i64; K]; DIFF_SIZE];
    counts[DIFF_OFFSET as usize][0] = 1;

    for _ in 0..N {
        let mut next = vec![vec![0i64; K]; DIFF_SIZE];

        for j in 0..DIFF_SIZE {
            for k in 0..K {
                let count = counts[j][k];
                if count == 0 { continue; }
                for d in 0..B as i32 {
                    let t = d * K as i32 + k as i32;
                    let new_j = (j as i32 - DIFF_OFFSET) - sum_digits(k as i32) + sum_digits(t) - d + DIFF_OFFSET;
                    let new_k = (t / B as i32) as usize;
                    if new_j >= 0 && (new_j as usize) < DIFF_SIZE {
                        next[new_j as usize][new_k] += count;
                    }
                }
            }
        }
        counts = next;
    }

    let mut ans: i64 = 0;
    for carry in 0..K {
        ans += counts[DIFF_OFFSET as usize][carry];
    }
    println!("{}", ans);
}
