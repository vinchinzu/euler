// Project Euler 209: Circular Logic
fn main() {
    let total = 1usize << 6; // 64

    // Lucas-like: good[1]=1, good[2]=3, good[n]=good[n-1]+good[n-2]
    let mut good = [0i64; 65];
    good[1] = 1;
    good[2] = 3;
    for i in 3..65 {
        good[i] = good[i - 1] + good[i - 2];
    }

    let mut seen = [false; 64];
    let mut ans: i64 = 1;

    for bits in 0..total {
        if seen[bits] { continue; }

        let mut cur = bits;
        let mut cycle_len = 0;
        while !seen[cur] {
            seen[cur] = true;
            let a = (cur >> 5) & 1;
            let b = (cur >> 4) & 1;
            let c = (cur >> 3) & 1;
            let d = (cur >> 2) & 1;
            let e = (cur >> 1) & 1;
            let f = cur & 1;
            let new_f = a ^ (b & c);
            cur = (b << 5) | (c << 4) | (d << 3) | (e << 2) | (f << 1) | new_f;
            cycle_len += 1;
        }

        if cycle_len > 0 {
            ans *= good[cycle_len];
        }
    }

    println!("{}", ans);
}
