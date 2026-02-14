// Project Euler 254: Sums of Digit Factorials

const B: usize = 10;
const LVAL: i64 = 10_000_000;
const NVAL: usize = 150;
const C_MAX: usize = 63;

fn main() {
    let mut g_stack: Vec<u8> = Vec::with_capacity(10_000_100);
    let mut g_len = 0usize;
    let mut sg = 0i64;

    let mut best_sg = [0i64; C_MAX + 1];
    let mut best_len = [0usize; C_MAX + 1];
    let mut best_g: Vec<Vec<u8>> = vec![Vec::new(); C_MAX + 1];
    let mut best_valid = [false; C_MAX + 1];

    let is_better = |sf: usize, g_stack: &[u8], g_len: usize, best_valid: &[bool], best_len: &[usize], best_g: &[Vec<u8>]| -> bool {
        if !best_valid[sf] { return true; }
        if g_len < best_len[sf] { return true; }
        if g_len > best_len[sf] { return false; }
        for i in (0..g_len).rev() {
            if g_stack[i] < best_g[sf][i] { return true; }
            if g_stack[i] > best_g[sf][i] { return false; }
        }
        false
    };

    let mut sf = 0i64;

    for f in 1..LVAL {
        // Update sf: digit sum of f
        sf += 1;
        {
            let mut n = f;
            while n % B as i64 == 0 {
                sf -= (B as i64) - 1;
                n /= B as i64;
            }
        }

        // Append "1" to g
        if g_len >= g_stack.len() { g_stack.push(1); } else { g_stack[g_len] = 1; }
        g_len += 1;
        sg += 1;

        // Consolidate
        {
            let mut n = f;
            let mut d = 2usize;
            while d < B && n % d as i64 == 0 {
                for i in 0..d {
                    sg -= g_stack[g_len - 1 - i] as i64;
                }
                g_len -= d;
                g_stack[g_len] = d as u8;
                g_len += 1;
                sg += d as i64;
                n /= d as i64;
                d += 1;
            }
        }

        if sf >= 1 && (sf as usize) <= C_MAX {
            let sfu = sf as usize;
            if is_better(sfu, &g_stack, g_len, &best_valid, &best_len, &best_g) {
                best_valid[sfu] = true;
                best_sg[sfu] = sg;
                best_len[sfu] = g_len;
                best_g[sfu] = g_stack[..g_len].to_vec();
            }
        }
    }

    let mut ans: i64 = 0;
    for i in 1..=C_MAX.min(NVAL) {
        if best_valid[i] {
            ans += best_sg[i];
        }
    }

    // For sf > C_MAX (64..150)
    let facts: [i64; 10] = {
        let mut f = [0i64; 10];
        f[0] = 1;
        for i in 1..10 { f[i] = f[i - 1] * i as i64; }
        f
    };

    for sf2 in (C_MAX + 1)..=NVAL {
        let mut f_val = (sf2 as i64 % (B as i64 - 1) + 1) * {
            let mut power = 1i64;
            for _ in 0..(sf2 / (B - 1)) { power *= B as i64; }
            power
        } - 1;

        let mut sg_val = 0i64;
        for d in (1..=9).rev() {
            let c = f_val / facts[d];
            f_val -= c * facts[d];
            sg_val += d as i64 * c;
        }
        ans += sg_val;
    }

    println!("{}", ans);
}
