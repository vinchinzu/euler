// Project Euler 848 - Guessing Game

#[inline(always)]
fn get_sn(n: i128) -> i128 {
    if n <= 0 { return 0; }
    if n == 1 { return 0; }
    if n == 2 { return 1; }
    if n == 3 { return 3; }

    let val = (n - 1) / 3;
    let j = if val <= 1 { 0 } else { (128 - val.leading_zeros() - 1) as i128 };

    let pow4j: i128 = 1i128 << (2 * j);
    let term1 = 3 * pow4j;

    let pow2j: i128 = 1i128 << j;
    let start_of_range = 3 * pow2j;

    let term2 = (n - start_of_range) * start_of_range;
    term1 + term2
}

#[inline(always)]
fn get_tn(n: i128) -> i128 {
    if n == 1 { return 1; }
    if n == 2 { return 2; }
    if n == 3 { return 3; }

    let val = (n - 1) / 3;
    let j_plus_1 = if val <= 1 { 1 } else { 128 - val.leading_zeros() };

    3 * (1i128 << j_plus_1)
}

fn main() {
    let mut pow7 = [0i128; 21];
    let mut pow5 = [0i128; 21];
    pow7[0] = 1; pow5[0] = 1;
    for i in 1..=20 {
        pow7[i] = pow7[i - 1] * 7;
        pow5[i] = pow5[i - 1] * 5;
    }

    let mut s_m_cache = [0i128; 21];
    let mut tn_cache = [0i128; 21];
    let mut cn_cache = [0.0; 21];

    for i in 0..=20 {
        s_m_cache[i] = get_sn(pow7[i]);
        tn_cache[i] = get_tn(pow5[i]);
        
        let n = pow5[i];
        cn_cache[i] = if n == 1 { 1.0 }
            else if n == 2 { 1.5 }
            else {
                let s_n = get_sn(n);
                2.0 * s_n as f64 / n as f64
            };
    }

    let mut total_sum: f64 = 0.0;

    for i in 0..=20 {
        let m = pow7[i];
        let s_m = s_m_cache[i];
        let m_f64 = m as f64;

        for j in 0..=20 {
            let n = pow5[j];
            let tn = tn_cache[j];

            let term: f64 = if m <= tn {
                let ratio = s_m as f64 / (m_f64 * n as f64);
                1.0 - ratio
            } else {
                cn_cache[j] / m_f64
            };

            total_sum += term;
        }
    }

    println!("{:.8}", total_sum);
}
