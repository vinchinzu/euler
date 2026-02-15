// Project Euler 821 - 123-Separable
// Max elements in {S ∪ 2S ∪ 3S} ∩ {1..N} where S,2S,3S disjoint

fn main() {
    let n: i64 = 10_000_000_000_000_000; // 10^16

    let mut nums: Vec<i64> = Vec::new();
    nums.push(1);
    nums.push(6);
    nums.push(24);
    nums.push(54);
    nums.push(n + 1);

    let mut i = 384i64;
    while i <= n {
        nums.push(i);
        i *= 8;
    }
    i = 243;
    while i <= n {
        nums.push(i);
        i *= 27;
    }

    nums.sort_unstable();

    let mut ans = n;
    for j in 0..nums.len() - 1 {
        let low = n / nums[j + 1];
        let high = n / nums[j];
        let euler_low = low - low / 2 - low / 3 + low / 6;
        let euler_high = high - high / 2 - high / 3 + high / 6;
        ans -= (euler_high - euler_low) * j as i64;
    }

    println!("{}", ans);
}
