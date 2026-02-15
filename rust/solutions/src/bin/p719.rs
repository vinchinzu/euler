// Project Euler 719 - Number Splitting
//
// For each i, check if i^2 can be split into parts summing to i.
// Uses recursive digit splitting with mod 9 prefilter.

fn can_make(target: i64, digits: i64) -> bool {
    if target < 0 || digits < target {
        return false;
    }
    if digits == 0 {
        return target == 0;
    }
    let mut pow_val: i64 = 1;
    while pow_val <= digits {
        if can_make(target - digits / pow_val, digits % pow_val) {
            return true;
        }
        pow_val *= 10;
    }
    false
}

fn main() {
    let n: i64 = 1_000_000_000_000; // 10^12
    let max_i: i64 = 1_000_000; // isqrt(10^12)
    let mut ans: i64 = 0;

    for i in 2..=max_i {
        let i_sq = i * i;
        // mod 9 filter
        if i % 9 == i_sq % 9 {
            if can_make(i, i_sq) {
                ans += i_sq;
            }
        }
    }

    println!("{}", ans);
}
