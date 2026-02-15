// Project Euler 839 - Beans in Bowls
// BBS sequence, merge blocks, compute steps

const N: usize = 10_000_000;

fn main() {
    // Generate BBS sequence
    let mut s = vec![0i64; N];
    s[0] = 290797;
    let m: i64 = 50515093;
    for i in 1..N {
        s[i] = s[i - 1] * s[i - 1] % m;
    }

    // Merge blocks
    let mut blocks: Vec<(i64, i64)> = Vec::with_capacity(N); // (value, length)
    for i in 0..N {
        blocks.push((s[i], 1));
        while blocks.len() >= 2 {
            let nb = blocks.len();
            let (v1, l1) = blocks[nb - 2];
            let (v2, l2) = blocks[nb - 1];
            let avg1 = (v1 + l1 - 1) / l1;
            let avg2 = v2 / l2;
            if avg1 <= avg2 { break; }
            blocks[nb - 2] = (v1 + v2, l1 + l2);
            blocks.pop();
        }
    }

    // Compute final state T
    let mut t = vec![0i64; N];
    let mut idx = 0;
    for &(v, len) in &blocks {
        for i in 0..len as usize {
            t[idx] = (v + i as i64) / len;
            idx += 1;
        }
    }

    // Count steps
    let mut ans: i64 = 0;
    for i in 0..N - 1 {
        let diff = s[i] - t[i];
        s[i + 1] += diff;
        ans += diff;
    }

    println!("{}", ans);
}
