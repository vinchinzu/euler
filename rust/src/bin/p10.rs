fn main() {
    let limit = 2_000_000u64;
    let mut sieve = vec![true; limit as usize];
    sieve[0] = false;
    sieve[1] = false;
    let mut p = 2usize;
    while p * p < limit as usize {
        if sieve[p] {
            let mut multiple = p * p;
            while multiple < limit as usize {
                sieve[multiple] = false;
                multiple += p;
            }
        }
        p += 1;
    }
    let sum: u64 = sieve.iter().enumerate().filter(|&(_, &b)| b).map(|(i, _)| i as u64).sum();
    println!("{}", sum);
}
