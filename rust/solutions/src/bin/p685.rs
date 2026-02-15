// Project Euler 685 - Inverse Digit Sum II
// f(n,m) = m-th smallest number with digit sum n. Find S(10000).

const MOD: i64 = 1_000_000_007;

fn power_mod(mut base: i64, mut exp: i64) -> i64 {
    let mut r = 1i64;
    base = ((base % MOD) + MOD) % MOD;
    while exp > 0 {
        if exp & 1 == 1 { r = (r as i128 * base as i128 % MOD as i128) as i64; }
        base = (base as i128 * base as i128 % MOD as i128) as i64;
        exp >>= 1;
    }
    r
}

fn comb_exact(n: i64, r: i32, cap: i64) -> i64 {
    if r < 0 { return 0; }
    if r == 0 { return 1; }
    if n < r as i64 { return 0; }
    let mut r = r;
    if r as i64 > n - r as i64 && n - r as i64 >= 0 && n - r as i64 <= 18 {
        r = (n - r as i64) as i32;
    }
    let mut result = 1i64;
    for i in 0..r {
        if n - i as i64 <= 0 { return 0; }
        if result > cap / (n - i as i64) + 1 { return -1; }
        result *= n - i as i64;
        result /= i as i64 + 1;
        if result > cap { return -1; }
    }
    result
}

fn count_digits(p: i64, k_ll: i64, cap: i64) -> i64 {
    if p == 0 { return if k_ll == 0 { 1 } else { 0 }; }
    if k_ll < 0 || k_ll > 9 * p { return 0; }
    let deficit = 9 * p - k_ll;
    let ek = k_ll.min(deficit);
    if ek <= 9 { return comb_exact(p - 1 + ek, ek as i32, cap); }
    let mut total = 0i64;
    let ek_int = ek as i32;
    let mut j = 0;
    while j <= ek_int / 10 && j as i64 <= p {
        let c1 = comb_exact(p, j, cap);
        if c1 == -1 { if j % 2 == 0 { return -1; } j += 1; continue; }
        let c2 = comb_exact(p - 1 + (ek_int - 10 * j) as i64, ek_int - 10 * j, cap);
        if c2 == -1 { if j % 2 == 0 { return -1; } j += 1; continue; }
        if c1 > cap / (c2 + 1) { if j % 2 == 0 { return -1; } j += 1; continue; }
        if j % 2 == 0 { total += c1 * c2; if total > cap { return -1; } }
        else { total -= c1 * c2; }
        j += 1;
    }
    total
}

fn find_number_small(d: i32, rem_sum: i64, rem_pos: i32, mut m: i64) -> i64 {
    let mut val = d as i64 % MOD;
    let mut cur_sum = rem_sum;
    for pos in 0..rem_pos {
        val = val * 10 % MOD;
        let max_dig = if cur_sum > 9 { 9 } else { cur_sum as i32 };
        for dig in 0..=max_dig {
            let new_sum = cur_sum - dig as i64;
            let new_pos = rem_pos - pos - 1;
            if new_sum > 9 * new_pos as i64 { continue; }
            let cnt = count_digits(new_pos as i64, new_sum, 2_000_000_000_000_000_000);
            if cnt == -1 || m < cnt {
                val = (val + dig as i64) % MOD;
                cur_sum = new_sum;
                break;
            }
            m -= cnt;
        }
    }
    val
}

fn find_deficit(big_d: i64, d: i32, p: i64, deficit: i32, mut m: i64) -> i64 {
    if deficit == 0 {
        let val = ((d as i64 + 1) % MOD * power_mod(10, p) % MOD - 1 + MOD) % MOD;
        return val;
    }
    let mut slot_pos = [0i64; 10];
    let mut slot_def = [0i32; 10];
    let mut num_slots = 0;
    let mut cur_p = p;
    let mut cur_k = deficit;
    let mut offset = 0i64;
    while cur_k > 0 && cur_p > 0 {
        let total_ge1 = comb_exact(cur_p - 2 + cur_k as i64, cur_k - 1, 2_000_000_000_000_000_000);
        if total_ge1 != -1 && m >= total_ge1 {
            m -= total_ge1;
            if cur_k == 1 {
                let mut skip = m;
                if skip > cur_p - 2 { skip = cur_p - 2; }
                m -= skip;
                offset += 1 + skip;
                cur_p -= 1 + skip;
            } else if cur_k == 2 {
                let mut lo = 0i64;
                let mut hi = cur_p - 1 - cur_k as i64;
                if hi < 0 { hi = 0; }
                while lo < hi {
                    let mid = lo + (hi - lo + 1) / 2;
                    let s = (mid + 1) as i128 * (2 * (cur_p - 1) - mid) as i128 / 2;
                    if s <= m as i128 { lo = mid; } else { hi = mid - 1; }
                }
                let s = ((lo + 1) as i128 * (2 * (cur_p - 1) - lo) as i128 / 2) as i64;
                m -= s;
                offset += 1 + lo + 1;
                cur_p -= 1 + lo + 1;
            } else {
                offset += 1;
                cur_p -= 1;
            }
        } else {
            for j in (1..=cur_k).rev() {
                let cnt2 = comb_exact(cur_p - 2 + (cur_k - j) as i64, cur_k - j, 2_000_000_000_000_000_000);
                if cnt2 == -1 || m < cnt2 {
                    slot_pos[num_slots] = offset;
                    slot_def[num_slots] = j;
                    num_slots += 1;
                    offset += 1;
                    cur_p -= 1;
                    cur_k -= j;
                    break;
                }
                m -= cnt2;
            }
        }
    }
    let mut val = ((d as i64 + 1) % MOD * power_mod(10, big_d - 1) % MOD - 1 + MOD) % MOD;
    for i in 0..num_slots {
        let place = big_d - 2 - slot_pos[i];
        if place < 0 { continue; }
        let sub = slot_def[i] as i64 % MOD * power_mod(10, place) % MOD;
        val = (val - sub + MOD) % MOD;
    }
    val
}

fn f_func(s: i64, mut m: i64) -> i64 {
    if s <= 0 { return 0; }
    let big_l;
    let r;
    if s % 9 == 0 { big_l = s / 9; r = 9; } else { big_l = s / 9 + 1; r = (s % 9) as i32; }
    m -= 1;
    for big_d in big_l..=big_l + 5 {
        let d_start = if big_d == big_l { r } else { 1 };
        for d in d_start..=9 {
            let p = big_d - 1;
            let rem_sum = s - d as i64;
            if rem_sum < 0 || rem_sum > 9 * p { continue; }
            let deficit = 9 * p - rem_sum;
            let cnt = if deficit <= 8 {
                comb_exact(p - 1 + deficit, deficit as i32, 2_000_000_000_000_000_000)
            } else {
                count_digits(p, rem_sum, 2_000_000_000_000_000_000)
            };
            if cnt == -1 || m < cnt {
                if deficit <= 8 && p > 50 {
                    return find_deficit(big_d, d, p, deficit as i32, m);
                } else if p <= 50 {
                    return find_number_small(d, rem_sum, p as i32, m);
                } else {
                    return -1;
                }
            }
            m -= cnt;
        }
    }
    -1
}

fn main() {
    let mut total = 0i64;
    for n in 1..=10000i64 {
        let s = n * n * n;
        let m = n * n * n * n;
        total = (total + f_func(s, m)) % MOD;
    }
    println!("{}", total);
}
