// Project Euler 516 - 5-smooth totients
// Sum of all n <= N where phi(n) is a Hamming number (5-smooth).

use euler_utils::miller_rabin;

const M_VAL: u64 = 1u64 << 32;
const N_VAL: u64 = 1_000_000_000_000;

fn main() {
    // Generate all Hamming numbers (5-smooth) up to N_VAL
    let mut hammings = Vec::new();
    let mut n2 = 1u64;
    while n2 <= N_VAL {
        let mut n3 = n2;
        while n3 <= N_VAL {
            let mut n5 = n3;
            while n5 <= N_VAL {
                hammings.push(n5);
                if n5 > N_VAL / 5 { break; }
                n5 *= 5;
            }
            if n3 > N_VAL / 3 { break; }
            n3 *= 3;
        }
        if n2 > N_VAL / 2 { break; }
        n2 *= 2;
    }
    hammings.sort();

    // Prefix sums mod M_VAL
    let num_hammings = hammings.len();
    let mut prefix = vec![0u64; num_hammings + 1];
    for i in 0..num_hammings {
        prefix[i + 1] = (prefix[i] + (hammings[i] % M_VAL)) % M_VAL;
    }

    // Find good primes: p > 5 with p-1 Hamming
    let mut good_primes: Vec<u64> = hammings.iter()
        .filter_map(|&h| {
            let p = h + 1;
            if p > 5 && p <= N_VAL && miller_rabin(p) { Some(p) } else { None }
        })
        .collect();
    good_primes.sort();
    good_primes.dedup();

    fn bisect_right(arr: &[u64], val: u64) -> usize {
        let mut lo = 0usize;
        let mut hi = arr.len();
        while lo < hi {
            let mid = (lo + hi) / 2;
            if arr[mid] <= val { lo = mid + 1; } else { hi = mid; }
        }
        lo
    }

    let mut global_ans: u64 = 0;

    fn gen_products(
        start: usize, prod: u64,
        good_primes: &[u64], hammings: &[u64], prefix: &[u64],
        ans: &mut u64,
    ) {
        let limit = N_VAL / prod;
        let idx = bisect_right(hammings, limit);
        let sum_h = prefix[idx];
        *ans = (*ans + (prod % M_VAL).wrapping_mul(sum_h) % M_VAL) % M_VAL;

        for i in start..good_primes.len() {
            let p = good_primes[i];
            if p > N_VAL / prod { break; }
            let new_prod = prod * p;
            if new_prod > N_VAL { break; }
            gen_products(i + 1, new_prod, good_primes, hammings, prefix, ans);
        }
    }

    gen_products(0, 1, &good_primes, &hammings, &prefix, &mut global_ans);

    println!("{}", global_ans % M_VAL);
}
