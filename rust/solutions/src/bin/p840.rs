// Project Euler 840 - Arithmetic Derivative Partition Sums
// D(n) arithmetic derivative, B[k], g[n] convolution, S(N) = sum g[1..N]

const NMAX: usize = 50000;
const MOD: i64 = 999676999;

fn main() {
    // SPF sieve
    let mut spf = vec![0u32; NMAX + 1];
    for i in 2..=NMAX {
        if spf[i] == 0 { spf[i] = i as u32; }
        if (i as u64) * (i as u64) <= NMAX as u64 {
            if spf[i] == i as u32 {
                let mut j = i * i;
                while j <= NMAX {
                    if spf[j] == 0 { spf[j] = i as u32; }
                    j += i;
                }
            }
        }
    }

    // Arithmetic derivative
    let mut d_arr = vec![0i64; NMAX + 1];
    d_arr[1] = 1;
    for n in 2..=NMAX {
        let mut m = n;
        let mut deriv: i64 = 0;
        while m > 1 {
            let p = spf[m] as usize;
            let mut e = 0;
            while m % p == 0 { m /= p; e += 1; }
            deriv = (deriv + e as i64 * (n / p) as i64) % MOD;
        }
        d_arr[n] = deriv;
    }

    // B[k] = sum_{d|k} d * D(d)^(k/d)
    let mut b_arr = vec![0i64; NMAX + 1];
    for d in 1..=NMAX {
        let y = d_arr[d] % MOD;
        let mut pow_y = y;
        let mut k = d;
        while k <= NMAX {
            b_arr[k] = (b_arr[k] + (d as i128 * pow_y as i128 % MOD as i128) as i64) % MOD;
            pow_y = (pow_y as i128 * y as i128 % MOD as i128) as i64;
            k += d;
        }
    }

    // Modular inverses
    let mut inv_arr = vec![0i64; NMAX + 1];
    inv_arr[1] = 1;
    for n in 2..=NMAX {
        inv_arr[n] = (MOD - (MOD / n as i64) * inv_arr[(MOD % n as i64) as usize] % MOD) % MOD;
    }

    // g[n]
    let mut g = vec![0i64; NMAX + 1];
    g[0] = 1;
    for n in 1..=NMAX {
        let mut s: i64 = 0;
        for k in 1..=n {
            s = (s + (b_arr[k] as i128 * g[n - k] as i128 % MOD as i128) as i64) % MOD;
        }
        g[n] = s % MOD * inv_arr[n] % MOD;
    }

    let mut total: i64 = 0;
    for n in 1..=NMAX {
        total = (total + g[n]) % MOD;
    }

    println!("{}", total);
}
