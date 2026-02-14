// Project Euler 328: Lowest-cost Search
const N: usize = 200_000;

fn popcount_ll(mut x: i64) -> i32 {
    let mut c = 0;
    while x != 0 { c += (x & 1) as i32; x >>= 1; }
    c
}

fn main() {
    let mut c_arr = vec![0i64; N + 1];
    let mut k: i32 = 0;
    let mut s: i64 = 0;
    let mut right_cost: i64 = 1;
    let mut ans: i64 = 0;

    for n in 2..=N {
        c_arr[n] = n as i64 - 1 + if n >= 2 { c_arr[n - 2] } else { 0 };

        let guess = n as i64 - 2 * ((1i64 << k) + s) - 1;
        if guess > 0 {
            let cost = guess + c_arr[(guess - 1) as usize].max(right_cost);
            if cost < c_arr[n] { c_arr[n] = cost; }
        }

        // Count trailing ones in s
        let mut num_ending_ones = 0;
        let mut tmp = s;
        while tmp & 1 != 0 { num_ending_ones += 1; tmp >>= 1; }

        let next_k;
        let next_s;
        let next_right_cost;

        if num_ending_ones == k {
            next_k = k + 1;
            next_s = if next_k == 1 { 1 } else { 3 };
            next_right_cost = (next_k as i64 + 1) * n as i64
                - ((next_k as i64) << (next_k + 1))
                + next_k as i64
                + if next_k == 1 { -1 } else { 3 };
        } else {
            next_k = k;
            let num_remaining_ones = popcount_ll(s & !((1i64 << num_ending_ones) - 1));
            if num_ending_ones < num_remaining_ones + 3 {
                next_s = s + (1i64 << num_ending_ones);
                next_right_cost = right_cost + (num_ending_ones as i64 - num_remaining_ones as i64) * (1i64 << (num_ending_ones + 1));
            } else {
                next_s = s + (1i64 << (num_remaining_ones + 3));
                next_right_cost = right_cost + 3i64 * (1i64 << (num_ending_ones + 1));
            }
        };

        let next_guess = n as i64 - 2 * ((1i64 << next_k) + next_s) - 1;
        if next_guess > 0 {
            let next_total_cost = next_guess + c_arr[(next_guess - 1) as usize].max(next_right_cost);
            if next_total_cost <= c_arr[n] {
                k = next_k;
                s = next_s;
                right_cost = next_right_cost;
                c_arr[n] = next_total_cost;
            }
        }

        ans += c_arr[n];
        right_cost += k as i64 + 1;
    }

    println!("{ans}");
}
