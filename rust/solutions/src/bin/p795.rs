// Project Euler 795 - Alternating GCD Sum
// Multiplicative function via smallest-prime-factor sieve.

const N: usize = 12_345_678;

fn main() {
    // Sieve smallest prime factor
    let mut ff = vec![0u32; N + 1];
    for i in 2..=N {
        if ff[i] == 0 {
            ff[i] = i as u32;
            if (i as u64) * (i as u64) <= N as u64 {
                let mut j = i * i;
                while j <= N {
                    if ff[j] == 0 { ff[j] = i as u32; }
                    j += i;
                }
            }
        }
    }

    // Compute multiplicative function f[n] for even n
    let mut f = vec![0i64; N + 1];
    f[1] = 1;
    for n in 2..=N {
        let mut nn = n;
        let p = ff[nn] as usize;
        let mut e = 0;
        while nn % p == 0 { nn /= p; e += 1; }

        if nn > 1 {
            f[n] = f[nn] * f[n / nn];
        } else {
            // n = p^e
            let mut pe_arr = [0i64; 40];
            pe_arr[0] = 1;
            for i in 1..=e { pe_arr[i] = pe_arr[i - 1] * p as i64; }

            let mut val: i64 = 0;
            for k in 0..=e {
                let count = pe_arr[e - k] - if e - k > 0 { pe_arr[e - k - 1] } else { 0 };
                let sign: i64 = if p == 2 && k == 0 { -1 } else { 1 };
                let power = if 2 * k <= e { pe_arr[2 * k] } else { pe_arr[e] };
                val += count * sign * power;
            }
            f[n] = val;
        }
    }

    let mut ans: i64 = 0;
    for n in 1..=N {
        if n % 2 == 0 {
            ans += f[n];
        } else {
            ans -= n as i64;
        }
    }

    println!("{}", ans);
}
