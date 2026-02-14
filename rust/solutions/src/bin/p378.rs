// Project Euler 378 - Triangle Triples

const N: usize = 60_000_000;
const MOD: i64 = 1_000_000_000_000_000_000;

fn main() {
    // Sieve smallest prime factor
    let mut spf = vec![0u32; N + 2];
    for i in 2..=N + 1 {
        if spf[i] == 0 {
            let mut j = i;
            while j <= N + 1 {
                if spf[j] == 0 {
                    spf[j] = i as u32;
                }
                j += i;
            }
        }
    }

    let count_divisors = |mut n: u32| -> u32 {
        if n <= 1 {
            return 1;
        }
        let mut result = 1u32;
        while n > 1 {
            let p = spf[n as usize];
            let mut e = 0u32;
            while n % p == 0 {
                e += 1;
                n /= p;
            }
            result *= e + 1;
        }
        result
    };

    let dt_func = |n: u32| -> u32 {
        let (mut a, mut b) = (n, n + 1);
        if a % 2 == 0 {
            a /= 2;
        } else {
            b /= 2;
        }
        count_divisors(a) * count_divisors(b)
    };

    let mut dt = vec![0u16; N + 1];
    let mut max_dt: usize = 0;
    for i in 1..=N {
        let d = dt_func(i as u32) as u16;
        dt[i] = d;
        if d as usize > max_dt {
            max_dt = d as usize;
        }
    }

    // First pass: compute right_arr using Fenwick tree
    let tree_size = max_dt + 2;
    let mut bit = vec![0i64; tree_size + 1];

    let bit_add = |bit: &mut Vec<i64>, mut pos: usize, val: i64| {
        pos += 1;
        while pos <= tree_size {
            bit[pos] = (bit[pos] + val) % MOD;
            pos += pos & pos.wrapping_neg();
        }
    };

    let bit_query = |bit: &Vec<i64>, mut pos: usize| -> i64 {
        pos += 1;
        let mut s = 0i64;
        while pos > 0 {
            s += bit[pos];
            pos -= pos & pos.wrapping_neg();
        }
        s % MOD
    };

    let mut right_arr = vec![0i64; N + 1];

    for j in (1..=N).rev() {
        right_arr[j] = if dt[j] > 0 {
            bit_query(&bit, dt[j] as usize - 1)
        } else {
            0
        };
        bit_add(&mut bit, dt[j] as usize, 1);
    }

    // Second pass: compute answer
    bit.iter_mut().for_each(|x| *x = 0);
    let mut answer: i64 = 0;

    for j in 1..=N {
        let left_j = if (dt[j] as usize) < max_dt {
            let total = bit_query(&bit, max_dt);
            let at_or_below = bit_query(&bit, dt[j] as usize);
            (total - at_or_below + MOD) % MOD
        } else {
            0
        };
        answer = (answer + left_j * right_arr[j]) % MOD;
        bit_add(&mut bit, dt[j] as usize, 1);
    }

    println!("{}", answer);
}
