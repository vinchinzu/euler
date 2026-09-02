// Project Euler 477: Number sequence game

const M: u64 = 1_000_000_007;
const N: usize = 100_000_000;

fn main() {
    // Fixed-size stack array (max depth observed is < 30)
    let mut reduced = [0i64; 64];
    let mut len: usize = 0;
    let mut sum: u64 = 0;
    let mut s: u64 = 0;

    for _ in 0..N {
        sum += s;
        unsafe {
            *reduced.get_unchecked_mut(len) = s as i64;
        }
        len += 1;
        while len >= 3 {
            // peak reduction: a <= b >= c  =>  a := a+c-b, drop b,c
            let a = unsafe { *reduced.get_unchecked(len - 3) };
            let b = unsafe { *reduced.get_unchecked(len - 2) };
            let c = unsafe { *reduced.get_unchecked(len - 1) };
            if a <= b && b >= c {
                unsafe {
                    *reduced.get_unchecked_mut(len - 3) = a + c - b;
                }
                len -= 2;
            } else {
                break;
            }
        }
        s = (s * s + 45) % M;
    }

    let mut reduced_score: i128 = 0;
    let mut start = 0usize;
    let mut end = len as i64 - 1;
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

    let ans = (sum as i128 + reduced_score) / 2;
    println!("{}", ans);
}
