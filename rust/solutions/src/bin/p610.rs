// Project Euler 610 - Roman Numerals II
// Expected value of random Roman numeral string generation

use std::collections::HashMap;

fn to_roman(mut n: i32) -> String {
    let vals = [1000, 900, 500, 400, 100, 90, 50, 40, 10, 9, 5, 4, 1];
    let syms = ["M", "CM", "D", "CD", "C", "XC", "L", "XL", "X", "IX", "V", "IV", "I"];
    let mut s = String::new();
    for i in 0..vals.len() {
        while n >= vals[i] {
            s.push_str(syms[i]);
            n -= vals[i];
        }
    }
    s
}

fn main() {
    let r = 0.14_f64;
    let re = 1.0 - 7.0 * r;
    let letters = [b'I', b'V', b'X', b'L', b'C', b'D'];

    // Build valid roman numeral lookup (values 0..999 without M)
    let mut valid: HashMap<String, i32> = HashMap::new();
    valid.insert(String::new(), 0);
    for i in 1..1000 {
        let s = to_roman(i);
        if !s.contains('M') {
            valid.insert(s, i);
        }
    }

    // Recursive expected value with memoization
    let mut cache: HashMap<String, f64> = HashMap::new();

    fn get_expected(
        prefix: &str,
        r: f64,
        re: f64,
        letters: &[u8; 6],
        valid: &HashMap<String, i32>,
        cache: &mut HashMap<String, f64>,
    ) -> f64 {
        if let Some(&v) = cache.get(prefix) {
            return v;
        }

        let mut sum_prob = re;
        let v = valid.get(prefix).copied().unwrap_or(-1);
        let mut expected = re * if v >= 0 { v as f64 } else { 0.0 };

        for &ch in letters.iter() {
            let mut new_str = prefix.to_string();
            new_str.push(ch as char);
            if valid.contains_key(&new_str) {
                sum_prob += r;
                expected += r * get_expected(&new_str, r, re, letters, valid, cache);
            }
        }

        let result = if sum_prob > 0.0 { expected / sum_prob } else { 0.0 };
        cache.insert(prefix.to_string(), result);
        result
    }

    let e0 = get_expected("", r, re, &letters, &valid, &mut cache);
    let ans = 1000.0 * r / (1.0 - r) + e0;
    println!("{:.8}", ans);
}
