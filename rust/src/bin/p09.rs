fn main() {
    for a in 1..1000 {
        for b in a+1..1000 {
            let c = 1000 - a - b;
            if a * a + b * b == c * c {
                println!("{}", a * b * c);
                return;
            }
            if c <= b { break; }
        }
    }
}
