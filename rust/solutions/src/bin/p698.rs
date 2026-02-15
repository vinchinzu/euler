// Project Euler 698 - 123 Numbers
// Valid digit counts from the set NUMS. Digit-by-digit construction.

const M: i64 = 123_123_123;

const NUMS: [i32; 13] = [0, 1, 2, 3, 11, 12, 13, 21, 22, 23, 31, 32, 33];

fn multinomial(c1: i32, c2: i32, c3: i32) -> i64 {
    let total = c1 + c2 + c3;
    let mut result = 1i64;
    for i in 0..c1 {
        result = result * (total - i) as i64 / (i + 1) as i64;
    }
    for i in 0..c2 {
        result = result * (total - c1 - i) as i64 / (i + 1) as i64;
    }
    result
}

fn main() {
    let n_target: i64 = 111_111_111_111_222_333;
    let mut limit = n_target;

    // Find the length of the N-th number
    let mut length = 0i32;
    loop {
        let mut total_count = 0i64;
        for &ni in &NUMS {
            for &nj in &NUMS {
                for &nk in &NUMS {
                    if ni + nj + nk == length {
                        total_count += multinomial(ni, nj, nk);
                    }
                }
            }
        }
        if total_count > limit { break; }
        limit -= total_count;
        length += 1;
    }

    // Build the number digit by digit
    let mut digits = vec![0i32; length as usize];
    let mut counts_so_far = [0i32; 3];

    for pos in 0..length as usize {
        for d in 0..3i32 {
            let mut total_count = 0i64;
            for &ni in &NUMS {
                for &nj in &NUMS {
                    for &nk in &NUMS {
                        if ni + nj + nk != length { continue; }
                        let mut cnts = [ni, nj, nk];
                        let mut valid = true;
                        for q in 0..3 {
                            cnts[q] -= counts_so_far[q];
                            if cnts[q] < 0 { valid = false; break; }
                        }
                        if !valid { continue; }
                        cnts[d as usize] -= 1;
                        if cnts[d as usize] < 0 { continue; }
                        let rem = length - pos as i32 - 1;
                        if cnts[0] + cnts[1] + cnts[2] != rem { continue; }
                        total_count += multinomial(cnts[0], cnts[1], cnts[2]);
                    }
                }
            }
            if total_count > limit {
                digits[pos] = d + 1;
                counts_so_far[d as usize] += 1;
                break;
            }
            limit -= total_count;
        }
    }

    // Compute the number mod M
    let mut result = 0i64;
    for i in 0..length as usize {
        result = (result * 10 + digits[i] as i64) % M;
    }

    println!("{}", result);
}
