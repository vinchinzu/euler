// Project Euler 403: Lattice points enclosed by parabola and line
fn main() {
    let big_n: i64 = 1_000_000_000_000;
    let big_m: i64 = 100_000_000;
    let l = {
        let mut v = (big_n as f64).sqrt() as i64;
        while (v + 1) * (v + 1) <= big_n { v += 1; }
        while v * v > big_n { v -= 1; }
        v
    };

    let sum_powers = |n: i64, exp: i32, modv: i64| -> i64 {
        if n <= 0 { return 0; }
        let m2 = 2 * modv;
        if exp == 1 {
            let val = (n % m2) as i128 * ((n + 1) % m2) as i128;
            ((val / 2) % modv as i128) as i64
        } else if exp == 3 {
            let half = (n % m2) as i128 * ((n + 1) % m2) as i128;
            let h = ((half / 2) % modv as i128) as i64;
            ((h as i128 * h as i128) % modv as i128) as i64
        } else { 0 }
    };

    let mut ans: i64 = 0;
    let mod6 = 6 * big_m;

    for r in -l..=l {
        let max_s = if r == 0 {
            big_n
        } else {
            let t1 = big_n / r.abs();
            let t2 = big_n - r;
            t1.min(t2)
        };
        let d1 = max_s - r;
        let d2 = l - r;

        let sc1 = sum_powers(d1, 3, mod6);
        let s1_1 = sum_powers(d1, 1, mod6);
        let term1 = ((sc1 + (5 * s1_1) % mod6) % mod6) / 6 + (d1 + 1) % big_m;

        let sc2 = sum_powers(d2, 3, mod6);
        let s1_2 = sum_powers(d2, 1, mod6);
        let term2 = ((sc2 + (5 * s1_2) % mod6) % mod6) / 6 + (d2 + 1) % big_m;

        ans = (ans + (2 * term1 - term2) % big_m + big_m) % big_m;
    }
    ans = ((ans % big_m) + big_m) % big_m;
    println!("{ans}");
}
