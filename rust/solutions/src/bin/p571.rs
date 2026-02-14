// Project Euler 571 - Super Pandigital Numbers
//
// Find the sum of the smallest 10 numbers that are pandigital in all bases
// from 2 to 12.

const BASE: usize = 12;
const K: usize = 10;

fn is_pandigital(mut n: i64, base: i64) -> bool {
    let mut used = [false; 16];
    while n > 0 {
        used[(n % base) as usize] = true;
        n /= base;
    }
    for i in 0..base as usize {
        if !used[i] {
            return false;
        }
    }
    true
}

fn helper(index: usize, n: i64, visited: &mut [bool; BASE], count: &mut usize, ans: &mut i64) {
    if index == BASE {
        // Check pandigital in base 11 first (most restrictive after 12)
        if !is_pandigital(n, 11) {
            return;
        }
        for base in 2..BASE as i64 {
            if base == 11 {
                continue;
            }
            if !is_pandigital(n, base) {
                return;
            }
        }
        *count += 1;
        *ans += n;
        return;
    }
    if *count == K {
        return;
    }
    for i in 0..BASE {
        if !visited[i] {
            visited[i] = true;
            helper(index + 1, n * BASE as i64 + i as i64, visited, count, ans);
            visited[i] = false;
            if *count == K {
                return;
            }
        }
    }
}

fn main() {
    let mut count = 0usize;
    let mut ans = 0i64;
    let mut visited = [false; BASE];
    helper(0, 0, &mut visited, &mut count, &mut ans);
    println!("{}", ans);
}
