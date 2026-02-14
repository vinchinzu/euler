// Project Euler 580 - Hilbert Numbers
//
// Count Hilbert squarefree numbers up to N=10^16.
// A Hilbert number is 4k+1. Hilbert squarefree means not divisible by
// square of any Hilbert number other than 1.

fn main() {
    let n: i64 = 10_000_000_000_000_000; // 10^16
    let mut l = (n as f64).sqrt() as i64;
    while l * l > n {
        l -= 1;
    }
    while (l + 1) * (l + 1) <= n {
        l += 1;
    }

    // Sieve smallest prime factor
    let l_usize = l as usize;
    let mut ff = vec![0u32; l_usize + 1];
    for i in 2..=l_usize {
        if ff[i] == 0 {
            ff[i] = i as u32;
            if (i as i64) * (i as i64) <= l {
                let mut j = i * i;
                while j <= l_usize {
                    if ff[j] == 0 {
                        ff[j] = i as u32;
                    }
                    j += i;
                }
            }
        }
    }

    // table[0] = count of 4k+1 primes with no dup
    // table[1] = count of dup 4k+1 primes
    // table[2] = count of 4k+3 primes with no dup
    // table[3] = count of dup 4k+3 primes
    let mut table0 = vec![0i8; l_usize + 1];
    let mut table1 = vec![0i8; l_usize + 1];
    let mut table2 = vec![0i8; l_usize + 1];
    let mut table3 = vec![0i8; l_usize + 1];

    for i in (3..=l_usize).step_by(2) {
        let d = if ff[i] == 0 { i as u32 } else { ff[i] };
        let prev = i / d as usize;
        table0[i] = table0[prev];
        table1[i] = table1[prev];
        table2[i] = table2[prev];
        table3[i] = table3[prev];

        let rem_type = d % 4;
        let is_square = i as u64 % (d as u64 * d as u64) == 0;

        if rem_type == 1 {
            if is_square {
                table0[i] += 1;
            } else {
                table1[i] += 1;
            }
        } else {
            if is_square {
                table2[i] += 1;
            } else {
                table3[i] += 1;
            }
        }
    }

    let mut ans: i64 = 0;
    for i in (1..=l_usize).step_by(2) {
        if table1[i] != 0 {
            continue;
        }

        let r = (table2[i] + table3[i]) as i32;
        let mut hilbert_mu: i64;

        if table3[i] == 0 {
            hilbert_mu = if r % 2 == 0 { 1 } else { -1 };
            hilbert_mu *= (r - 1) as i64;
        } else if table3[i] == 1 {
            hilbert_mu = if (r - 1) % 2 == 0 { 1 } else { -1 };
        } else {
            continue;
        }

        hilbert_mu *= if table0[i] % 2 == 0 { 1 } else { -1 };

        if hilbert_mu == 0 {
            continue;
        }

        let isq = i as i64 * i as i64;
        let q = n / isq;
        let count = (q + 3) / 4;

        ans += hilbert_mu * count;
    }

    println!("{}", ans);
}
