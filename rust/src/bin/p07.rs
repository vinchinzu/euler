fn main() {
    let mut count = 0;
    let mut n = 1;
    loop {
        n += 1;
        if is_prime(n) {
            count += 1;
            if count == 10001 {
                println!("{}", n);
                break;
            }
        }
    }
}

fn is_prime(n: u64) -> bool {
    if n < 2 {
        return false;
    }
    if n % 2 == 0 {
        return n == 2;
    }
    let mut d = 3;
    while d * d <= n {
        if n % d == 0 {
            return false;
        }
        d += 2;
    }
    true
}
