// Project Euler 103: Special Sum Sets
fn is_special_sum_set(set: &[i32]) -> bool {
    let n = set.len();
    let total = 3i32.pow(n as u32);
    for t in 0..total {
        let mut tmp = t;
        let mut sum_b = 0i32;
        let mut sum_c = 0i32;
        let mut size_b = 0;
        let mut size_c = 0;
        for i in 0..n {
            match tmp % 3 {
                1 => { sum_b += set[i]; size_b += 1; }
                2 => { sum_c += set[i]; size_c += 1; }
                _ => {}
            }
            tmp /= 3;
        }
        if size_b == 0 || size_c == 0 { continue; }
        if sum_b == sum_c { return false; }
        if size_b > size_c && sum_b <= sum_c { return false; }
        if size_c > size_b && sum_c <= sum_b { return false; }
    }
    true
}

fn main() {
    let set = [20, 31, 38, 39, 40, 42, 45];
    if is_special_sum_set(&set) {
        let s: String = set.iter().map(|x| x.to_string()).collect();
        println!("{s}");
    }
}
