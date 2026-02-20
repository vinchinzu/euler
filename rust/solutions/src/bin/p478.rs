// Project Euler 478 - Mixtures
// Mobius-based counting with half-plane decomposition.

const N_MAX: usize = 10_000_000;
const P_MOD: i64 = 214_358_881; // 11^8
const PHI_MOD: i64 = 389_743_420; // 2 * phi(11^8)

fn pow_mod(mut base: i64, mut exp: i64, modulus: i64) -> i64 {
    let mut result = 1i64;
    base = base.rem_euclid(modulus);
    while exp > 0 {
        if exp & 1 == 1 {
            result = result * base % modulus;
        }
        base = base * base % modulus;
        exp >>= 1;
    }
    result
}

fn main() {
    let n = N_MAX;

    // Linear sieve for primes, mu, phi
    let mut composite = vec![false; N_MAX + 1];
    let mut mu = vec![0i8; N_MAX + 1];
    let mut phi_arr = vec![0i32; N_MAX + 1];
    let mut primes = Vec::with_capacity(700_000);

    mu[1] = 1;
    phi_arr[1] = 1;

    for i in 2..=N_MAX {
        if !composite[i] {
            primes.push(i);
            mu[i] = -1;
            phi_arr[i] = (i - 1) as i32;
        }
        for &p in &primes {
            let t = i * p;
            if t > N_MAX {
                break;
            }
            composite[t] = true;
            if i % p == 0 {
                mu[t] = 0;
                phi_arr[t] = phi_arr[i] * p as i32;
                break;
            } else {
                mu[t] = -mu[i];
                phi_arr[t] = phi_arr[i] * (p - 1) as i32;
            }
        }
    }

    // Mertens function
    let mut mertens = vec![0i64; N_MAX + 1];
    for i in 1..=N_MAX {
        mertens[i] = mertens[i - 1] + mu[i] as i64;
    }

    // F(s, n): compute using quotient blocks
    let f = |s: usize, n: usize| -> i64 {
        let mut ret = 0i64;
        let mut i = 1i64;
        while i * s as i64 <= n as i64 {
            let j = n as i64 / (n as i64 / i) + 1;
            let t = n as i64 / i;
            let d = t / s as i64;
            let mut g = ((2 * t % PHI_MOD + 2
                - (s as i64 % PHI_MOD) * ((1 + d) % PHI_MOD) % PHI_MOD
                + 2 * PHI_MOD)
                % PHI_MOD)
                * (d % PHI_MOD)
                % PHI_MOD;
            // divide by 2
            if g % 2 == 0 {
                g /= 2;
            } else {
                g = (g + PHI_MOD) / 2;
            }
            let j_idx = if (j - 1) as usize > N_MAX {
                N_MAX
            } else {
                (j - 1) as usize
            };
            ret += g * ((mertens[j_idx] - mertens[(i - 1) as usize]) % PHI_MOD) % PHI_MOD;
            ret %= PHI_MOD;
            i = j;
        }
        (ret % PHI_MOD + PHI_MOD) % PHI_MOD
    };

    // total = |M(n)| mod PHI_MOD
    let mut total = 0i64;
    for i in 1..=n {
        if mu[i] == 0 {
            continue;
        }
        let x = (n / i + 1) as i64;
        let cube = (x % PHI_MOD) * (x % PHI_MOD) % PHI_MOD * (x % PHI_MOD) % PHI_MOD;
        let cube = (cube - 1 + PHI_MOD) % PHI_MOD;
        total = (total + mu[i] as i64 * cube % PHI_MOD + PHI_MOD) % PHI_MOD;
    }
    total = (total % PHI_MOD + PHI_MOD) % PHI_MOD;

    let mut ans = pow_mod(2, total, P_MOD);

    for b in 1..=n {
        let m_val = (6i64 * phi_arr[b] as i64) % P_MOD;
        let f_val = f(b, n);
        let half = (total - 1 + PHI_MOD) % PHI_MOD;
        let half = if half % 2 == 0 {
            half / 2
        } else {
            (half + PHI_MOD) / 2
        };
        let term1 = pow_mod(2, half, P_MOD);
        let term2 = pow_mod(2, (half - f_val + PHI_MOD) % PHI_MOD, P_MOD);
        let diff = (term1 - term2 + P_MOD) % P_MOD;
        ans = (ans - m_val * diff % P_MOD + P_MOD) % P_MOD;
    }

    ans = (ans - 1 + P_MOD) % P_MOD;
    println!("{}", ans);
}
