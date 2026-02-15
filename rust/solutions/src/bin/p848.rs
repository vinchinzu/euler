// Project Euler 848 - Guessing Game
// Rational arithmetic with i128 for precision

fn get_sn(n: i128) -> i128 {
    if n <= 0 { return 0; }
    if n == 1 { return 0; }
    if n == 2 { return 1; }
    if n == 3 { return 3; }

    let val = (n - 1) / 3;
    let mut j = 0;
    let mut tmp = val;
    while tmp > 1 { j += 1; tmp >>= 1; }

    let pow4j: i128 = 1i128 << (2 * j);
    let term1 = 3 * pow4j;

    let pow2j: i128 = 1i128 << j;
    let start_of_range = 3 * pow2j;
    let diff = 3 * pow2j;

    let term2 = (n - start_of_range) * diff;
    term1 + term2
}

fn get_tn(n: i128) -> i128 {
    if n == 1 { return 1; }
    if n == 2 { return 2; }
    if n == 3 { return 3; }

    let val = (n - 1) / 3;
    let mut j = 0;
    let mut tmp = val;
    while tmp > 1 { j += 1; tmp >>= 1; }

    3 * (1i128 << (j + 1))
}

fn main() {
    let mut pow7 = [0i128; 21];
    let mut pow5 = [0i128; 21];
    pow7[0] = 1; pow5[0] = 1;
    for i in 1..=20 {
        pow7[i] = pow7[i - 1] * 7;
        pow5[i] = pow5[i - 1] * 5;
    }

    let mut total_sum: f64 = 0.0;

    for i in 0..=20 {
        let m = pow7[i];
        let s_m = get_sn(m);

        for j in 0..=20 {
            let n = pow5[j];
            let tn = get_tn(n);

            let term: f64 = if m <= tn {
                // Regime 2: p(m,n) = 1 - S_m / (m * n)
                let ratio = s_m as f64 / (m as f64 * n as f64);
                1.0 - ratio
            } else {
                // Regime 1: p(m,n) = C_n / m
                let cn: f64 = if n == 1 { 1.0 }
                    else if n == 2 { 1.5 }
                    else {
                        let s_n = get_sn(n);
                        2.0 * s_n as f64 / n as f64
                    };
                cn / m as f64
            };

            total_sum += term;
        }
    }

    println!("{:.8}", total_sum);
}
