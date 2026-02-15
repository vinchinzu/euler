// Project Euler 735 - Divisors of 2n^2
//
// Mobius function sieve + counting lattice points.

const LIMIT: usize = 1_000_001;

fn main() {
    let big_n: i64 = 1_000_000_000_000;
    let mut l = (big_n as f64).sqrt() as i64;
    if l * l > big_n { l -= 1; }
    while (l + 1) * (l + 1) <= big_n { l += 1; }

    // Sieve Mobius
    let lim = l as usize;
    let mut mobius = vec![1i32; lim + 1];
    let mut is_prime = vec![true; lim + 1];
    is_prime[0] = false;
    if lim >= 1 { is_prime[1] = false; }

    for i in 2..=lim {
        if is_prime[i] {
            for j in (i..=lim).step_by(i) {
                if j != i { is_prime[j] = false; }
                mobius[j] *= -1;
            }
            let sq = i as u64 * i as u64;
            if sq <= lim as u64 {
                let mut j = sq as usize;
                while j <= lim {
                    mobius[j] = 0;
                    j += sq as usize;
                }
            }
        }
    }

    let isq = |n: i64| n * n;
    let icb = |n: i64| n * n * n;

    let mut ans: i64 = big_n;

    for g in 1..=lim {
        if isq(g as i64) >= big_n { break; }
        if mobius[g] == 0 { continue; }

        let mut t = 0;
        while isq(g as i64) * (1i64 << t) <= big_n {
            let n_val = (big_n / isq(g as i64)) >> t;
            if n_val < 1 { t += 1; continue; }

            let mut res: i64 = 0;

            // x*y*z <= n_val, y > x
            {
                let mut x: i64 = 1;
                while icb(x) <= n_val {
                    let nox = n_val / x;
                    let mut sq_nox = (nox as f64).sqrt() as i64;
                    while sq_nox * sq_nox > nox { sq_nox -= 1; }
                    while (sq_nox + 1) * (sq_nox + 1) <= nox { sq_nox += 1; }
                    let mut y = x + 1;
                    while y <= sq_nox {
                        res += n_val / (x * y) - y;
                        y += 1;
                    }
                    x += 1;
                }

                x = 1;
                while icb(x) <= n_val {
                    let nox = n_val / x;
                    let mut sq_nox = (nox as f64).sqrt() as i64;
                    while sq_nox * sq_nox > nox { sq_nox -= 1; }
                    while (sq_nox + 1) * (sq_nox + 1) <= nox { sq_nox += 1; }
                    let mut z = x + 1;
                    while z <= sq_nox {
                        res += n_val / (x * z) - (z - 1);
                        z += 1;
                    }
                    x += 1;
                }

                let mut z: i64 = 1;
                while icb(z) <= n_val {
                    let noz = n_val / z;
                    let mut sq_noz = (noz as f64).sqrt() as i64;
                    while sq_noz * sq_noz > noz { sq_noz -= 1; }
                    while (sq_noz + 1) * (sq_noz + 1) <= noz { sq_noz += 1; }
                    let mut x = z;
                    while x <= sq_noz {
                        res += n_val / (x * z) - x;
                        x += 1;
                    }
                    z += 1;
                }
            }

            // x*y*z <= n_val, y > 2x
            {
                let mut x: i64 = 1;
                while icb(x) <= n_val {
                    let nox = n_val / x;
                    let mut sq_nox = (nox as f64).sqrt() as i64;
                    while sq_nox * sq_nox > nox { sq_nox -= 1; }
                    while (sq_nox + 1) * (sq_nox + 1) <= nox { sq_nox += 1; }
                    let mut y = 2 * x + 1;
                    while y <= sq_nox {
                        res += n_val / (x * y) - y;
                        y += 1;
                    }
                    x += 1;
                }

                x = 1;
                while icb(x) <= n_val {
                    let nox = n_val / x;
                    let mut sq_nox = (nox as f64).sqrt() as i64;
                    while sq_nox * sq_nox > nox { sq_nox -= 1; }
                    while (sq_nox + 1) * (sq_nox + 1) <= nox { sq_nox += 1; }
                    let mut z = x + 1;
                    while z <= sq_nox {
                        if 2 * z * isq(x) > n_val { break; }
                        let maxv = (2 * x).max(z - 1);
                        res += n_val / (x * z) - maxv;
                        z += 1;
                    }
                    x += 1;
                }

                let mut z: i64 = 1;
                while icb(z) <= n_val {
                    let mut x = z;
                    while 2 * z * isq(x) <= n_val {
                        res += n_val / (x * z) - 2 * x;
                        x += 1;
                    }
                    z += 1;
                }
            }

            let parity = if t % 2 == 0 { 1i64 } else { -1 };
            ans += res * parity * mobius[g] as i64;
            t += 1;
        }
    }

    println!("{}", ans);
}
