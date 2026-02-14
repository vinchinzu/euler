// Project Euler 024: Lexicographic Permutations
// Find the millionth lexicographic permutation of digits 0-9.

fn main() {
    let mut available: Vec<u8> = (0..10).collect();
    let mut n: u64 = 1_000_000 - 1; // zero-based
    let mut result = String::with_capacity(10);

    for i in (0..10u64).rev() {
        let fact: u64 = (1..=i).product::<u64>().max(1);
        let idx = (n / fact) as usize;
        n %= fact;
        result.push((b'0' + available.remove(idx)) as char);
    }

    println!("{result}");
}
