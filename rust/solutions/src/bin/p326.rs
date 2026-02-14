// Project Euler 326
fn main() {
    let n: i64 = 1_000_000_000_000;
    let m_val: i64 = 1_000_000;
    let period = 6 * m_val;

    // Build a[0..period-1]
    let mut a = vec![0i32; period as usize];
    for i in 0..m_val as usize {
        a[6 * i]     = (3 * i) as i32;
        a[6 * i + 1] = (4 * i + 1) as i32;
        a[6 * i + 2] = (3 * i + 1) as i32;
        a[6 * i + 3] = i as i32;
        a[6 * i + 4] = (6 * i + 3) as i32;
        a[6 * i + 5] = i as i32;
    }

    // Count occurrences of each S(i) mod M value
    let mut counts = vec![0i64; m_val as usize];
    let mut s: i64 = 0;
    for i in 0..period {
        s = (s + a[i as usize] as i64) % m_val;
        counts[s as usize] += (n - i + period) / period;
    }

    let mut ans: i64 = 0;
    for &c in &counts {
        ans += c * (c - 1) / 2;
    }

    println!("{}", ans);
}
