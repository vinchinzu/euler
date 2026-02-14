// Project Euler 196 - Prime triplets
// Segmented sieve over 5 rows of the triangular grid.

fn tr(n: i64) -> i64 {
    n * (n + 1) / 2
}

fn sum_good_primes(row: i64) -> i64 {
    let start = tr(row - 3) + 1;
    let end = tr(row + 2);

    // Odd-only segmented sieve
    let base = if start % 2 == 0 { start + 1 } else { start };
    let sieve_len = ((end - base) / 2 + 1) as usize;
    let sieve_limit = ((end as f64).sqrt() as i64) + 1;

    // Small primes sieve
    let mut small_sieve = vec![false; (sieve_limit + 1) as usize];
    small_sieve[0] = true;
    small_sieve[1] = true;
    {
        let mut i = 2i64;
        while i * i <= sieve_limit {
            if !small_sieve[i as usize] {
                let mut j = i * i;
                while j <= sieve_limit {
                    small_sieve[j as usize] = true;
                    j += i;
                }
            }
            i += 1;
        }
    }

    // Main sieve - odd numbers from base to end
    let mut is_composite = vec![false; sieve_len];

    let mut p = 3i64;
    while p <= sieve_limit {
        if small_sieve[p as usize] {
            p += 2;
            continue;
        }
        let mut first = p * p;
        if first < base {
            let mut k = (base + p - 1) / p;
            if k % 2 == 0 {
                k += 1;
            }
            first = p * k;
        }
        if first <= end {
            let idx = ((first - base) / 2) as usize;
            let mut i = idx;
            while i < sieve_len {
                is_composite[i] = true;
                i += p as usize;
            }
        }
        p += 2;
    }

    let is_prime_num = |num: i64| -> bool {
        if num < 2 {
            return false;
        }
        if num == 2 {
            return true;
        }
        if num % 2 == 0 {
            return false;
        }
        if num >= base && num <= end {
            !is_composite[((num - base) / 2) as usize]
        } else {
            false
        }
    };

    // Build 5 rows: row-2..row+2, indexed 0..4
    let mut row_starts = [0i64; 5];
    let mut row_lens = [0i64; 5];
    for i in 0..5 {
        row_starts[i] = tr(row - 3 + i as i64) + 1;
        row_lens[i] = row - 2 + i as i64;
    }

    // Build primality arrays
    let mut is_p: Vec<Vec<bool>> = Vec::new();
    for ri in 0..5 {
        let len = row_lens[ri] as usize;
        let mut arr = vec![false; len];
        for j in 0..len {
            let num = row_starts[ri] + j as i64;
            if is_prime_num(num) {
                arr[j] = true;
            }
        }
        is_p.push(arr);
    }

    // Directions for adjacency in triangular grid
    let dirs: [(i32, i32); 6] = [(-1, -1), (-1, 0), (-1, 1), (1, -1), (1, 0), (1, 1)];

    // Compute central for rows 1,2,3 (prime with >= 2 prime neighbours)
    let mut central: Vec<Option<Vec<bool>>> = vec![None; 5];
    for ri in 1..=3 {
        let len = row_lens[ri] as usize;
        let mut arr = vec![false; len];
        for j in 0..len {
            if !is_p[ri][j] {
                continue;
            }
            let mut count = 0;
            for &(dr, dc) in &dirs {
                let ni = ri as i32 + dr;
                let nj = j as i64 + dc as i64;
                if ni >= 0 && ni < 5 && nj >= 0 && nj < row_lens[ni as usize] {
                    if is_p[ni as usize][nj as usize] {
                        count += 1;
                        if count >= 2 {
                            break;
                        }
                    }
                }
            }
            if count >= 2 {
                arr[j] = true;
            }
        }
        central[ri] = Some(arr);
    }

    // Sum good primes in target row (ri=2)
    let target_ri = 2;
    let target_start = row_starts[target_ri];
    let target_len = row_lens[target_ri] as usize;

    let mut total = 0i64;
    for j in 0..target_len {
        if !is_p[target_ri][j] {
            continue;
        }
        let mut good = central[target_ri].as_ref().unwrap()[j];
        if !good {
            for &(dr, dc) in &dirs {
                let ni = target_ri as i32 + dr;
                let nj = j as i64 + dc as i64;
                if ni >= 1
                    && ni <= 3
                    && nj >= 0
                    && nj < row_lens[ni as usize]
                {
                    if central[ni as usize].as_ref().unwrap()[nj as usize] {
                        good = true;
                        break;
                    }
                }
            }
        }
        if good {
            total += target_start + j as i64;
        }
    }

    total
}

fn main() {
    let result = sum_good_primes(5678027) + sum_good_primes(7208785);
    println!("{}", result);
}
