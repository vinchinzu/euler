// Project Euler 500: Problem 500!!!
use euler_utils::sieve;
use std::collections::BinaryHeap;
use std::cmp::Reverse;

const NVAL: usize = 500_500;
const MOD_VAL: u64 = 500_500_507;
const SIEVE_LIMIT: usize = 7_800_000;

fn main() {
    let is_p = sieve(SIEVE_LIMIT);
    let mut heap: BinaryHeap<Reverse<u64>> = BinaryHeap::new();

    let mut count = 0;
    for i in 2..=SIEVE_LIMIT {
        if count >= NVAL { break; }
        if is_p[i] {
            heap.push(Reverse(i as u64));
            count += 1;
        }
    }

    let mut ans: u64 = 1;
    for _ in 0..NVAL {
        let Reverse(v) = heap.pop().unwrap();
        ans = ans * (v % MOD_VAL) % MOD_VAL;
        heap.push(Reverse(v * v));
    }

    println!("{ans}");
}
