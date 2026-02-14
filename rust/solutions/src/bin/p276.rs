// Project Euler 276 - Primitive Triangles
//
// Count primitive integer triangles with perimeter <= 10^7.

fn main() {
    let n: usize = 10_000_000;

    // Sieve smallest prime factor + Mobius
    let mut spf = vec![0u32; n + 1];
    for i in 2..=n {
        if spf[i] == 0 {
            let mut j = i;
            while j <= n {
                if spf[j] == 0 {
                    spf[j] = i as u32;
                }
                j += i;
            }
        }
    }

    let mut mobius = vec![0i8; n + 1];
    mobius[1] = 1;
    for i in 2..=n {
        let p = spf[i] as usize;
        let prev = i / p;
        if prev % p == 0 {
            mobius[i] = 0;
        } else {
            mobius[i] = -mobius[prev];
        }
    }

    // Cumulative sum of T(k)
    let mut sum_tri = vec![0i64; n + 1];
    for k in 1..=n {
        let t: i64 = if k % 2 == 0 {
            ((k as i64) * (k as i64) + 24) / 48
        } else {
            let kp3 = k as i64 + 3;
            (kp3 * kp3 + 24) / 48
        };
        sum_tri[k] = sum_tri[k - 1] + t;
    }

    let mut ans: i64 = 0;
    for d in 1..=n {
        if mobius[d] != 0 {
            ans += mobius[d] as i64 * sum_tri[n / d];
        }
    }

    println!("{}", ans);
}
