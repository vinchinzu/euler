// Project Euler 273: Sum of Squares
// Find primes p < 150 with p = 1 mod 4. Express each as p = a^2 + b^2.
// Recursively form all products of Gaussian integers, sum min(|x|,|y|).

use rayon::prelude::*;

const SPLIT: usize = 5;

fn helper(idx: usize, x: i128, y: i128, ga: &[i128], gb: &[i128], n: usize, ans: &mut i128) {
    if idx == n {
        if y > 0 {
            let ax = x.abs();
            let ay = y.abs();
            *ans += ax.min(ay);
        }
        return;
    }
    helper(idx + 1, x, y, ga, gb, n, ans);
    let (a, b) = (ga[idx], gb[idx]);
    let nx = x * a - y * b;
    let ny = x * b + y * a;
    helper(idx + 1, nx, ny, ga, gb, n, ans);
    let nx2 = x * a + y * b;
    let ny2 = -x * b + y * a;
    helper(idx + 1, nx2, ny2, ga, gb, n, ans);
}

fn main() {
    let mut is_prime = [true; 150];
    is_prime[0] = false;
    is_prime[1] = false;
    let mut i = 2;
    while i * i < 150 {
        if is_prime[i] {
            let mut j = i * i;
            while j < 150 {
                is_prime[j] = false;
                j += i;
            }
        }
        i += 1;
    }

    let mut primes = Vec::new();
    for i in 2..150 {
        if is_prime[i] && i % 4 == 1 {
            primes.push(i as i128);
        }
    }

    let mut ga = vec![0i128; primes.len()];
    let mut gb = vec![0i128; primes.len()];
    for (k, &p) in primes.iter().enumerate() {
        let mut a = 1i128;
        while a * a < p {
            let rem = p - a * a;
            let b = (rem as f64).sqrt() as i128;
            if b * b == rem {
                ga[k] = a;
                gb[k] = b;
                break;
            }
            a += 1;
        }
    }

    let num_primes = primes.len();
    let n_pref = 3usize.pow(SPLIT as u32);
    let ans: i128 = (0..n_pref)
        .into_par_iter()
        .map(|mut code| {
            let mut x = 1i128;
            let mut y = 0i128;
            for idx in 0..SPLIT {
                let choice = code % 3;
                code /= 3;
                let (a, b) = (ga[idx], gb[idx]);
                match choice {
                    0 => {}
                    1 => {
                        let nx = x * a - y * b;
                        let ny = x * b + y * a;
                        x = nx;
                        y = ny;
                    }
                    _ => {
                        let nx = x * a + y * b;
                        let ny = -x * b + y * a;
                        x = nx;
                        y = ny;
                    }
                }
            }
            let mut local = 0i128;
            helper(SPLIT, x, y, &ga, &gb, num_primes, &mut local);
            local
        })
        .sum();
    println!("{}", ans);
}
