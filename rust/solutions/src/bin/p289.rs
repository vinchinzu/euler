// Project Euler 289: Eulerian Cycles
// Profile DP for counting non-crossing Eulerian cycles on circle grid.
// Computes L(6,10) mod 10^10. Uses L(W,H) = L(H,W) symmetry to orient
// the profile along the shorter dimension (M=6) for efficiency.

use std::collections::HashMap;

// L(6,10): M=6 (profile/inner dim), N=10 (sweep/outer dim)
const N: usize = 10;
const M: usize = 6;
const MOD: i64 = 10_000_000_000;

const CONNECTIONS: [i32; 14] = [
    1111, 1114, 1133, 1134, 1131,
    1211, 1214, 1221, 1222, 1224,
    1231, 1232, 1233, 1234,
];

fn decode(s: i64, state: &mut [i64; 20]) {
    for i in 0..(M + 3) {
        state[i] = (s >> (i * 4)) & 15;
    }
}

fn merge(mut s: i64, x: i64, y: i64) -> i64 {
    if s == -1 { return s; }
    let mut i = 0usize;
    while (s >> (i * 4)) != 0 {
        let w = (s >> (i * 4)) & 15;
        if w == x {
            s ^= (x ^ y) << (i * 4);
        }
        i += 1;
    }
    s
}

fn find_count(mut s: i64, k: i64) -> i32 {
    let mut ret = 0;
    for _ in 0..(M + 4) {
        if (s & 15) == k { ret += 1; }
        s >>= 4;
    }
    ret
}

fn rep(s: i64) -> i64 {
    let mut ret: i64 = 0;
    let mut t: i64 = 0;
    let mut f = [0i64; 20];
    let mut i = 0usize;
    while (s >> (i * 4)) != 0 {
        let x = ((s >> (i * 4)) & 15) as usize;
        if f[x] == 0 {
            t += 1;
            f[x] = t;
        }
        ret |= (f[x] - 1) << (i * 4);
        i += 1;
    }
    ret
}

fn init_cache(k: i64, x: usize, y: usize) -> Vec<i64> {
    let mut state = [0i64; 20];
    decode(k, &mut state);
    let mut result = Vec::new();

    for &conn in &CONNECTIONS {
        let indices = [
            (conn / 1000 - 1) as i64,
            ((conn / 100) % 10 - 1) as i64,
            ((conn / 10) % 10 - 1) as i64,
            (conn % 10 - 1) as i64,
        ];

        let new_color: i64 = if y == M || x == N { 0 } else { 15 };
        let colors = [state[y], state[y + 1], state[y + 2], new_color];

        let mut kk: i64 = (k << 4) | new_color;

        // Check consistency
        for i in 0..4 {
            for j in 0..4 {
                if colors[i] == 0 && ((colors[j] == 0) != (indices[i] == indices[j])) {
                    kk = -1;
                }
            }
        }

        // Merge connections
        for i in 0..4usize {
            if kk == -1 { break; }
            if i as i64 == indices[i] { continue; }
            let c_src = if i == 3 {
                new_color
            } else {
                (kk >> (4 * (y + i + 1))) & 15
            };
            let c_dst = (kk >> (4 * ((y as i64 + indices[i] + 1) as usize))) & 15;
            if c_src == 0 { continue; }
            if c_src == c_dst || c_src == 0 || c_dst == 0 {
                kk = -1;
            } else {
                kk = merge(kk, c_src, c_dst);
            }
        }

        if kk != -1 {
            let cur = kk & 15;
            kk >>= 4;
            let old = (kk >> (4 * (y + 1))) & 15;
            if find_count(kk, old) > 1 || old == cur || (x == N && y == M) {
                kk ^= (cur ^ old) << (4 * (y + 1));
                if y == M {
                    kk <<= 4;
                }
                result.push(rep(kk));
            }
        }
    }

    result
}

fn main() {
    let mut f: HashMap<i64, i64> = HashMap::new();
    let mut cache: HashMap<(i64, usize), Vec<i64>> = HashMap::new();

    f.insert(0, 1);

    for x in 0..=N {
        for y in 0..=M {
            let g = std::mem::take(&mut f);
            for (&k, &v) in &g {
                if x == N {
                    for kk in init_cache(k, x, y) {
                        let e = f.entry(kk).or_insert(0);
                        *e = (*e + v) % MOD;
                    }
                } else {
                    let key = (k, y);
                    let successors = cache.entry(key).or_insert_with(|| init_cache(k, x, y));
                    for &kk in successors.iter() {
                        let e = f.entry(kk).or_insert(0);
                        *e = (*e + v) % MOD;
                    }
                }
            }
        }
    }

    println!("{}", f.get(&0).copied().unwrap_or(0));
}
