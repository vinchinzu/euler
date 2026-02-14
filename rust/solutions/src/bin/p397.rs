// Project Euler 397: Triangle on Parabola
//
// Count integer triangles on y=x^2 with specific properties.

const K_MAX: i64 = 1_000_000;
const N_VAL: i64 = 1_000_000_000;
const SIEVE_MAX: usize = 2_000_001;

fn main() {
    let mut smallest_prime = vec![0u32; SIEVE_MAX];
    for i in 2..SIEVE_MAX {
        if smallest_prime[i] == 0 {
            let mut j = i;
            while j < SIEVE_MAX {
                if smallest_prime[j] == 0 {
                    smallest_prime[j] = i as u32;
                }
                j += i;
            }
        }
    }

    let mut ans: i64 = 0;

    for k in 1..=K_MAX {
        let prod = 2 * k * k;
        let divs = gen_divisors(prod, (2 * k) as u32, &smallest_prime);

        for &d in &divs {
            // Case 1
            {
                let apb = -(k + d);
                let bpc = prod / d + k;
                let mut lo = (apb + 1) / 2;
                let mut hi = (bpc - 1) / 2;
                let lo2 = bpc - N_VAL;
                let hi2 = apb + N_VAL;
                if lo2 > lo { lo = lo2; }
                if hi2 < hi { hi = hi2; }
                if lo <= hi {
                    ans += hi - lo + 1;
                }

                let den = d + 2 * k;
                if prod % den == 0 {
                    let apc = prod / den - k;
                    let s = apb + bpc + apc;
                    if s % 2 == 0 {
                        let a = s / 2 - bpc;
                        let c = s / 2 - apb;
                        if a >= -N_VAL && c <= N_VAL {
                            ans -= 2;
                        }
                    }
                }
            }

            // Case 2
            {
                let apb = k - d;
                let bpc = prod / d - k;
                let mut lo = if bpc >= 0 {
                    bpc / 2 + 1
                } else {
                    (bpc - 1) / 2 + 1
                };
                let lo2 = bpc - N_VAL;
                if lo2 > lo { lo = lo2; }
                let mut hi = apb + N_VAL;
                if N_VAL < hi { hi = N_VAL; }
                if lo <= hi {
                    ans += 2 * (hi - lo + 1);
                }

                if d != 2 * k {
                    let den = 2 * k - d;
                    let check = if den > 0 { prod % den == 0 } else { prod % (-den) == 0 };
                    if check {
                        let apc = k - prod / den;
                        let s = apb + bpc + apc;
                        if s % 2 == 0 {
                            let a = s / 2 - bpc;
                            let b_val = s / 2 - apc;
                            let c = s / 2 - apb;
                            if a >= -N_VAL && c < b_val && b_val <= N_VAL {
                                ans -= 1;
                            }
                        }
                    }
                }
            }
        }
    }

    println!("{}", ans);
}

fn gen_divisors(n: i64, val: u32, smallest_prime: &[u32]) -> Vec<i64> {
    let mut divs = vec![1i64];

    let mut tmp = val;
    while tmp > 1 {
        let p = smallest_prime[tmp as usize];
        while tmp % p == 0 {
            tmp /= p;
        }

        let p64 = p as i64;
        if n % p64 != 0 {
            continue;
        }
        let mut e = 0;
        let mut t = n;
        while t % p64 == 0 {
            e += 1;
            t /= p64;
        }

        let old = divs.len();
        let mut pk = 1i64;
        for _ in 0..e {
            pk *= p64;
            for i in 0..old {
                divs.push(divs[i] * pk);
            }
        }
    }
    divs
}
