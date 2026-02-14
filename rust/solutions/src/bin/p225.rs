fn main() {
    let target = 124;
    let mut d = 1;
    let mut count = 0;

    while count < target {
        d += 2;
        let (mut a, mut b, mut c) = (1i32, 1i32, 1i32);
        let mut found_zero = false;

        loop {
            let next = (a + b + c) % d;
            if next == 0 {
                found_zero = true;
                break;
            }
            a = b;
            b = c;
            c = next;
            if a == 1 && b == 1 && c == 1 {
                break;
            }
        }

        if !found_zero {
            count += 1;
        }
    }
    println!("{}", d);
}
