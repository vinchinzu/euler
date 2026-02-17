// Problem 978: Random Walk Skewness
// Ported from python/978.py.

fn skewness_at(t: usize) -> f64 {
    assert!(t >= 2, "skewness undefined for t < 2");

    let mut a0: i128 = 0;
    let mut a1: i128 = 1;
    let mut m0: i128 = 0;
    let mut m1: i128 = 1;

    for _ in 2..=t {
        let na = a1 + a0;
        let nm = m1 + 3 * m0;
        a0 = a1;
        a1 = na;
        m0 = m1;
        m1 = nm;
    }

    let a_t = a1 as f64;
    let m_t = m1 as f64;
    let var = a_t - 1.0;
    let central3 = m_t - 3.0 * a_t + 2.0;
    central3 / var.powf(1.5)
}

fn main() {
    let ans = skewness_at(50);
    println!("{ans:.8}");
}
