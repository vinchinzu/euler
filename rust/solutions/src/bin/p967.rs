// Project Euler 967 - B-trivisible integers
// F(10^18, 120) using DFT with cube roots of unity
// Exact integer arithmetic using Z[omega] representation

use rayon::prelude::*;

fn main() {
    let n: u64 = 1_000_000_000_000_000_000;

    // Primes <= 120, excluding 3
    let small_primes: [u64; 29] = [
        2, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59,
        61, 67, 71, 73, 79, 83, 89, 97, 101, 103, 107, 109, 113,
    ];
    let nsp = small_primes.len();

    // Represent h in Z[omega] as (a, b) meaning a + b*omega where omega^2+omega+1=0
    // Multiplication: (a+b*w)(c+d*w) = (ac-bd) + (ad+bc-bd)*w
    // two_re(h1*h2) = h1a*(2*h2a-h2b) + h1b*(-h2a-h2b)

    // Meet in the middle: split primes into first 15 and last 14
    let half1 = 15;
    let half2 = nsp - half1;
    let n1 = 1usize << half1;
    let n2 = 1usize << half2;

    #[derive(Clone, Copy)]
    struct H1 {
        d: u64,
        ha: i16,
        hb: i16,
    }

    let mut h1 = Vec::with_capacity(n1);
    for mask in 0..n1 {
        let mut d: u64 = 1;
        let mut ha: i64 = 1;
        let mut hb: i64 = 0;
        let mut overflow = false;

        for i in 0..half1 {
            if mask & (1 << i) != 0 {
                let p = small_primes[i];
                if d > n / p {
                    overflow = true;
                    break;
                }
                d *= p;
                let (fa, fb) = if p % 3 == 1 { (-1i64, 1i64) } else { (-2i64, -1i64) };
                let new_ha = ha * fa - hb * fb;
                let new_hb = ha * fb + hb * fa - hb * fb;
                ha = new_ha;
                hb = new_hb;
            }
        }

        if !overflow {
            h1.push(H1 {
                d,
                ha: ha as i16,
                hb: hb as i16,
            });
        }
    }
    h1.sort_unstable_by_key(|e| e.d);

    #[derive(Clone, Copy)]
    struct H2 {
        d: u64,
        c0: i32,
        c1: i32,
    }

    let mut h2s = Vec::with_capacity(n2);
    for mask2 in 0..n2 {
        let mut d2: u64 = 1;
        let mut h2a: i64 = 1;
        let mut h2b: i64 = 0;
        let mut overflow2 = false;

        for i in 0..half2 {
            if mask2 & (1 << i) != 0 {
                let p = small_primes[half1 + i];
                if d2 > n / p {
                    overflow2 = true;
                    break;
                }
                d2 *= p;
                let (fa, fb) = if p % 3 == 1 { (-1i64, 1i64) } else { (-2i64, -1i64) };
                let new_ha = h2a * fa - h2b * fb;
                let new_hb = h2a * fb + h2b * fa - h2b * fb;
                h2a = new_ha;
                h2b = new_hb;
            }
        }

        if overflow2 {
            continue;
        }

        h2s.push(H2 {
            d: d2,
            c0: (2 * h2a - h2b) as i32,
            c1: (-h2a - h2b) as i32,
        });
    }

    let total_2re: i128 = h2s
        .par_iter()
        .map(|h2| {
            let limit = n / h2.d;
            let end = h1.partition_point(|e| e.d <= limit);
            let c0 = h2.c0 as i64;
            let c1 = h2.c1 as i64;
            let mut acc = 0i128;
            // SAFETY: end comes from partition_point on h1
            let slice = unsafe { h1.get_unchecked(..end) };
            for e in slice {
                let two_re = e.ha as i64 * c0 + e.hb as i64 * c1;
                acc += two_re as i128 * (limit / e.d) as i128;
            }
            acc
        })
        .sum();

    let f = (n as i128 + total_2re) / 3;
    println!("{}", f);
}
