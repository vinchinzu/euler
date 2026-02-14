// Project Euler 149: Maximum-sum subsequence in a 2000x2000 table
//
// Generate table with Lagged Fibonacci Generator, then find max sum
// contiguous subsequence in any row, column, or diagonal (Kadane's algorithm).

const N: usize = 2000;
const TOTAL: usize = N * N;
const MODV: i64 = 1_000_000;
const OFFSET: i64 = 500_000;

fn kadane(arr: &[i32]) -> i32 {
    if arr.is_empty() {
        return i32::MIN;
    }
    let mut max_here = arr[0];
    let mut max_so_far = arr[0];
    for &v in &arr[1..] {
        max_here = max_here.max(0) + v;
        max_so_far = max_so_far.max(max_here);
    }
    max_so_far
}

fn main() {
    // Generate sequence
    let mut s = vec![0i32; TOTAL];

    for i in 0..55 {
        let k = (i + 1) as i64;
        let temp = ((100003 - 200003 * k + 300007 * k * k * k) % MODV + MODV) % MODV;
        s[i] = (temp - OFFSET) as i32;
    }
    for i in 55..TOTAL {
        let temp = ((s[i - 24] as i64 + s[i - 55] as i64 + MODV) % MODV) as i32;
        s[i] = temp - OFFSET as i32;
    }

    // Fill table
    let mut table = vec![vec![0i32; N]; N];
    for i in 0..N {
        for j in 0..N {
            table[i][j] = s[i * N + j];
        }
    }

    let mut max_sum = i32::MIN;
    let mut buf = vec![0i32; N];

    // Horizontal
    for i in 0..N {
        let v = kadane(&table[i]);
        max_sum = max_sum.max(v);
    }

    // Vertical
    for j in 0..N {
        for i in 0..N {
            buf[i] = table[i][j];
        }
        let v = kadane(&buf[..N]);
        max_sum = max_sum.max(v);
    }

    // Main diagonals (top-left to bottom-right)
    for r in 0..N {
        let mut len = 0;
        let (mut i, mut j) = (r, 0);
        while i < N && j < N {
            buf[len] = table[i][j];
            len += 1;
            i += 1;
            j += 1;
        }
        let v = kadane(&buf[..len]);
        max_sum = max_sum.max(v);
    }
    for c in 1..N {
        let mut len = 0;
        let (mut i, mut j) = (0, c);
        while i < N && j < N {
            buf[len] = table[i][j];
            len += 1;
            i += 1;
            j += 1;
        }
        let v = kadane(&buf[..len]);
        max_sum = max_sum.max(v);
    }

    // Anti-diagonals (top-right to bottom-left)
    for c in 0..N {
        let mut len = 0;
        let mut i = 0usize;
        let mut j = c as isize;
        while i < N && j >= 0 {
            buf[len] = table[i][j as usize];
            len += 1;
            i += 1;
            j -= 1;
        }
        let v = kadane(&buf[..len]);
        max_sum = max_sum.max(v);
    }
    for r in 1..N {
        let mut len = 0;
        let mut i = r;
        let mut j = (N - 1) as isize;
        while i < N && j >= 0 {
            buf[len] = table[i][j as usize];
            len += 1;
            i += 1;
            j -= 1;
        }
        let v = kadane(&buf[..len]);
        max_sum = max_sum.max(v);
    }

    println!("{}", max_sum);
}
