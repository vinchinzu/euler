// Project Euler 886
// Permutation counting with coprimality and parity constraints on numbers up to 34.

const NN: usize = 34;
const MOD: i64 = 83_456_729;

fn gcd(mut a: i32, mut b: i32) -> i32 {
    while b != 0 { let t = b; b = a % b; a = t; }
    a
}

fn main() {
    let l = (NN - 2) / 2;

    // Sieve primes up to NN/2
    let sieve_limit = NN / 2;
    let mut is_p = vec![true; sieve_limit + 1];
    is_p[0] = false;
    if sieve_limit > 0 { is_p[1] = false; }
    for i in 2..=sieve_limit {
        if is_p[i] {
            let mut j = i * i;
            while j <= sieve_limit { is_p[j] = false; j += i; }
        }
    }
    let primes_list: Vec<i32> = (2..=sieve_limit).filter(|&i| is_p[i]).map(|i| i as i32).collect();

    // Max counts (radical equivalence classes)
    let mut max_counts = [0i32; NN + 1];
    for i in 2..=NN {
        let mut num = 1;
        for &p in &primes_list {
            if i as i32 % p == 0 { num *= p; }
        }
        max_counts[num as usize] += 1;
    }

    // Products for encoding
    let mut prods_arr = [0i64; NN + 1];
    let mut prod_val = 1i64;
    for num in 1..=NN {
        prods_arr[num] = prod_val;
        prod_val *= (max_counts[num] + 1) as i64;
    }

    // GCD table
    let mut gcds_table = [[0i32; NN + 1]; NN + 1];
    for i in 0..=NN {
        for j in 0..=NN {
            gcds_table[i][j] = gcd(i as i32, j as i32);
        }
    }

    // Cache
    let cache_size = NN / 2 * prod_val as usize;
    let mut cache_arr = vec![-1i32; cache_size];

    let mut ans = 0i64;

    fn num_perms(
        counts: &mut [i32; NN + 1], encoded_counts: i64, num_remaining: i32, prev: i32,
        cache_arr: &mut [i32], prod_val: i64, prods_arr: &[i64; NN + 1],
        gcds_table: &[[i32; NN + 1]; NN + 1],
    ) -> i64 {
        if num_remaining == 0 { return 1; }
        let key_prev = if prev % 2 == 0 { prev / 2 } else { prev };
        let key = ((key_prev - 1) / 2) as usize * prod_val as usize + encoded_counts as usize;
        if cache_arr[key] != -1 { return cache_arr[key] as i64; }
        let mut result = 0i64;
        for num in 1..=NN {
            if counts[num] > 0 && gcds_table[num][prev as usize] == 1 && num % 2 != prev as usize % 2 {
                counts[num] -= 1;
                result += num_perms(counts, encoded_counts - prods_arr[num], num_remaining - 1, num as i32,
                                    cache_arr, prod_val, prods_arr, gcds_table);
                counts[num] += 1;
            }
        }
        result = ((result % MOD) + MOD) % MOD;
        cache_arr[key] = result as i32;
        result
    }

    fn helper(
        num: usize, counts: &mut [i32; NN + 1], other_counts: &mut [i32; NN + 1],
        encoded_counts: i64, num_used: usize, num_odds: usize,
        l: usize, max_counts: &[i32; NN + 1], prods_arr: &[i64; NN + 1], prod_val: i64,
        cache_arr: &mut [i32], gcds_table: &[[i32; NN + 1]; NN + 1],
        ans: &mut i64,
    ) {
        if num > NN {
            if num_used == l && num_odds == l / 2 {
                for middle_num in 1..=NN {
                    if other_counts[middle_num] > 0 && middle_num % 2 == l % 2 {
                        other_counts[middle_num] -= 1;
                        let other_encoded = prod_val - 1 - prods_arr[middle_num] - encoded_counts;

                        let val1 = num_perms(counts, encoded_counts, l as i32, middle_num as i32,
                                            cache_arr, prod_val, prods_arr, gcds_table);
                        let val2 = num_perms(other_counts, other_encoded, l as i32, middle_num as i32,
                                            cache_arr, prod_val, prods_arr, gcds_table);
                        *ans = (*ans + val1 * val2) % MOD;
                        other_counts[middle_num] += 1;
                    }
                }
            }
            return;
        }
        for count in 0..=max_counts[num] {
            counts[num] += count;
            other_counts[num] -= count;
            helper(num + 1, counts, other_counts,
                   encoded_counts + count as i64 * prods_arr[num],
                   num_used + count as usize,
                   num_odds + if num % 2 == 1 { count as usize } else { 0 },
                   l, max_counts, prods_arr, prod_val, cache_arr, gcds_table, ans);
            counts[num] -= count;
            other_counts[num] += count;
        }
    }

    let mut counts = [0i32; NN + 1];
    let mut other_counts = max_counts;
    helper(0, &mut counts, &mut other_counts, 0, 0, 0, l, &max_counts, &prods_arr, prod_val,
           &mut cache_arr, &gcds_table, &mut ans);

    // Multiply by factorials of max_counts
    let mut factorials = [0i64; NN + 1];
    factorials[0] = 1;
    for i in 1..=NN { factorials[i] = factorials[i - 1] * i as i64 % MOD; }
    for num in 1..=NN {
        ans = ans * factorials[max_counts[num] as usize] % MOD;
    }

    println!("{}", ans);
}
