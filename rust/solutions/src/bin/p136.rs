// Project Euler Problem 136: Singleton difference.
//
// Count n < 50,000,000 with exactly 1 solution to x^2 - y^2 - z^2 = n.
// x = a+d, y = a, z = a-d => n = a(4d-a) = u*m where u=a, m=4d-a.
// Constraints: (u+m)%4==0, m>=1, m<=3u-4, u*m<LIMIT.

const LIMIT: usize = 50_000_000;

fn main() {
    let mut counts = vec![0u8; LIMIT];

    for u in 1..LIMIT {
        let max_m_div = (LIMIT - 1) / u;
        let max_m_cond = 3 * u as i64 - 4;
        if max_m_cond < 1 {
            continue;
        }
        let max_m_cond = max_m_cond as usize;
        let max_m = max_m_div.min(max_m_cond);
        if max_m < 1 {
            continue;
        }

        let rem = u % 4;
        let mut first_m = (4 - rem) % 4;
        if first_m == 0 {
            first_m = 4;
        }

        let mut m = first_m;
        while m <= max_m {
            let n = u * m;
            if n >= LIMIT {
                break;
            }
            if counts[n] < 2 {
                counts[n] += 1;
            }
            m += 4;
        }
    }

    let result = counts[1..].iter().filter(|&&c| c == 1).count();
    println!("{}", result);
}
