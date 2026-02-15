// Project Euler 700 - Eulercoin
// Two-phase scan: forward for small n, backward using modular inverse.

fn mod_inverse(a: u64, m: u64) -> u64 {
    let mut t = 0i64;
    let mut new_t = 1i64;
    let mut r = m as i64;
    let mut new_r = (a % m) as i64;
    while new_r != 0 {
        let q = r / new_r;
        let tmp_t = new_t;
        new_t = t - q * new_t;
        t = tmp_t;
        let tmp_r = new_r;
        new_r = r - q * new_r;
        r = tmp_r;
    }
    if t < 0 { t += m as i64; }
    t as u64
}

fn main() {
    let n_val: u64 = 1_504_170_715_041_707;
    let m: u64 = 4_503_599_627_370_517;
    let l = (m as f64).sqrt() as u64;

    let mut min_el = m;
    let mut ans: u64 = 0;
    let mut el = n_val;
    for _ in 1..=l {
        if el < min_el {
            min_el = el;
            ans += el;
        }
        el = (el + n_val) % m;
    }

    let mod_inv = mod_inverse(n_val, m);
    let mut min_n = m;
    let mut n_val2 = mod_inv;
    for e in 1..min_el {
        if n_val2 < min_n {
            min_n = n_val2;
            ans += e;
        }
        n_val2 = (n_val2 + mod_inv) % m;
    }

    println!("{}", ans);
}
