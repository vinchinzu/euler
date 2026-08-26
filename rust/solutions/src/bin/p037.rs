// Project Euler 037: Truncatable Primes
// Sum of the eleven primes truncatable both left-to-right and right-to-left.

fn is_prime(n: u64) -> bool {
    if n < 2 {
        return false;
    }
    if n < 4 {
        return true;
    }
    if n % 2 == 0 || n % 3 == 0 {
        return false;
    }
    let mut i = 5u64;
    while i * i <= n {
        if n % i == 0 || n % (i + 2) == 0 {
            return false;
        }
        i += 6;
    }
    true
}

fn is_left_truncatable(n: u64) -> bool {
    let mut pow10 = 10u64;
    while pow10 < n {
        if !is_prime(n % pow10) {
            return false;
        }
        pow10 *= 10;
    }
    true
}

fn main() {
    // Right-truncatable primes: append 1,3,7,9 onto a smaller right-truncatable prime.
    let mut layer = vec![2u64, 3, 5, 7];
    let mut sum = 0u64;
    let mut found = 0u32;

    while found < 11 && !layer.is_empty() {
        let mut next = Vec::new();
        for &p in &layer {
            for d in [1u64, 3, 7, 9] {
                let n = p * 10 + d;
                if is_prime(n) {
                    next.push(n);
                    if is_left_truncatable(n) {
                        sum += n;
                        found += 1;
                    }
                }
            }
        }
        layer = next;
    }

    println!("{sum}");
}
