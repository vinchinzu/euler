// Project Euler 360: Scary Sphere
use euler_utils::sieve;

const R5: i64 = 9765625; // 5^10
const MAX_N: usize = 19531260;

fn main() {
    // Build primes
    let is_p = sieve(5000);
    let primes: Vec<usize> = (2..5000).filter(|&i| is_p[i]).collect();

    // f_no5[n] and v5[n] and cof[n]
    let mut f_no5 = vec![1i16; MAX_N];
    let mut v5 = vec![0i8; MAX_N];
    let mut cof = vec![0i32; MAX_N];

    for n in 1..MAX_N {
        let mut tmp = n;
        while tmp % 2 == 0 { tmp /= 2; }
        let mut v = 0i8;
        while tmp % 5 == 0 { tmp /= 5; v += 1; }
        v5[n] = v;
        cof[n] = tmp as i32;
    }

    for &p in &primes {
        if p == 2 || p == 5 { continue; }
        let mod4 = p & 3;
        let mut n = p;
        while n < MAX_N {
            if f_no5[n] != 0 && cof[n] % p as i32 == 0 {
                let mut v = 0;
                while cof[n] % p as i32 == 0 { cof[n] /= p as i32; v += 1; }
                if mod4 == 1 {
                    f_no5[n] *= (v + 1) as i16;
                } else if v & 1 != 0 {
                    f_no5[n] = 0;
                }
            }
            n += p;
        }
    }

    for n in 1..MAX_N {
        if f_no5[n] == 0 { continue; }
        if cof[n] > 1 {
            if (cof[n] & 3) == 1 {
                f_no5[n] *= 2;
            } else {
                f_no5[n] = 0;
            }
        }
    }

    let r = R5;
    let mut partial: i128 = 0;
    for x in 1..r {
        let a = (r - x) as usize;
        let b = (r + x) as usize;
        if a >= MAX_N || b >= MAX_N { continue; }
        if f_no5[a] == 0 || f_no5[b] == 0 { continue; }
        let cv = v5[a] as i64 + v5[b] as i64;
        let f_combined = f_no5[a] as i64 * f_no5[b] as i64 * (cv + 1);
        partial += x as i128 * f_combined as i128;
    }
    let s_r5 = 6 * r as i128 + 24 * partial;
    let s_final = 1024 * s_r5;
    println!("{}", s_final as i64);
}
