// Project Euler 450 - Hypocycloid lattice points

fn gcd_ll(a: i64, b: i64) -> i64 {
    let (mut a, mut b) = (a.abs(), b.abs());
    while b != 0 { let t = b; b = a % b; a = t; }
    a
}

fn isqrt_int(n: i64) -> i32 {
    let mut r = (n as f64).sqrt() as i64;
    while r * r > n { r -= 1; }
    while (r + 1) * (r + 1) <= n { r += 1; }
    r as i32
}

fn main() {
    let big_n: usize = 1_000_000;
    let lim = 2 * big_n;

    let mut phi = vec![0i64; lim + 1];
    for i in 0..=lim { phi[i] = i as i64; }
    for i in 2..=lim {
        if phi[i] == i as i64 {
            for j in (i..=lim).step_by(i) {
                phi[j] = phi[j] / i as i64 * (i as i64 - 1);
            }
        }
    }

    let mut mu = vec![1i32; lim + 1];
    let mut is_prime = vec![true; lim + 1];
    is_prime[0] = false;
    is_prime[1] = false;
    for i in 2..=lim {
        if is_prime[i] {
            for j in (i..=lim).step_by(i) {
                if j != i { is_prime[j] = false; }
                if (j as i64) % ((i as i64) * (i as i64)) == 0 {
                    mu[j] = 0;
                } else {
                    mu[j] = -mu[j];
                }
            }
        }
    }

    let mut ans: i64 = 0;

    for s in 3..=big_n {
        let ns = (big_n / s) as i64;
        let tr_ns = ns * (ns + 1) / 2;
        ans += 2 * phi[s] * tr_ns * s as i64;

        if s % 4 != 0 {
            let mut res: i64 = 0;
            let sq = isqrt_int(s as i64);
            for d in 1..=sq as usize {
                if s % d == 0 {
                    let half = ((s - 1) / 2 / d) as i64;
                    let tr_val = half * (half + 1) / 2;
                    res += d as i64 * tr_val * mu[d] as i64;

                    if d != s / d {
                        let d2 = s / d;
                        let half2 = ((s - 1) / 2 / d2) as i64;
                        let tr_val2 = half2 * (half2 + 1) / 2;
                        res += d2 as i64 * tr_val2 * mu[d2] as i64;
                    }
                }
            }
            let factor = if s % 2 == 0 { 2i64 } else { 1i64 };
            ans -= 2 * factor * tr_ns * res;
        }
    }

    // Pythagorean triples part
    for m in 2..1000i64 {
        for nn in 1..m {
            if (m + nn) % 2 == 0 { continue; }
            if gcd_ll(m, nn) != 1 { continue; }

            let tr_a = m * m - nn * nn;
            let tr_b = 2 * m * nn;
            let cc = m * m + nn * nn;

            let ox_vals = [tr_a, tr_b];
            let oy_vals = [tr_b, tr_a];

            for oi in 0..2 {
                let _order_x = ox_vals[oi];
                let order_y = oy_vals[oi];

                for sx in 0..2 {
                    let sin_val: i64 = if sx == 0 { -ox_vals[oi] } else { ox_vals[oi] };
                    for cx in 0..2 {
                        let cos_val: i64 = if cx == 0 { -order_y } else { order_y };

                        for n in 2..100i64 {
                            let exp = if n - 1 > 2 { n - 1 } else { 2 };
                            let mut common: i64 = 1;
                            let mut overflow = false;
                            for _ in 0..exp {
                                if common > big_n as i64 / cc + 1 { overflow = true; break; }
                                common *= cc;
                            }
                            if overflow || common > big_n as i64 { break; }

                            for mp in 1..n {
                                if gcd_ll(mp, n) != 1 { continue; }

                                for k in 1i64.. {
                                    let r_val = k * mp * common;
                                    let big_r_val = k * (mp + n) * common;
                                    if big_r_val > big_n as i64 { break; }

                                    let cc2 = cc * cc;
                                    let mut t_arr = vec![0i64; (n + 1) as usize];
                                    t_arr[0] = 1;
                                    t_arr[1] = cos_val;
                                    for i in 2..=(n as usize) {
                                        t_arr[i] = 2 * cos_val * t_arr[i - 1] - cc2 * t_arr[i - 2];
                                    }

                                    let mut u_arr = vec![0i64; n as usize];
                                    u_arr[0] = 1;
                                    if n > 1 { u_arr[1] = 2 * cos_val; }
                                    for i in 2..(n as usize) {
                                        u_arr[i] = 2 * cos_val * u_arr[i - 1] - cc2 * u_arr[i - 2];
                                    }

                                    let mut cpnm: i64 = 1;
                                    for _ in 0..(n - mp) { cpnm *= cc; }

                                    let x_cn = (big_r_val - r_val) * cpnm * t_arr[mp as usize]
                                             + r_val * t_arr[n as usize];
                                    let y_cn = (big_r_val - r_val) * cpnm * sin_val * u_arr[(mp - 1) as usize]
                                             - r_val * sin_val * u_arr[(n - 1) as usize];

                                    let mut cpn: i64 = 1;
                                    for _ in 0..n { cpn *= cc; }

                                    if cpn != 0 && x_cn % cpn == 0 && y_cn % cpn == 0 {
                                        let xv = x_cn / cpn;
                                        let yv = y_cn / cpn;
                                        ans += xv.abs() + yv.abs();
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    println!("{}", ans);
}
