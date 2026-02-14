// Project Euler 549 - Divisibility of Factorials
//
// s(n) = smallest m such that n | m!. Find sum_{i=2}^{10^8} s(i).
// Uses recursive enumeration with Lucy_Hedgehog for prime-sum lookups.

const N_VAL: i64 = 100_000_000;

fn main() {
    let limit = 2 * (N_VAL as f64).sqrt() as usize + 10;

    // Sieve primes
    let mut is_p = vec![false; limit + 1];
    for i in 2..=limit { is_p[i] = true; }
    let mut i = 2;
    while i * i <= limit {
        if is_p[i] {
            let mut j = i * i;
            while j <= limit { is_p[j] = false; j += i; }
        }
        i += 1;
    }
    let primes: Vec<i32> = (2..=limit).filter(|&i| is_p[i]).map(|i| i as i32).collect();

    // Lucy Hedgehog
    let sqrt_n = (N_VAL as f64).sqrt() as usize;
    let mut small_arr = vec![0i64; sqrt_n + 2];
    let mut large_arr = vec![0i64; sqrt_n + 2];

    for i in 1..=sqrt_n {
        small_arr[i] = i as i64 * (i as i64 + 1) / 2 - 1;
    }
    for i in 1..=sqrt_n {
        let v = N_VAL / i as i64;
        large_arr[i] = v * (v + 1) / 2 - 1;
    }

    for p in 2..=sqrt_n {
        if small_arr[p] == small_arr[p - 1] { continue; }
        let sp1 = small_arr[p - 1];
        let p2 = (p as i64) * (p as i64);
        let p64 = p as i64;

        let upper = sqrt_n.min((N_VAL / p2) as usize);
        for i in 1..=upper {
            let ip = i as i64 * p64;
            if ip <= sqrt_n as i64 {
                large_arr[i] -= p64 * (large_arr[ip as usize] - sp1);
            } else {
                large_arr[i] -= p64 * (small_arr[(N_VAL / ip) as usize] - sp1);
            }
        }

        let limit2 = sqrt_n.min((N_VAL / p2) as usize);
        for i in (p2 as usize..=limit2).rev() {
            small_arr[i] -= p64 * (small_arr[i / p] - sp1);
        }
    }

    fn num_factors_in_factorial(m: i32, p: i32) -> i32 {
        let mut count = 0;
        let mut power = p as i64;
        while power <= m as i64 {
            count += m / power as i32;
            power *= p as i64;
        }
        count
    }

    // Recursive helper with explicit stack to avoid stack overflow
    struct State<'a> {
        primes: &'a [i32],
        ans: i64,
        sqrt_n: usize,
        small_arr: &'a [i64],
        large_arr: &'a [i64],
    }

    impl<'a> State<'a> {
        fn sum_primes_up_to(&self, x: i64) -> i64 {
            if x <= 0 { return 0; }
            if x <= self.sqrt_n as i64 { return self.small_arr[x as usize]; }
            let k = N_VAL / x;
            self.large_arr[k as usize]
        }

        fn helper(&mut self, min_index: usize, n: i64, s: i32) {
            if n > 1 {
                self.ans += s as i64;
            }

            for index in min_index..self.primes.len() {
                let p = self.primes[index];
                if p as i64 > s as i64 && n * (p as i64) * (p as i64) > N_VAL {
                    // Only single-prime factors remain
                    if p as i64 <= N_VAL / n {
                        let sp_upper = self.sum_primes_up_to(N_VAL / n);
                        let sp_lower = if index > 0 {
                            self.sum_primes_up_to(self.primes[index - 1] as i64)
                        } else {
                            0
                        };
                        self.ans += sp_upper - sp_lower;
                    }
                    return;
                }

                let mut new_n = n;
                let mut e = 1;
                loop {
                    new_n *= p as i64;
                    if new_n > N_VAL { break; }

                    let mut mult = p;
                    loop {
                        if num_factors_in_factorial(mult, p) >= e {
                            let new_s = if mult > s { mult } else { s };
                            self.helper(index + 1, new_n, new_s);
                            break;
                        }
                        mult += p;
                    }
                    e += 1;
                }
            }
        }
    }

    let mut state = State {
        primes: &primes,
        ans: 0,
        sqrt_n,
        small_arr: &small_arr,
        large_arr: &large_arr,
    };

    state.helper(0, 1, 0);

    println!("{}", state.ans);
}
