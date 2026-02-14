// Project Euler 370: Geometric triangles
// Counts lattice triangles with specific properties.

fn main() {
    let big_l: i64 = 25_000_000_000_000;
    let maxq = 2_900_000usize;

    // Smallest prime factor sieve
    let mut spf = vec![0u32; maxq + 1];
    for i in 2..=maxq {
        if spf[i] == 0 {
            let mut j = i;
            while j <= maxq {
                if spf[j] == 0 { spf[j] = i as u32; }
                j += i;
            }
        }
    }

    fn get_prime_factors(mut q: usize, spf: &[u32]) -> Vec<i64> {
        let mut factors = Vec::new();
        while q > 1 {
            let p = spf[q] as usize;
            factors.push(p as i64);
            while q % p == 0 { q /= p; }
        }
        factors
    }

    fn sum_floor(big_l: i64, a: i64, b: i64, c: i64, mut r_max: i64) -> i64 {
        if r_max <= 0 { return 0; }
        let d1 = a + b + c;
        if d1 > big_l { return 0; }

        {
            let dr = a + b * r_max + c * r_max * r_max;
            if dr > big_l {
                let disc = (b as f64) * (b as f64) - 4.0 * (c as f64) * ((a - big_l) as f64);
                if disc < 0.0 { return 0; }
                r_max = ((-b as f64 + disc.sqrt()) / (2.0 * c as f64)) as i64;
                while r_max + 1 >= 1 && a + b * (r_max + 1) + c * (r_max + 1) * (r_max + 1) <= big_l {
                    r_max += 1;
                }
                while r_max >= 1 && a + b * r_max + c * r_max * r_max > big_l {
                    r_max -= 1;
                }
                if r_max <= 0 { return 0; }
            }
        }

        let dr = a + b * r_max + c * r_max * r_max;
        let t_max = big_l / d1;
        let t_min = big_l / dr;

        if t_max - t_min + 1 < r_max {
            let mut sum: i64 = 0;
            let mut prev_r: i64 = 0;
            let mut t = t_max;
            while t >= t_min && t >= 1 {
                let dt = big_l / t;
                let disc = (b as f64) * (b as f64) - 4.0 * (c as f64) * ((a - dt) as f64);
                if disc < 0.0 { t -= 1; continue; }
                let mut r_up = ((-b as f64 + disc.sqrt()) / (2.0 * c as f64)) as i64;
                while r_up + 1 <= r_max && a + b * (r_up + 1) + c * (r_up + 1) * (r_up + 1) <= dt {
                    r_up += 1;
                }
                while r_up >= 1 && a + b * r_up + c * r_up * r_up > dt {
                    r_up -= 1;
                }
                if r_up > r_max { r_up = r_max; }
                if r_up <= prev_r { t -= 1; continue; }
                sum += t * (r_up - prev_r);
                prev_r = r_up;
                if prev_r >= r_max { break; }
                t -= 1;
            }
            sum
        } else {
            let mut sum: i64 = 0;
            for rp in 1..=r_max {
                let d = a + b * rp + c * rp * rp;
                if d > big_l { break; }
                sum += big_l / d;
            }
            sum
        }
    }

    let mut total = big_l / 3;
    let mut q_max = ((big_l as f64 / 3.0).sqrt() as i64) + 2;
    while 3 * q_max * q_max + 3 * q_max + 1 > big_l { q_max -= 1; }
    if q_max > maxq as i64 { q_max = maxq as i64; }

    for q in 1..=q_max {
        if 3 * q * q + 3 * q + 1 > big_l { break; }
        let factors = get_prime_factors(q as usize, &spf);
        let nf = factors.len();
        let mut max_r = (q as f64 * 0.6180339887498949) as i64;
        while max_r >= 1 && q * q <= q * max_r + max_r * max_r { max_r -= 1; }
        if max_r < 1 { continue; }

        let nsub = 1usize << nf;
        for mask in 0..nsub {
            let mut sign: i64 = 1;
            let mut d: i64 = 1;
            for i in 0..nf {
                if mask & (1 << i) != 0 {
                    sign = -sign;
                    d *= factors[i];
                }
            }
            let r = max_r / d;
            if r < 1 { continue; }
            let s = sum_floor(big_l, 3 * q * q, 3 * q * d, d * d, r);
            total += sign * s;
        }
    }

    println!("{}", total);
}
