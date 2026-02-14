// Project Euler 43: Sub-string divisibility
// Sum of all 0-9 pandigital numbers with the sub-string divisibility property.

fn next_permutation(arr: &mut [u8]) -> bool {
    let n = arr.len();
    if n < 2 {
        return false;
    }
    let mut i = n - 2;
    while arr[i] >= arr[i + 1] {
        if i == 0 {
            return false;
        }
        i -= 1;
    }
    let mut j = n - 1;
    while arr[j] <= arr[i] {
        j -= 1;
    }
    arr.swap(i, j);
    arr[i + 1..].sort_unstable();
    true
}

fn main() {
    let divisors = [2, 3, 5, 7, 11, 13, 17];
    let mut digits: Vec<u8> = (0..=9).collect();
    let mut sum: u64 = 0;

    loop {
        let mut valid = true;
        for (i, &div) in divisors.iter().enumerate() {
            let sub = digits[i + 1] as u64 * 100 + digits[i + 2] as u64 * 10 + digits[i + 3] as u64;
            if sub % div != 0 {
                valid = false;
                break;
            }
        }
        if valid {
            let num: u64 = digits.iter().fold(0u64, |acc, &d| acc * 10 + d as u64);
            sum += num;
        }
        if !next_permutation(&mut digits) {
            break;
        }
    }

    println!("{sum}");
}
