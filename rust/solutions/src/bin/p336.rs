// Project Euler 336: Maximix Arrangements
//
// Maximix of n are generated from maximix of n-1:
//   t = reverse([1] + [x+1 for x in seq])
//   for y in 1..n-1: emit t[:y] + reverse(t[y:])
// Then take the 2011th in lexicographic order.

const N: usize = 11;
const TARGET: usize = 2011;
const MAX_COUNT: usize = 362_880; // 9!

#[inline]
fn pack(a: &[u8; N]) -> u64 {
    let mut x = 0u64;
    for i in 0..N {
        x = (x << 4) | a[i] as u64;
    }
    x
}

fn main() {
    let mut cur: Vec<[u8; N]> = Vec::with_capacity(MAX_COUNT / 9);
    let mut nxt: Vec<[u8; N]> = Vec::with_capacity(MAX_COUNT / 9);
    let mut seed = [0u8; N];
    seed[0] = 2;
    seed[1] = 1;
    cur.push(seed);

    for n in 3..N {
        nxt.clear();
        for seq in &cur {
            let mut t = [0u8; N];
            t[0] = 1;
            for i in 0..n - 1 {
                t[i + 1] = seq[i] + 1;
            }
            t[..n].reverse();
            for y in 1..n - 1 {
                let mut p = t;
                p[y..n].reverse();
                nxt.push(p);
            }
        }
        std::mem::swap(&mut cur, &mut nxt);
    }

    let mut keys = Vec::with_capacity(MAX_COUNT);
    for seq in &cur {
        let mut t = [0u8; N];
        t[0] = 1;
        for i in 0..N - 1 {
            t[i + 1] = seq[i] + 1;
        }
        t.reverse();
        for y in 1..N - 1 {
            let mut p = t;
            p[y..].reverse();
            keys.push(pack(&p));
        }
    }
    keys.sort_unstable();
    let mut x = keys[TARGET - 1];
    let mut s = [0u8; N];
    for i in (0..N).rev() {
        s[i] = b'A' + (x as u8 & 0xf) - 1;
        x >>= 4;
    }
    // SAFETY: s is ASCII letters A..K
    println!("{}", unsafe { std::str::from_utf8_unchecked(&s) });
}
