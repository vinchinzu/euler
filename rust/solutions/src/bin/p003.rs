// Project Euler 3: Largest prime factor of 600851475143

fn main() {
    let mut n: u64 = 600851475143;
    let mut d: u64 = 2;

    while d * d <= n {
        while n % d == 0 {
            n /= d;
        }
        d += 1;
    }

    println!("{n}");
}
