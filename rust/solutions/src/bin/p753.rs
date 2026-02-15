// Project Euler 753 - Fermat Equation
// Count solutions to x^3 + y^3 = z^3 (mod p) for primes p <= N.

fn main() {
    const N: usize = 6_000_000;
    let mut is_prime = vec![true; N + 1];
    is_prime[0] = false;
    is_prime[1] = false;
    for i in 2..=N {
        if is_prime[i] {
            let mut j = i * i;
            while j <= N {
                is_prime[j] = false;
                j += i;
            }
        }
    }

    let mut ans: u64 = 0;

    // Case 1: p % 3 != 1
    for p in 2..=N {
        if is_prime[p] && p % 3 != 1 {
            ans += (p as u64 - 1) * (p as u64 - 2);
        }
    }

    // Case 2: enumerate L, M with L^2 + 27*M^2 = 4p, L = 1 (mod 3)
    let max_abs_l = ((4.0 * N as f64).sqrt() as i64) + 1;
    for abs_l in 1..=max_abs_l {
        for &l in &[-abs_l, abs_l] {
            let lmod3 = ((l % 3) + 3) % 3;
            if lmod3 != 1 { continue; }

            let m_start = if abs_l % 2 == 0 { 0 } else { 1 };
            let mut m = m_start;
            loop {
                let p_val = (l as i64 * l as i64 + 27 * m as i64 * m as i64) / 4;
                if p_val > N as i64 { break; }
                if p_val >= 2 && is_prime[p_val as usize] {
                    ans += (l + p_val - 8) as u64 * (p_val as u64 - 1);
                }
                m += 2;
            }
        }
    }

    println!("{}", ans);
}
