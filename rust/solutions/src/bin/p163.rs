// Project Euler 163: Cross-hatched triangles
const N: i32 = 36;

struct Line {
    s: i32,
    e: i32,
}

fn intersect(l1: &Line, l2: &Line) -> bool {
    let (s1, e1, s2, e2) = (l1.s, l1.e, l2.s, l2.e);
    (s1 <= s2 && s2 <= e1 && e1 <= e2) || (s2 <= s1 && s1 <= e2 && e2 <= e1)
}

fn main() {
    let p = 6 * N;
    let mut lines = Vec::new();

    for base in 0..3 {
        for i in 0..=N {
            let mut s = (2 * N * base + 2 * i) % p;
            let mut e = ((2 * N * base - 2 * i % p) % p + p) % p;
            if s > e { std::mem::swap(&mut s, &mut e); }
            lines.push(Line { s, e });
        }
        for i in 0..=(2 * N) {
            let mut s = (2 * N * base + 2 * i) % p;
            let mut e = ((2 * N * base - i) % p + p) % p;
            if s > e { std::mem::swap(&mut s, &mut e); }
            lines.push(Line { s, e });
        }
    }

    let nlines = lines.len();
    let nw = (nlines + 63) / 64;
    let mut adj = vec![0u64; nlines * nw];

    for i in 0..nlines {
        for j in (i + 1)..nlines {
            if intersect(&lines[i], &lines[j]) {
                adj[i * nw + j / 64] |= 1u64 << (j % 64);
                adj[j * nw + i / 64] |= 1u64 << (i % 64);
            }
        }
    }

    let mut ans: i64 = 0;
    for i in 0..nlines {
        for j in (i + 1)..nlines {
            if adj[i * nw + j / 64] & (1u64 << (j % 64)) == 0 { continue; }
            for w in (j / 64)..nw {
                let mut common = adj[i * nw + w] & adj[j * nw + w];
                if w == j / 64 {
                    let bit = j % 64;
                    if bit < 63 {
                        common &= !((1u64 << (bit + 1)) - 1);
                    } else {
                        common = 0;
                    }
                }
                ans += common.count_ones() as i64;
            }
        }
    }

    let tr = (N as i64 + 1) * (N as i64 + 2) / 2;
    ans -= 20 * tr + (N as i64) * (N as i64);

    println!("{ans}");
}
