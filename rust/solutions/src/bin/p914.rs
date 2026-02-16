// Project Euler 914 - Pythagorean Triangles in Circle
// For a given R, find largest inradius of primitive Pythagorean triangles
// fitting inside (without touching) a circle of radius R.
// Inradius = n*(m-n) for triple parametrized by (m,n).
// Circumradius = c/2 = (m^2+n^2)/2, must be < R.

fn gcd(mut a: i64, mut b: i64) -> i64 {
    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }
    a
}

fn isqrt(n: i64) -> i64 {
    if n <= 0 {
        return 0;
    }
    let mut x = (n as f64).sqrt() as i64;
    while x * x > n {
        x -= 1;
    }
    while (x + 1) * (x + 1) <= n {
        x += 1;
    }
    x
}

fn main() {
    let r: i64 = 1_000_000_000_000_000_000; // 10^18
    let limit = 2 * r;
    let sqrt_r = (r as f64).sqrt();

    let ratio_n = (1.0 - (2.0f64).sqrt() / 2.0).sqrt();
    let n_center = (ratio_n * sqrt_r + 0.5) as i64;

    let mut window = (sqrt_r / 1000.0) as i64;
    if window < 1000 {
        window = 1000;
    }
    let mut max_window = sqrt_r as i64 + 5;
    if max_window < 1000 {
        max_window = 1000;
    }

    let mut best: i64 = 0;
    let mut initial_min: i64;
    let mut initial_max: i64;

    loop {
        let n_min = (n_center - window).max(1);
        let n_max = n_center + window;

        for n in n_min..=n_max {
            let t = limit - n * n - 1;
            if t <= 0 {
                continue;
            }

            let m_max = isqrt(t);
            if m_max <= n {
                continue;
            }

            let mut m = m_max;
            if (m - n) % 2 == 0 {
                m -= 1;
            }

            while m > n {
                if m * m + n * n >= limit {
                    break;
                }
                if gcd(n, m) == 1 {
                    let val = n * (m - n);
                    if val > best {
                        best = val;
                    }
                    break;
                }
                m -= 2;
            }
        }

        initial_min = n_min;
        initial_max = n_max;

        if best > 0 || window >= max_window {
            break;
        }
        window *= 2;
    }

    // Expand search downward
    let mut n = initial_min - 1;
    while n > 0 {
        let t = limit - n * n;
        if t <= 0 {
            break;
        }
        let r_upper = n as f64 * ((t as f64).sqrt() - n as f64);
        if r_upper <= best as f64 + 1.0 {
            break;
        }

        let t_adj = t - 1;
        if t_adj <= 0 {
            n -= 1;
            continue;
        }
        let m_max = isqrt(t_adj);
        if m_max <= n {
            n -= 1;
            continue;
        }

        let mut m = m_max;
        if (m - n) % 2 == 0 {
            m -= 1;
        }

        while m > n {
            if m * m + n * n >= limit {
                break;
            }
            if gcd(n, m) == 1 {
                let val = n * (m - n);
                if val > best {
                    best = val;
                }
                break;
            }
            m -= 2;
        }
        n -= 1;
    }

    // Expand search upward
    let mut n = initial_max + 1;
    loop {
        let t = limit - n * n;
        if t <= 0 {
            break;
        }
        let r_upper = n as f64 * ((t as f64).sqrt() - n as f64);
        if r_upper <= best as f64 + 1.0 {
            break;
        }

        let t_adj = t - 1;
        if t_adj <= 0 {
            n += 1;
            continue;
        }
        let m_max = isqrt(t_adj);
        if m_max <= n {
            n += 1;
            continue;
        }

        let mut m = m_max;
        if (m - n) % 2 == 0 {
            m -= 1;
        }

        while m > n {
            if m * m + n * n >= limit {
                break;
            }
            if gcd(n, m) == 1 {
                let val = n * (m - n);
                if val > best {
                    best = val;
                }
                break;
            }
            m -= 2;
        }
        n += 1;
    }

    println!("{}", best);
}
