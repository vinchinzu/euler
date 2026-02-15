// Project Euler 723 - Pythagorean Quadrilaterals
//
// Count Pythagorean quadrilaterals on circles using combinatorics over
// divisors of N.

const NUM_PRIMES: usize = 8;
const PRIMES: [i32; NUM_PRIMES] = [5, 13, 17, 29, 37, 41, 53, 61];
const EXPS: [i32; NUM_PRIMES] = [6, 3, 2, 1, 1, 1, 1, 1];

fn sq(n: i64) -> i64 {
    n * n
}

fn ncr(n: i64, r: i64) -> i64 {
    if r < 0 || r > n {
        return 0;
    }
    let r = r.min(n - r);
    let mut result: i64 = 1;
    for i in 0..r {
        result = result * (n - i) / (i + 1);
    }
    result
}

fn num_factors(exp_arr: &[i32]) -> i64 {
    exp_arr.iter().map(|&e| (e + 1) as i64).product()
}

fn f_from_exps(d_exps: &[i32]) -> i64 {
    let mut active_exps: Vec<i32> = Vec::new();
    for i in 0..NUM_PRIMES {
        if d_exps[i] > 0 {
            active_exps.push(d_exps[i]);
        }
    }

    if active_exps.is_empty() {
        let k_total: i64 = 1;
        return 2 * k_total * sq(2 * k_total - 1) - ncr(2 * k_total, 2);
    }

    let mut axes_sizes: Vec<i32> = active_exps.iter().map(|&e| e + 1).collect();
    let total_combos: i32 = axes_sizes.iter().product();
    let active = active_exps.len();

    let mut result: i64 = 0;

    for combo in 0..total_combos {
        let mut ds: Vec<i32> = vec![0; active];
        let mut tmp = combo;
        for i in 0..active {
            ds[i] = tmp % axes_sizes[i];
            tmp /= axes_sizes[i];
        }

        let k = num_factors(&ds);
        let mut mult: i64 = 1;
        for i in 0..active {
            if ds[i] < active_exps[i] {
                mult *= 2;
            }
        }

        // 45 degree pairs
        result += 4 * mult * ncr(k, 2);

        // Parallel pairs
        let mut k_parallel = k;
        if ds.iter().all(|&d| d % 2 == 0) {
            k_parallel -= 1;
        }
        result += 4 * mult * ncr(k_parallel, 2);
    }

    // Diameter-based quadrilaterals
    let k_total = num_factors(&active_exps);
    result += 2 * k_total * sq(2 * k_total - 1) - ncr(2 * k_total, 2);

    result
}

fn main() {
    let mut total_divisors: i32 = 1;
    for i in 0..NUM_PRIMES {
        total_divisors *= EXPS[i] + 1;
    }

    let mut ans: i64 = 0;
    let mut d_exps = [0i32; NUM_PRIMES];

    for combo in 0..total_divisors {
        let mut tmp = combo;
        for i in 0..NUM_PRIMES {
            d_exps[i] = tmp % (EXPS[i] + 1);
            tmp /= EXPS[i] + 1;
        }
        ans += f_from_exps(&d_exps);
    }

    println!("{}", ans);
}
