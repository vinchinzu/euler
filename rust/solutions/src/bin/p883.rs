// Project Euler 883
// Lattice point counting with GCD constraints and hexagonal geometry.

fn gcd(mut a: i32, mut b: i32) -> i32 {
    while b != 0 { let t = b; b = a % b; a = t; }
    a
}

fn main() {
    let n = 1_000_000.0f64;
    let mut ans: i64 = 0;
    let two_n = 2_000_000i32;

    for n_val in 1..=two_n {
        let max_md = two_n / n_val;
        if max_md < 1 { break; }
        for md in 1..=max_md {
            let m = n_val + md;
            let r = m % n_val;
            if n_val > 1 && (r == 0 || gcd(r, n_val % r) != 1) { continue; }

            let l = 2.0 * n / (n_val as f64 * md as f64);
            let four_l_sq = 4.0 * l * l;
            let mult: i64 = if m + n_val == 3 { 2 } else { 6 };
            let mod3_flag = (n_val + m) % 3;

            let mut x = 1i64;
            while 3.0 * (x * x) as f64 <= four_l_sq {
                let disc_sq_d = four_l_sq - 3.0 * (x * x) as f64;
                let disc_sq_floor = disc_sq_d.floor() as i64;
                let mut disc = (disc_sq_floor as f64).sqrt() as i64;
                while (disc + 1) * (disc + 1) <= disc_sq_floor { disc += 1; }
                while disc * disc > disc_sq_floor { disc -= 1; }

                let xp1h = (x + 1) / 2;
                let xdh = (x - disc) / 2;
                let mut min_y = xp1h.max(xdh) - 1;
                let max_y_a = 2 * x - 1;
                let max_y_b = (x + disc) / 2;
                let mut max_y = max_y_a.min(max_y_b);

                if mod3_flag > 0 {
                    min_y = (x + min_y) / 3;
                    max_y = (x + max_y) / 3;
                }
                if max_y > min_y {
                    ans += (max_y - min_y) * mult;
                }
                x += 1;
            }
        }
    }
    println!("{}", ans);
}
