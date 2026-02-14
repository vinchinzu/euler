// Project Euler 462: Permutation of 3-smooth numbers
// Hook length formula on Young diagram of exponents.

fn main() {
    let n: i64 = 1_000_000_000_000_000_000;

    let mut e2_arr = Vec::new();
    let mut e3_arr = Vec::new();

    let mut pow2: i64 = 1;
    for e2 in 0..100 {
        if pow2 > n { break; }
        let mut pow3: i64 = 1;
        for e3 in 0..100 {
            if pow3 > n / pow2 { break; }
            e2_arr.push(e2);
            e3_arr.push(e3);
            if pow3 > n / 3 { break; }
            pow3 *= 3;
        }
        if pow2 > n / 2 { break; }
        pow2 *= 2;
        let _ = e2;
    }

    let npoints = e2_arr.len();

    let mut ans: f64 = 0.0;

    // Add ln(n!)
    for i in 1..=npoints {
        ans += (i as f64).ln();
    }

    // Subtract ln(hook_length) for each cell
    for p in 0..npoints {
        let mut hook = 0i32;
        for q in 0..npoints {
            if (e2_arr[q] == e2_arr[p] && e3_arr[q] >= e3_arr[p])
                || (e3_arr[q] == e3_arr[p] && e2_arr[q] >= e2_arr[p])
            {
                hook += 1;
            }
        }
        ans -= (hook as f64).ln();
    }

    // Convert to log base 10
    ans /= 10.0_f64.ln();

    let exp_part = ans as i32;
    let mut mantissa = 10.0_f64.powf(ans - exp_part as f64);
    let mut exp_final = exp_part;

    if mantissa < 1.0 {
        mantissa *= 10.0;
        exp_final -= 1;
    }

    println!("{:.10}e{}", mantissa, exp_final);
}
