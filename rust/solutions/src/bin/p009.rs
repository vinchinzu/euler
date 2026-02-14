// Project Euler 9: Pythagorean triplet where a+b+c = 1000

fn main() {
    for a in 1..333 {
        for b in (a + 1)..500 {
            let c = 1000 - a - b;
            if c > b && a * a + b * b == c * c {
                println!("{}", a * b * c);
                return;
            }
        }
    }
}
