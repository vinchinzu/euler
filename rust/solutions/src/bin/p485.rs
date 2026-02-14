// Project Euler 485 - Maximum number of divisors
//
// Divisor sieve + sliding window maximum over windows of size K.

const N: usize = 100_000_000;
const K: usize = 100_000;

fn main() {
    // Divisor count sieve
    let mut divs = vec![0u16; N + 1];
    for i in 1..=N {
        for j in (i..=N).step_by(i) {
            divs[j] += 1;
        }
    }

    // Sliding window maximum using monotonic deque
    let mut deque = Vec::with_capacity(K + 1);
    let mut head: usize = 0;
    let mut sum: u64 = 0;

    for i in 1..=N {
        while deque.len() > head && divs[*deque.last().unwrap()] <= divs[i] {
            deque.pop();
        }
        deque.push(i);

        let left = i as isize - K as isize + 1;
        if left >= 1 {
            while deque[head] < left as usize {
                head += 1;
            }
            sum += divs[deque[head]] as u64;
        }
    }

    println!("{}", sum);
}
