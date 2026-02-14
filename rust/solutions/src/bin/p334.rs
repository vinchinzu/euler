// Project Euler 334: Spilling the Beans
// Quadratic potential function for 1D abelian sandpile.

fn main() {
    const NPOS: usize = 1500;

    let mut b = vec![0i64; NPOS + 2];
    let mut t: i64 = 123456;
    for i in 1..=NPOS {
        if t % 2 == 0 {
            t /= 2;
        } else {
            t = (t / 2) ^ 926252;
        }
        b[i] = (t % (1 << 11)) + 1;
    }

    let mut big_b: i64 = 0;
    let mut m: i64 = 0;
    let mut phi_init: i128 = 0;
    for j in 1..=NPOS {
        big_b += b[j];
        m += j as i64 * b[j];
        phi_init += (j as i128) * (j as i128) * (b[j] as i128);
    }

    let half_bb = big_b * (big_b - 1) / 2;
    let s_num = m - half_bb;

    let phi_final: i128;

    if s_num % big_b == 0 {
        let s = s_num / big_b;
        let ss = s as i128;
        let bb = big_b as i128;
        phi_final = bb * ss * ss + ss * bb * (bb - 1) + bb * (bb - 1) * (2 * bb - 1) / 6;
    } else {
        let mut s = s_num / big_b;
        if s_num < 0 && s_num % big_b != 0 {
            s -= 1;
        }

        let g = (big_b + 1) * s + big_b * (big_b + 1) / 2 - m;

        let a = s as i128;
        let e = (s + big_b) as i128;
        let sum_sq = e * (e + 1) * (2 * e + 1) / 6 - (a - 1) * a * (2 * a - 1) / 6;
        phi_final = sum_sq - (g as i128) * (g as i128);
    }

    let total = (phi_final - phi_init) / 2;
    println!("{}", total as i64);
}
