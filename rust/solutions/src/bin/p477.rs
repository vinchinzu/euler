// Project Euler 477: Number sequence game

const M: i64 = 1_000_000_007;
const N: usize = 100_000_000;

fn main() {
    let mut nums: Vec<i64> = Vec::with_capacity(N);
    let mut s: i64 = 0;
    for _ in 0..N {
        nums.push(s);
        s = ((s as i128 * s as i128 + 45) % M as i128) as i64;
    }

    let mut reduced: Vec<i64> = Vec::with_capacity(N);
    let mut sum: i128 = 0;

    for i in 0..N {
        sum += nums[i] as i128;
        reduced.push(nums[i]);
        let mut idx = reduced.len();
        while idx >= 3
            && reduced[idx - 3] <= reduced[idx - 2]
            && reduced[idx - 2] >= reduced[idx - 1]
        {
            reduced[idx - 3] += reduced[idx - 1] - reduced[idx - 2];
            reduced.truncate(idx - 2);
            idx = reduced.len();
        }
    }

    drop(nums);

    let mut reduced_score: i128 = 0;
    let mut start = 0usize;
    let mut end = reduced.len() as i64 - 1;
    while start as i64 <= end {
        let score;
        if reduced[start] > reduced[end as usize] {
            score = reduced[start];
            start += 1;
        } else {
            score = reduced[end as usize];
            end -= 1;
        }
        let parity = if (start as i64 + end) % 2 == 0 { 1i128 } else { -1i128 };
        reduced_score += parity * score as i128;
    }

    let ans = (sum + reduced_score) / 2;

    println!("{}", ans);
}
