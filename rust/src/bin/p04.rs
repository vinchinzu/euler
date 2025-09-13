fn main() {
    let mut max_pal = 0;
    for i in 100..1000 {
        for j in i..1000 {
            let prod = i * j;
            if prod > max_pal && is_palindrome(prod) {
                max_pal = prod;
            }
        }
    }
    println!("{}", max_pal);
}

fn is_palindrome(n: i32) -> bool {
    let s = n.to_string();
    s.chars().eq(s.chars().rev())
}
