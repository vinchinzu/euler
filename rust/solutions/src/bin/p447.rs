// Project Euler 447: Retractions C
// F(N) = sum_{g=1}^{sqrt(N)} g*mu(g) * sum_floor_quotients(N/g^2) - N*(N+1)/2

const BIG_N: i64 = 100_000_000_000_000; // 10^14
const MOD: i64 = 1_000_000_007;

fn sum_floor_quotients(m: i64) -> i64 {
    if m <= 0 { return 0; }

    let mut result: i64 = 0;
    let mut sqrt_m = (m as f64).sqrt() as i64;
    while (sqrt_m + 1) * (sqrt_m + 1) <= m { sqrt_m += 1; }
    while sqrt_m * sqrt_m > m { sqrt_m -= 1; }

    for x in 1..=sqrt_m {
        result = (result + (x % MOD) * ((m / x) % MOD)) % MOD;
    }

    for q in 1..=sqrt_m {
        let x_lo_raw = m / (q + 1) + 1;
        let x_hi = m / q;
        let x_lo = if x_lo_raw <= sqrt_m { sqrt_m + 1 } else { x_lo_raw };
        if x_hi >= x_lo {
            let sum_x = ((x_hi as i128 * (x_hi as i128 + 1) / 2
                - x_lo as i128 * (x_lo as i128 - 1) / 2) % MOD as i128 + MOD as i128) % MOD as i128;
            result = (result + sum_x as i64 * (q % MOD)) % MOD;
        }
    }

    result
}

fn tr(n: i64) -> i64 {
    let n_mod = n % MOD;
    let np1_mod = (n + 1) % MOD;
    let inv2 = (MOD + 1) / 2;
    (n_mod as i128 * np1_mod as i128 % MOD as i128 * inv2 as i128 % MOD as i128) as i64
}

fn main() {
    let l = (BIG_N as f64).sqrt() as usize + 1;

    let mut mu = vec![0i8; l + 1];
    let mut spf = vec![0u32; l + 1];
    for i in 0..=l { spf[i] = i as u32; }

    mu[1] = 1;
    for i in 2..=l {
        if spf[i] == i as u32 {
            mu[i] = -1;
            let mut j = i as u64 * i as u64;
            while j <= l as u64 {
                if spf[j as usize] == j as u32 { spf[j as usize] = i as u32; }
                j += i as u64;
            }
        } else {
            let p = spf[i] as usize;
            let q = i / p;
            if q % p == 0 {
                mu[i] = 0;
            } else {
                mu[i] = -mu[q];
            }
        }
    }

    let mut ans: i64 = 0;
    for g in 1..=l {
        if mu[g] != 0 {
            let m = BIG_N / (g as i64 * g as i64);
            if m > 0 {
                let sfq = sum_floor_quotients(m);
                let contribution = ((g as i128 * mu[g] as i128 * sfq as i128) % MOD as i128 + MOD as i128) % MOD as i128;
                ans = (ans + contribution as i64) % MOD;
            }
        }
    }

    ans = (ans - tr(BIG_N)) % MOD;
    if ans < 0 { ans += MOD; }

    println!("{ans}");
}
