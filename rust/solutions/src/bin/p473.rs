// Project Euler 473: Phigital palindromes
// Recursive enumeration of base-phi palindrome representations.

const N: i64 = 10_000_000_000;
const PHI: f64 = 1.6180339887498948482;

fn helper(n: i64, min_e: i32, ans: &mut i64) {
    if n > N { return; }
    *ans += n;

    let mut e = min_e;
    loop {
        let val = PHI.powi(e) + PHI.powi(e + 3)
                + PHI.powi(-(e + 1)) + PHI.powi(-(e + 4));
        let increment = val.round() as i64;
        let new_n = n + increment;
        if new_n > N { break; }
        helper(new_n, e + 6, ans);
        e += 2;
    }
}

fn main() {
    let mut ans: i64 = 1; // 1 is trivially palindromic
    helper(0, 2, &mut ans);
    helper(2, 4, &mut ans);
    println!("{}", ans);
}
