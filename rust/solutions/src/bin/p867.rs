// Project Euler 867 - Dodecagon Tilings
// Profile DP and memoization for tiling count. N = 10.

const MOD: i64 = 1_000_000_007;

fn mod_pow(mut base: i64, mut exp: i64, m: i64) -> i64 {
    let mut r = 1i64;
    base %= m;
    while exp > 0 {
        if exp & 1 == 1 { r = r * base % m; }
        base = base * base % m;
        exp >>= 1;
    }
    r
}

fn tilings_with_tri_hex_iter(px: &[i32], py: &[i32], npts: usize, window_len: usize) -> i64 {
    if npts == 0 { return 1; }
    if window_len > 20 { return 0; }

    let mask_mod = 1 << window_len;
    let mut dp_next = vec![1i64; mask_mod]; // base case: index == npts -> 1
    let mut dp_curr = vec![0i64; mask_mod];

    for index in (0..npts).rev() {
        let pxi = px[index];
        let pyi = py[index];

        for prev in 0..mask_mod {
            let mut good = true;
            let lim = window_len.min(index);
            for i in 0..lim {
                if (prev >> i) & 1 == 1 {
                    let qi = index - i - 1;
                    let dy = (pyi - py[qi]).abs();
                    let dx = (pxi - px[qi]).abs();
                    if dy <= 1 && dx + dy <= 2 {
                        good = false;
                        break;
                    }
                }
            }

            let next_prev_0 = (prev << 1) & (mask_mod - 1);
            let next_prev_1 = ((prev << 1) | 1) & (mask_mod - 1);

            let mut res = dp_next[next_prev_0];
            if good {
                res = (res + dp_next[next_prev_1]) % MOD;
            }
            dp_curr[prev] = res;
        }

        std::mem::swap(&mut dp_next, &mut dp_curr);
    }

    dp_next[0]
}

fn tilings_for_hexagon(size: i32, hex_cache: &mut [Option<i64>; 12]) -> i64 {
    if size <= 0 { return 1; }
    if let Some(v) = hex_cache[size as usize] { return v; }

    let mut px = Vec::new();
    let mut py = Vec::new();
    for y in (-(size - 1))..size {
        let ay = y.abs();
        let mut x = -2 * size + ay + 2;
        while x < 2 * size - ay {
            px.push(x);
            py.push(y);
            x += 2;
        }
    }

    let val = tilings_with_tri_hex_iter(&px, &py, px.len(), (2 * size - 1) as usize);
    hex_cache[size as usize] = Some(val);
    val
}

fn tilings_for_trapezoid(base: i32, height: i32, trap_cache: &mut [[Option<i64>; 12]; 12]) -> i64 {
    if height <= 0 || base <= 0 { return 1; }
    if let Some(v) = trap_cache[base as usize][height as usize] { return v; }

    let mut px = Vec::new();
    let mut py = Vec::new();
    for y in (base - height)..(base - 1) {
        let mut x = 1 - y;
        while x < y {
            px.push(x);
            py.push(y);
            x += 2;
        }
    }

    let val = tilings_with_tri_hex_iter(&px, &py, px.len(), (base - 1) as usize);
    trap_cache[base as usize][height as usize] = Some(val);
    val
}

fn tilings_for_dodecagon(
    a: i32, b: i32, allow_a: bool, allow_b: bool,
    hex_cache: &mut [Option<i64>; 12],
    trap_cache: &mut [[Option<i64>; 12]; 12],
    dodec_cache: &mut [Option<i64>; 2000],
) -> i64 {
    if a == 0 { return tilings_for_hexagon(b, hex_cache); }
    if b == 0 { return tilings_for_hexagon(a, hex_cache); }

    let key = a as usize * 100 + b as usize * 4 + (allow_a as usize) * 2 + allow_b as usize;
    if let Some(v) = dodec_cache[key] { return v; }

    let mut res = 0i64;

    if allow_a {
        for h in 1..=b {
            let t = tilings_for_trapezoid(b, h, trap_cache);
            let t6 = mod_pow(t, 6, MOD);
            let sub = tilings_for_dodecagon(a, b - h, false, true, hex_cache, trap_cache, dodec_cache);
            res = (res + t6 * sub) % MOD;
        }
    }

    if allow_b {
        for h in 1..=a {
            let t = tilings_for_trapezoid(a, h, trap_cache);
            let t6 = mod_pow(t, 6, MOD);
            let sub = tilings_for_dodecagon(a - h, b, true, false, hex_cache, trap_cache, dodec_cache);
            res = (res + t6 * sub) % MOD;
        }
    }

    if a == 1 && b == 1 {
        res = (res + 1) % MOD;
    }

    dodec_cache[key] = Some(res);
    res
}

fn main() {
    let mut hex_cache = [None; 12];
    let mut trap_cache = [[None; 12]; 12];
    let mut dodec_cache = [None; 2000];

    let ans = tilings_for_dodecagon(10, 10, true, true, &mut hex_cache, &mut trap_cache, &mut dodec_cache);
    println!("{}", ans);
}
