// Problem 974: Very Odd Numbers
// Ported from python/974.py.

const DIGITS: [u8; 5] = [1, 3, 5, 7, 9];

#[inline]
fn digit_bit(d: u8) -> usize {
    match d {
        1 => 0,
        3 => 1,
        5 => 2,
        7 => 3,
        9 => 4,
        _ => unreachable!("invalid digit"),
    }
}

fn build_dp(len: usize) -> Vec<u128> {
    let states = (len + 1) * 3 * 7 * 32;
    let mut dp = vec![0_u128; states];

    let idx = |pos: usize, mod3: usize, mod7: usize, mask: usize| -> usize {
        (((pos * 3 + mod3) * 7 + mod7) * 32) + mask
    };

    for mod3 in 0..3 {
        for mod7 in 0..7 {
            for mask in 0..32 {
                let v = if mod3 == 0 && mod7 == 0 && mask == 31 {
                    1_u128
                } else {
                    0_u128
                };
                dp[idx(len, mod3, mod7, mask)] = v;
            }
        }
    }

    for pos in (0..len).rev() {
        let last_pos = pos + 1 == len;
        for mod3 in 0..3 {
            for mod7 in 0..7 {
                for mask in 0..32 {
                    let mut total = 0_u128;
                    if last_pos {
                        let d = 5_u8;
                        let n3 = (mod3 * 10 + d as usize) % 3;
                        let n7 = (mod7 * 10 + d as usize) % 7;
                        let nmask = mask ^ (1usize << digit_bit(d));
                        total += dp[idx(pos + 1, n3, n7, nmask)];
                    } else {
                        for &d in &DIGITS {
                            let n3 = (mod3 * 10 + d as usize) % 3;
                            let n7 = (mod7 * 10 + d as usize) % 7;
                            let nmask = mask ^ (1usize << digit_bit(d));
                            total += dp[idx(pos + 1, n3, n7, nmask)];
                        }
                    }
                    dp[idx(pos, mod3, mod7, mask)] = total;
                }
            }
        }
    }

    dp
}

fn count_len(len: usize) -> u128 {
    let dp = build_dp(len);
    dp[0]
}

fn unrank(len: usize, mut k: u128) -> String {
    let dp = build_dp(len);
    let idx = |pos: usize, mod3: usize, mod7: usize, mask: usize| -> usize {
        (((pos * 3 + mod3) * 7 + mod7) * 32) + mask
    };

    let mut mod3 = 0usize;
    let mut mod7 = 0usize;
    let mut mask = 0usize;
    let mut out = String::with_capacity(len);

    for pos in 0..len {
        let last_pos = pos + 1 == len;
        if last_pos {
            let d = 5_u8;
            let n3 = (mod3 * 10 + d as usize) % 3;
            let n7 = (mod7 * 10 + d as usize) % 7;
            let nmask = mask ^ (1usize << digit_bit(d));
            let cnt = dp[idx(pos + 1, n3, n7, nmask)];
            if k <= cnt {
                out.push('5');
                mod3 = n3;
                mod7 = n7;
                mask = nmask;
            } else {
                unreachable!("k is out of bounds for the target length");
            }
            continue;
        }

        let mut chosen = false;
        for &d in &DIGITS {
            let n3 = (mod3 * 10 + d as usize) % 3;
            let n7 = (mod7 * 10 + d as usize) % 7;
            let nmask = mask ^ (1usize << digit_bit(d));
            let cnt = dp[idx(pos + 1, n3, n7, nmask)];
            if k > cnt {
                k -= cnt;
            } else {
                out.push((b'0' + d) as char);
                mod3 = n3;
                mod7 = n7;
                mask = nmask;
                chosen = true;
                break;
            }
        }
        assert!(chosen, "failed to choose next digit");
    }

    out
}

fn theta(n: u128, max_len: usize) -> String {
    let mut cum = 0_u128;
    let mut len = 1usize;
    while len <= max_len {
        let c = count_len(len);
        if cum + c >= n {
            return unrank(len, n - cum);
        }
        cum += c;
        len += 2;
    }
    panic!("max_len too small");
}

fn main() {
    println!("{}", theta(10_u128.pow(16), 200));
}
