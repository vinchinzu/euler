// Project Euler 157: Solving the diophantine equation 1/a + 1/b = p/10^n

fn num_divisors(mut num: i64) -> i64 {
    if num <= 1 { return num; }
    let mut result = 1i64;
    let mut i = 2i64;
    while i * i <= num {
        if num % i == 0 {
            let mut exp = 0;
            while num % i == 0 { exp += 1; num /= i; }
            result *= exp + 1;
        }
        i += 1;
    }
    if num > 1 { result *= 2; }
    result
}

fn main() {
    let mut total: i64 = 0;

    for n in 1..=9 {
        let mut count = 0i64;
        let mut pow2 = vec![1i64; n + 1];
        let mut pow5 = vec![1i64; n + 1];
        for i in 1..=n {
            pow2[i] = pow2[i - 1] * 2;
            pow5[i] = pow5[i - 1] * 5;
        }

        // Case 1: m=1, k = 2^a * 5^b
        for a in 0..=n {
            for b in 0..=n {
                let k = pow2[a] * pow5[b];
                let s = pow2[n - a] * pow5[n - b];
                let mk_sum = 1 + k;
                let s_val = s * mk_sum;
                count += num_divisors(s_val);
            }
        }

        // Case 2: m = 2^alpha, k = 5^beta, m <= k
        for alpha in 1..=n {
            for beta in 1..=n {
                let m = pow2[alpha];
                let k = pow5[beta];
                if m > k { continue; }
                let s = pow2[n - alpha] * pow5[n - beta];
                let mk_sum = m + k;
                let s_val = s * mk_sum;
                count += num_divisors(s_val);
            }
        }

        // Case 3: m = 5^beta, k = 2^alpha, m <= k
        for beta in 1..=n {
            for alpha in 1..=n {
                let m = pow5[beta];
                let k = pow2[alpha];
                if m > k { continue; }
                let s = pow2[n - alpha] * pow5[n - beta];
                let mk_sum = m + k;
                let s_val = s * mk_sum;
                count += num_divisors(s_val);
            }
        }

        total += count;
    }

    println!("{}", total);
}
