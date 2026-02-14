// Project Euler 281: Pizza Toppings
//
// f(m,n) = number of ways to put m toppings on m*n slices using Burnside's lemma.
// Sum all f(m,n) <= 10^15 for m>=2, n>=1.
// Uses u128 to handle large intermediate values.

fn gcd(mut a: u32, mut b: u32) -> u32 {
    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }
    a
}

fn multinomial(g: u32, cp: u32, m: u32) -> u128 {
    if cp == 0 {
        return 1;
    }

    let mut result: u128 = 1;
    let mut denom_copy = 0u32;
    let mut denom_val = 1u32;

    for i in 1..=g {
        result *= i as u128;

        while denom_copy < m {
            if result % (denom_val as u128) == 0 {
                result /= denom_val as u128;
                denom_val += 1;
                if denom_val > cp {
                    denom_val = 1;
                    denom_copy += 1;
                }
            } else {
                break;
            }
        }
    }

    while denom_copy < m {
        result /= denom_val as u128;
        denom_val += 1;
        if denom_val > cp {
            denom_val = 1;
            denom_copy += 1;
        }
    }

    result
}

fn f_func(m: u32, n: u32) -> u128 {
    let mn = m * n;
    let mut total: u128 = 0;
    for k in 0..mn {
        let g = gcd(k, mn);
        if g % m == 0 {
            let cp = g / m;
            total += multinomial(g, cp, m);
        }
    }
    total / mn as u128
}

fn main() {
    let n_limit: u64 = 1_000_000_000_000_000;
    let mut ans: u64 = 0;

    let mut m = 2u32;
    loop {
        let f1 = f_func(m, 1);
        if f1 > n_limit as u128 {
            break;
        }
        let mut n = 1u32;
        loop {
            let fv = f_func(m, n);
            if fv > n_limit as u128 {
                break;
            }
            ans += fv as u64;
            n += 1;
        }
        m += 1;
    }

    println!("{}", ans);
}
