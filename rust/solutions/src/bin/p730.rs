// Project Euler 730 - Shifted Pythagorean Triples
//
// DFS using Barning matrices on primitive triples.

const N: i64 = 100_000_000;
const K: usize = 100;
const L: usize = 200;

fn gcd(mut a: i64, mut b: i64) -> i64 {
    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }
    a
}

static mut TOTAL_ANS: i64 = 0;

fn helper(used: &mut Vec<Vec<Vec<bool>>>, k: usize, a: i64, b: i64, c: i64) {
    if a + b + c > N {
        return;
    }
    let (a, b) = if a > b { (b, a) } else { (a, b) };

    if (a as usize) < L && (b as usize) < L && k <= K {
        if used[k][a as usize][b as usize] {
            return;
        }
        used[k][a as usize][b as usize] = true;
    }
    unsafe {
        TOTAL_ANS += 1;
    }
    helper(used, k, a - 2 * b + 2 * c, 2 * a - b + 2 * c, 2 * a - 2 * b + 3 * c);
    helper(used, k, a + 2 * b + 2 * c, 2 * a + b + 2 * c, 2 * a + 2 * b + 3 * c);
    if a != b {
        helper(used, k, -a + 2 * b + 2 * c, -2 * a + b + 2 * c, -2 * a + 2 * b + 3 * c);
    }
}

fn main() {
    let mut used = vec![vec![vec![false; L]; L]; K + 1];

    for k in 0..=K {
        for p in 1..L as i64 {
            for q in p..L as i64 {
                let r2 = p * p + q * q + k as i64;
                let mut r = (r2 as f64).sqrt() as i64;
                while r * r < r2 {
                    r += 1;
                }
                while r * r > r2 {
                    r -= 1;
                }
                if r * r == r2 && p + q + r <= N && gcd(gcd(p, q), r) == 1 {
                    helper(&mut used, k, p, q, r);
                }
            }
        }
    }

    println!("{}", unsafe { TOTAL_ANS });
}
