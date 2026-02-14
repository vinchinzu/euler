// Project Euler 75: Singular integer right triangles
// Count perimeters L <= 1,500,000 with exactly one Pythagorean triple.
// Generate primitive triples via Euclid's formula: L = 2*m*(m+n), gcd(m,n)=1, m-n odd.

use euler_utils::gcd;

fn main() {
    let limit = 1_500_000usize;
    let mut counts = vec![0u32; limit + 1];
    let m_limit = ((limit / 2) as f64).sqrt() as usize;

    for m in 2..=m_limit {
        for n in 1..m {
            if (m - n) % 2 == 0 { continue; }
            if gcd(m as u64, n as u64) != 1 { continue; }

            let prim_l = 2 * m * (m + n);
            if prim_l > limit { break; }

            let mut k = 1;
            while k * prim_l <= limit {
                counts[k * prim_l] += 1;
                k += 1;
            }
        }
    }

    let answer = counts.iter().filter(|&&c| c == 1).count();
    println!("{answer}");
}
