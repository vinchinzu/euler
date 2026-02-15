// Project Euler 793 - Median of Products
// Binary search with two-pointer counting over sorted BBS sequence.

fn main() {
    const N: usize = 1_000_003;

    let mut s = vec![0i64; N];
    s[0] = 290797;
    for i in 1..N {
        s[i] = s[i - 1] * s[i - 1] % 50_515_093;
    }

    s.sort_unstable();

    let mut low: i64 = 0;
    let mut high: i64 = s[N - 1] * s[N - 1];
    let target: i64 = (N as i64 - 1) * N as i64 / 2;

    while low + 1 < high {
        let mid = low + (high - low) / 2;
        let mut rank: i64 = 0;
        let mut row_count = N as i32 - 1;

        for i in 0..N {
            let sv = s[i];
            while row_count >= 0 && sv * s[row_count as usize] >= mid {
                row_count -= 1;
            }
            rank += row_count as i64 + if sv * sv < mid { 0 } else { 1 };
        }

        if rank > target {
            high = mid;
        } else {
            low = mid;
        }
    }

    println!("{}", low);
}
