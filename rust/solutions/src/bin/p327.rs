// Project Euler 327: Rooms of Doom
// M(C, R) = minimum cards to traverse R rooms with capacity C.

fn m_func(c: i64, r: i64) -> i64 {
    if r < c {
        return r + 1;
    }
    let k = m_func(c, r - 1);
    k + (k - 2) / (c - 2) * 2 + 1
}

fn main() {
    let mut ans: i64 = 0;
    for c in 3..=40 {
        ans += m_func(c, 30);
    }
    println!("{}", ans);
}
