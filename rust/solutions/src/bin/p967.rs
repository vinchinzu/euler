// Project Euler 967 - B-trivisible integers
// F(10^18, 120) using DFT with cube roots of unity
// Exact integer arithmetic using Z[omega] representation

fn main() {
    let n: i64 = 1_000_000_000_000_000_000;

    // Primes <= 120, excluding 3
    let small_primes: Vec<i64> = vec![
        2, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59,
        61, 67, 71, 73, 79, 83, 89, 97, 101, 103, 107, 109, 113,
    ];
    let nsp = small_primes.len();
    let mod3: Vec<i64> = small_primes.iter().map(|&p| p % 3).collect();

    // Represent h in Z[omega] as (a, b) meaning a + b*omega where omega^2+omega+1=0
    // Multiplication: (a+b*w)(c+d*w) = (ac-bd) + (ad+bc-bd)*w

    // Meet in the middle: split primes into first 15 and last 14
    let half1 = 15;
    let half2 = nsp - half1;
    let n1 = 1usize << half1;
    let n2 = 1usize << half2;

    let mut d1_arr = vec![0i64; n1];
    let mut h1a_arr = vec![0i64; n1];
    let mut h1b_arr = vec![0i64; n1];

    for mask in 0..n1 {
        let mut d: i64 = 1;
        let mut ha: i64 = 1;
        let mut hb: i64 = 0;
        let mut overflow = false;

        for i in 0..half1 {
            if mask & (1 << i) != 0 {
                if d > n / small_primes[i] { overflow = true; break; }
                d *= small_primes[i];
                let (fa, fb) = if mod3[i] == 1 { (-1i64, 1i64) } else { (-2i64, -1i64) };
                let new_ha = ha * fa - hb * fb;
                let new_hb = ha * fb + hb * fa - hb * fb;
                ha = new_ha;
                hb = new_hb;
            }
        }

        d1_arr[mask] = if overflow { n + 1 } else { d };
        h1a_arr[mask] = ha;
        h1b_arr[mask] = hb;
    }

    let mut total_2re: i128 = 0;

    for mask2 in 0..n2 {
        let mut d2: i64 = 1;
        let mut h2a: i64 = 1;
        let mut h2b: i64 = 0;
        let mut overflow2 = false;

        for i in 0..half2 {
            if mask2 & (1 << i) != 0 {
                let pi = half1 + i;
                if d2 > n / small_primes[pi] { overflow2 = true; break; }
                d2 *= small_primes[pi];
                let (fa, fb) = if mod3[pi] == 1 { (-1i64, 1i64) } else { (-2i64, -1i64) };
                let new_ha = h2a * fa - h2b * fb;
                let new_hb = h2a * fb + h2b * fa - h2b * fb;
                h2a = new_ha;
                h2b = new_hb;
            }
        }

        if overflow2 { continue; }

        for mask1 in 0..n1 {
            let d1 = d1_arr[mask1];
            if d1 > n / d2 { continue; }

            let dd = d1 * d2;
            let nd = n / dd;
            if nd == 0 { continue; }

            let ha = h1a_arr[mask1] as i128;
            let hb = h1b_arr[mask1] as i128;
            let h2a = h2a as i128;
            let h2b = h2b as i128;

            let ra = ha * h2a - hb * h2b;
            let rb = ha * h2b + hb * h2a - hb * h2b;
            let two_re = 2 * ra - rb;

            total_2re += two_re * nd as i128;
        }
    }

    let f = (n as i128 + total_2re) / 3;
    println!("{}", f);
}
