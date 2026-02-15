// Project Euler 238: Infinite string tour
const S0: i64 = 14025256;
const M: i64 = 20300713;

fn main() {
    let big_n: i64 = 2_000_000_000_000_000;
    let max_period: usize = 3_000_000;

    // Phase 1: Generate BBS sequence and find period
    let mut seq = Vec::with_capacity(max_period);
    let mut s = S0 as i64;
    loop {
        seq.push(s as i32);
        s = (s * s) % M;
        if s == S0 as i64 || seq.len() >= max_period { break; }
    }
    let period = seq.len();

    // Phase 2: Build cumulative digit sums
    let mut total_len: i64 = 0;
    for &val in &seq {
        let mut n = val;
        if n == 0 { total_len += 1; }
        else { while n > 0 { total_len += 1; n /= 10; } }
    }
    let l = total_len;

    let mut cumsum = vec![0i64; (l + 1) as usize];
    let mut pos: usize = 0;
    for &val in &seq {
        let s = format!("{}", val);
        for b in s.bytes() {
            cumsum[pos + 1] = cumsum[pos] + (b - b'0') as i64;
            pos += 1;
        }
    }

    let d = cumsum[l as usize]; // total digit sum per period

    // Phase 3: Find first occurrence of each cumsum value mod D
    let mut s_bool = vec![false; d as usize];
    let mut first_occs: Vec<(i64, i64)> = Vec::new(); // (value, position)

    for i in 0..l {
        let v = cumsum[i as usize] % d;
        if !s_bool[v as usize] {
            s_bool[v as usize] = true;
            first_occs.push((v, i));
        }
    }

    let q = big_n / d;
    let r = big_n % d;

    let mut covered = vec![false; d as usize];
    let mut total_f: i64 = 0;
    let mut partial_g: i64 = 0;
    let mut total_covered: i64 = 0;

    for step in 0..first_occs.len() {
        if total_covered >= d { break; }
        let (v, a_pos) = first_occs[step];
        let f_val = a_pos + 1;

        for si in 0..first_occs.len() {
            let s_val = first_occs[si].0;
            let r_val = ((s_val - v % d) % d + d) % d;
            if !covered[r_val as usize] {
                covered[r_val as usize] = true;
                total_f += f_val;
                total_covered += 1;
                if r_val >= 1 && r_val <= r {
                    partial_g += f_val;
                }
            }
        }
    }

    let answer = q * total_f + partial_g;
    println!("{answer}");
}
