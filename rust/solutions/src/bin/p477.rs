// Project Euler 477: Number sequence game

const M: i64 = 1_000_000_007;
const N: usize = 100_000_000;

fn main() {
    // Generate sequence on the fly — no intermediate nums buffer
    let mut reduced: Vec<i64> = Vec::with_capacity(N / 2);
    let mut sum: i128 = 0;
    let mut s: i64 = 0;

    for _ in 0..N {
        sum += s as i128;
        reduced.push(s);
        // SAFETY: we only index within reduced.len()
        let mut idx = reduced.len();
        while idx >= 3 {
            // peak reduction: a <= b >= c  =>  a := a+c-b, drop b,c
            let a = unsafe { *reduced.get_unchecked(idx - 3) };
            let b = unsafe { *reduced.get_unchecked(idx - 2) };
            let c = unsafe { *reduced.get_unchecked(idx - 1) };
            if a <= b && b >= c {
                unsafe {
                    *reduced.get_unchecked_mut(idx - 3) = a + c - b;
                }
                reduced.truncate(idx - 2);
                idx = reduced.len();
            } else {
                break;
            }
        }
        s = ((s as i128 * s as i128 + 45) % M as i128) as i64;
    }

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
        let parity = if (start as i64 + end) % 2 == 0 {
            1i128
        } else {
            -1i128
        };
        reduced_score += parity * score as i128;
    }

    let ans = (sum + reduced_score) / 2;
    println!("{}", ans);
}
