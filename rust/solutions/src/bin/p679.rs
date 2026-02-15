// Project Euler 679 - Free Farea
// DP: count length-30 strings over "AEFR" containing exactly one of each keyword.

const N: usize = 30;
const NCHARS: usize = 4;
const BASE: usize = 5;
const PREFIX_STATES: usize = 625; // 5^4
const MASK_STATES: usize = 16;
const CHARS: [u8; 4] = [b'A', b'E', b'F', b'R'];
const KEYWORDS: [&[u8]; 4] = [b"FREE", b"FARE", b"AREA", b"REEF"];

fn decode_prefix(mut v: usize) -> [u8; 4] {
    let chars_v2 = [b'A', b'E', b'F', b'R', b'_'];
    let mut out = [0u8; 4];
    for i in (0..4).rev() { out[i] = chars_v2[v % BASE]; v /= BASE; }
    out
}

fn main() {
    let mut next_prefix = [[0usize; NCHARS]; PREFIX_STATES];
    let mut keyword_match = [0u16; PREFIX_STATES];

    for p in 0..PREFIX_STATES {
        let ps = decode_prefix(p);
        let mut km = 0u16;
        for (k, kw) in KEYWORDS.iter().enumerate() {
            if ps == **kw { km |= 1 << k; }
        }
        keyword_match[p] = km;
        for (ci, &ch) in CHARS.iter().enumerate() {
            let new_p = [ps[1], ps[2], ps[3], ch];
            let mut v = 0usize;
            for i in 0..4 {
                let c = CHARS.iter().position(|&x| x == new_p[i]).map_or(4, |p| p);
                v = v * BASE + c;
            }
            next_prefix[p][ci] = v;
        }
    }

    // dp[prefix][mask]
    let mut dp = vec![vec![0i64; MASK_STATES]; PREFIX_STATES];
    let init_prefix = 4 * 125 + 4 * 25 + 4 * 5 + 4; // 624
    dp[init_prefix][0] = 1;

    for _ in 0..N {
        let mut new_dp = vec![vec![0i64; MASK_STATES]; PREFIX_STATES];
        for p in 0..PREFIX_STATES {
            for mask in 0..MASK_STATES {
                if dp[p][mask] == 0 { continue; }
                let cnt = dp[p][mask];
                for c in 0..NCHARS {
                    let np = next_prefix[p][c];
                    let kw = keyword_match[np] as usize;
                    if kw & mask != 0 { continue; }
                    new_dp[np][mask | kw] += cnt;
                }
            }
        }
        dp = new_dp;
    }

    let result: i64 = (0..PREFIX_STATES).map(|p| dp[p][0xF]).sum();
    println!("{}", result);
}
