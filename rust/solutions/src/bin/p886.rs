// Project Euler 886
// Permutation counting with coprimality and parity constraints on numbers up to 34.
// Optimized: eliminate bounds checks in hot paths, minimize parameter passing via struct.

#![allow(unsafe_op_in_unsafe_fn)]

const NN: usize = 34;
const MOD: i64 = 83_456_729;

fn gcd(mut a: i32, mut b: i32) -> i32 {
    while b != 0 { let t = b; b = a % b; a = t; }
    a
}

struct Ctx {
    cache: *mut i32,
    prod_val: i32,
    prods: [i32; NN + 1],
    gcds: [[i32; NN + 1]; NN + 1],
    max_counts: [i32; NN + 1],
    l: i32,
    ans: i64,
}

unsafe fn num_perms(ctx: &Ctx, counts: &mut [i32; NN + 1], encoded_counts: i32, num_remaining: i32, prev: usize) -> i64 {
    if num_remaining == 0 { return 1; }
    let key_prev = if prev % 2 == 0 { prev / 2 } else { prev };
    let key = (key_prev - 1) / 2 * ctx.prod_val as usize + encoded_counts as usize;
    let cached = *ctx.cache.add(key);
    if cached != -1 { return cached as i64; }
    let mut result = 0i64;
    let prev_parity = prev & 1;
    let gcds_prev = ctx.gcds.get_unchecked(prev);
    for num in 1..=NN {
        if *counts.get_unchecked(num) > 0
            && *gcds_prev.get_unchecked(num) == 1
            && (num & 1) != prev_parity
        {
            *counts.get_unchecked_mut(num) -= 1;
            result += num_perms(
                ctx,
                counts,
                encoded_counts - *ctx.prods.get_unchecked(num),
                num_remaining - 1,
                num,
            );
            *counts.get_unchecked_mut(num) += 1;
        }
    }
    result = ((result % MOD) + MOD) % MOD;
    *ctx.cache.add(key) = result as i32;
    result
}

unsafe fn helper(
    ctx: &mut Ctx,
    num: usize,
    counts: &mut [i32; NN + 1],
    other_counts: &mut [i32; NN + 1],
    encoded_counts: i32,
    num_used: i32,
    num_odds: i32,
) {
    if num > NN {
        if num_used == ctx.l && num_odds == ctx.l / 2 {
            for middle_num in 1..=NN {
                if *other_counts.get_unchecked(middle_num) > 0 && middle_num as i32 % 2 == ctx.l % 2 {
                    *other_counts.get_unchecked_mut(middle_num) -= 1;
                    let other_encoded = ctx.prod_val - 1 - *ctx.prods.get_unchecked(middle_num) - encoded_counts;
                    let val1 = num_perms(ctx, counts, encoded_counts, ctx.l, middle_num);
                    let val2 = num_perms(ctx, other_counts, other_encoded, ctx.l, middle_num);
                    ctx.ans = (ctx.ans + val1 * val2) % MOD;
                    *other_counts.get_unchecked_mut(middle_num) += 1;
                }
            }
        }
        return;
    }
    let mc = *ctx.max_counts.get_unchecked(num);
    for count in 0..=mc {
        *counts.get_unchecked_mut(num) += count;
        *other_counts.get_unchecked_mut(num) -= count;
        helper(
            ctx,
            num + 1,
            counts,
            other_counts,
            encoded_counts + count * *ctx.prods.get_unchecked(num),
            num_used + count,
            num_odds + if num % 2 == 1 { count } else { 0 },
        );
        *counts.get_unchecked_mut(num) -= count;
        *other_counts.get_unchecked_mut(num) += count;
    }
}

fn main() {
    let l = (NN - 2) / 2;

    // Sieve primes up to NN/2
    let sieve_limit = NN / 2;
    let mut is_p = [true; NN + 1];
    is_p[0] = false;
    is_p[1] = false;
    for i in 2..=sieve_limit {
        if is_p[i] {
            let mut j = i * i;
            while j <= sieve_limit { is_p[j] = false; j += i; }
        }
    }
    let primes_list: Vec<i32> = (2..=sieve_limit).filter(|&i| is_p[i]).map(|i| i as i32).collect();

    let mut max_counts = [0i32; NN + 1];
    for i in 2..=NN {
        let mut num = 1;
        for &p in &primes_list {
            if i as i32 % p == 0 { num *= p; }
        }
        max_counts[num as usize] += 1;
    }

    let mut prods = [0i32; NN + 1];
    let mut prod_val: i32 = 1;
    for num in 1..=NN {
        prods[num] = prod_val;
        prod_val *= max_counts[num] + 1;
    }

    let mut gcds = [[0i32; NN + 1]; NN + 1];
    for i in 0..=NN {
        for j in 0..=NN {
            gcds[i][j] = gcd(i as i32, j as i32);
        }
    }

    let cache_size = NN / 2 * prod_val as usize;

    unsafe {
        let layout = std::alloc::Layout::array::<i32>(cache_size).unwrap();
        let cache = std::alloc::alloc(layout) as *mut i32;
        std::ptr::write_bytes(cache, 0xFF, cache_size);

        let mut ctx = Box::new(Ctx {
            cache,
            prod_val,
            prods,
            gcds,
            max_counts,
            l: l as i32,
            ans: 0,
        });

        let mut counts = [0i32; NN + 1];
        let mut other_counts = max_counts;
        helper(&mut ctx, 0, &mut counts, &mut other_counts, 0, 0, 0);

        // Multiply by factorials of max_counts
        let mut factorials = [0i64; NN + 1];
        factorials[0] = 1;
        for i in 1..=NN { factorials[i] = factorials[i - 1] * i as i64 % MOD; }
        for num in 1..=NN {
            ctx.ans = ctx.ans * factorials[max_counts[num] as usize] % MOD;
        }

        println!("{}", ctx.ans);

        std::alloc::dealloc(cache as *mut u8, layout);
    }
}
