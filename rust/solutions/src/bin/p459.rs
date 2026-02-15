const N: usize = 1_000_000;
const L: usize = 512;

fn nim_prod(a: i32, b: i32, cache: &mut Vec<Vec<i32>>) -> i32 {
    if a == 0 || b == 0 { return 0; }
    if a == 1 { return b; }
    if b == 1 { return a; }
    let au = a as usize;
    let bu = b as usize;
    if au < L && bu < L && cache[au][bu] >= 0 { return cache[au][bu]; }

    let result;
    if a & (a - 1) != 0 {
        let low = a & (-a);
        result = nim_prod(low, b, cache) ^ nim_prod(a ^ low, b, cache);
    } else if b & (b - 1) != 0 {
        let low = b & (-b);
        result = nim_prod(a, low, cache) ^ nim_prod(a, b ^ low, cache);
    } else {
        let i = a.trailing_zeros();
        let j = b.trailing_zeros();
        if i & j == 0 {
            result = a * b;
        } else {
            let common = i & j;
            let common_bit = 31 - common.leading_zeros();
            let fk = 1u32 << common_bit;
            let f_k = 1i32 << fk;
            let sq_fk = f_k + (f_k >> 1);

            let a2 = 1i32 << (i - fk);
            let b2 = 1i32 << (j - fk);
            result = nim_prod(nim_prod(a2, b2, cache), sq_fk, cache);
        }
    }

    if au < L && bu < L { cache[au][bu] = result; }
    result
}

fn main() {
    let mut cache = vec![vec![-1i32; L]; L];

    let mut sq_steps = Vec::new();
    let mut tr_steps = Vec::new();
    let mut i = 1i64;
    while i * i <= N as i64 { sq_steps.push((i * i) as usize); i += 1; }
    i = 1;
    while i * (i + 1) / 2 <= N as i64 { tr_steps.push((i * (i + 1) / 2) as usize); i += 1; }

    let mut rn_x = vec![0i32; N + 1];
    let mut rn_y = vec![0i32; N + 1];

    for j in 1..=N {
        let mut used = vec![false; L];
        for &s in &sq_steps {
            if s > j { break; }
            let val = (rn_x[j - 1] ^ rn_x[j - s]) as usize;
            if val < L { used[val] = true; }
        }
        let mut mex = 0;
        while mex < L && used[mex] { mex += 1; }
        rn_x[j] = rn_x[j - 1] ^ mex as i32;
    }

    for j in 1..=N {
        let mut used = vec![false; L];
        for &s in &tr_steps {
            if s > j { break; }
            let val = (rn_y[j - 1] ^ rn_y[j - s]) as usize;
            if val < L { used[val] = true; }
        }
        let mut mex = 0;
        while mex < L && used[mex] { mex += 1; }
        rn_y[j] = rn_y[j - 1] ^ mex as i32;
    }

    let mut cnt_x = vec![0i64; L];
    let mut cnt_y = vec![0i64; L];

    for j in 1..=N {
        for &s in &sq_steps {
            if s > j { break; }
            let val = (rn_x[j] ^ rn_x[j - s]) as usize;
            if val < L { cnt_x[val] += 1; }
        }
    }
    for j in 1..=N {
        for &s in &tr_steps {
            if s > j { break; }
            let val = (rn_y[j] ^ rn_y[j - s]) as usize;
            if val < L { cnt_y[val] += 1; }
        }
    }

    let target = nim_prod(rn_x[N], rn_y[N], &mut cache);

    let mut ans: i64 = 0;
    for n0 in 0..L {
        if cnt_x[n0] == 0 { continue; }
        for n1 in 0..L {
            if cnt_y[n1] == 0 { continue; }
            if nim_prod(n0 as i32, n1 as i32, &mut cache) == target {
                ans += cnt_x[n0] * cnt_y[n1];
            }
        }
    }

    println!("{}", ans);
}
