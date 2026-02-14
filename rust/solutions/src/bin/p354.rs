// Project Euler 354: Honeycomb distance distribution
//
// Recursive template-based enumeration of numbers with specific prime structure.

const N_VAL: f64 = 500_000_000_000.0;
const K_VAL: i32 = 450;

static mut ANS: i64 = 0;

fn is_prime_large(n: i64) -> bool {
    if n < 2 { return false; }
    if n == 2 || n == 3 { return true; }
    if n % 2 == 0 || n % 3 == 0 { return false; }
    let mut i = 5i64;
    while i * i <= n {
        if n % i == 0 || n % (i + 2) == 0 { return false; }
        i += 6;
    }
    true
}

fn is_prime(p: i64, spf: &[i32], l2: usize) -> bool {
    if p <= 1 { return false; }
    if p as usize <= l2 { return spf[p as usize] == p as i32; }
    is_prime_large(p)
}

fn find_nums_for_template(
    index: usize, prod_primes: i64, min_prime: i64,
    limit: f64, tmpl: &[i32], spf: &[i32], num_2mod3s: &[i32], l2: usize,
) {
    if index == tmpl.len() {
        let mut remaining = limit;
        while remaining > 1.0 {
            let idx = (remaining.sqrt() as usize).min(l2);
            unsafe { ANS += num_2mod3s[idx] as i64; }
            remaining /= 3.0;
        }
        return;
    }

    let e = tmpl[index];
    let mut p = if min_prime > 1 { min_prime } else { 1 };
    // Advance to p = 1 mod 3
    match p % 3 {
        0 => p += 1,
        2 => p += 2,
        _ => {}
    }

    loop {
        let mut pe = 1.0;
        let mut ok = true;
        for _ in 0..e {
            pe *= p as f64;
            if pe > limit {
                ok = false;
                break;
            }
        }
        if !ok { break; }

        if prod_primes % p != 0 && is_prime(p, spf, l2) {
            let next_min = if index + 1 < tmpl.len() && tmpl[index] == tmpl[index + 1] {
                p + 3
            } else {
                1
            };

            find_nums_for_template(
                index + 1, prod_primes * p, next_min,
                limit / pe, tmpl, spf, num_2mod3s, l2,
            );
        }
        p += 3;
    }
}

fn find_all_templates(
    n: i32, max_d: i32, tmpl: &mut Vec<i32>,
    spf: &[i32], num_2mod3s: &[i32], l2: usize,
) {
    if n == 1 {
        let l1 = N_VAL / 3.0f64.sqrt();
        let l1_sq = l1 * l1;
        find_nums_for_template(0, 1, 1, l1_sq, tmpl, spf, num_2mod3s, l2);
        return;
    }
    for d in 2..=max_d {
        if n % d == 0 {
            tmpl.push(d - 1);
            find_all_templates(n / d, d, tmpl, spf, num_2mod3s, l2);
            tmpl.pop();
        }
    }
}

fn main() {
    let l1 = N_VAL / 3.0f64.sqrt();
    let l1_sq = l1 * l1;
    let l2 = (l1_sq / (7.0f64.powi(4) * 13.0f64.powi(4))).sqrt() as usize;

    let mut spf = vec![0i32; l2 + 1];
    for i in 0..=l2 {
        spf[i] = i as i32;
    }
    {
        let mut i = 2;
        while i * i <= l2 {
            if spf[i] == i as i32 {
                let mut j = i * i;
                while j <= l2 {
                    if spf[j] == j as i32 {
                        spf[j] = i as i32;
                    }
                    j += i;
                }
            }
            i += 1;
        }
    }

    let mut num_2mod3s = vec![0i32; l2 + 1];
    for n in 1..=l2 {
        let mut ok = true;
        let mut temp = n;
        while temp > 1 {
            let p = spf[temp] as usize;
            if p % 3 != 2 {
                ok = false;
                break;
            }
            while temp % p == 0 {
                temp /= p;
            }
        }
        num_2mod3s[n] = num_2mod3s[n - 1] + if ok { 1 } else { 0 };
    }

    let mut tmpl = Vec::new();
    find_all_templates(K_VAL / 6, K_VAL / 6, &mut tmpl, &spf, &num_2mod3s, l2);

    println!("{}", unsafe { ANS });
}
