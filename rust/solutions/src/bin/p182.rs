// Project Euler 182: RSA unconcealed messages.
// φ = 2^5 · 3^3 · 7 · 607. gcd(e-1, p-1) and gcd(e-1, q-1) are table lookups.

fn gcd(mut a: u64, mut b: u64) -> u64 {
    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }
    a
}

fn main() {
    const P: u64 = 1009;
    const Q: u64 = 3643;
    let p1 = P - 1;
    let q1 = Q - 1;
    let phi = p1 * q1;

    let mut gp = vec![0u16; p1 as usize];
    let mut gq = vec![0u16; q1 as usize];
    for i in 0..p1 {
        gp[i as usize] = gcd(i, p1) as u16;
    }
    for i in 0..q1 {
        gq[i as usize] = gcd(i, q1) as u16;
    }

    let mut min_u = i64::MAX;
    let mut sum_e = 0u64;
    let mut e = 3u64;
    while e < phi {
        if e % 3 != 0 && e % 7 != 0 && e % 607 != 0 {
            let u = (1 + gp[((e - 1) % p1) as usize] as i64)
                * (1 + gq[((e - 1) % q1) as usize] as i64);
            if u < min_u {
                min_u = u;
                sum_e = e;
            } else if u == min_u {
                sum_e += e;
            }
        }
        e += 2;
    }
    println!("{}", sum_e);
}
