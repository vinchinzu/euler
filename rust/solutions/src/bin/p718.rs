// Project Euler 718 - Unreachable Numbers
//
// Find sum of positive integers not expressible as A*a + B*b + C*c (a,b,c > 0)
// where A=17^6, B=19^6, C=23^6. Uses two-queue BFS over residues mod A.

const MOD: u64 = 1_000_000_007;

fn main() {
    let a: u64 = 17u64.pow(6);
    let b: u64 = 19u64.pow(6);
    let c: u64 = 23u64.pow(6);
    let au = a as usize;

    let b_div = b / a;
    let b_mod = b % a;
    let c_div = c / a;
    let c_mod = c % a;

    let mut visited = vec![0u64; (au + 63) >> 6];
    let test = |v: &[u64], i: usize| (v[i >> 6] >> (i & 63)) & 1 != 0;
    let setb = |v: &mut [u64], i: usize| v[i >> 6] |= 1u64 << (i & 63);

    // Pack (q, r) as q << 32 | r; r < A < 2^32 and val comparison is lex on (q, r)
    let pack = |q: u64, r: u64| (q << 32) | r;
    let unpack = |p: u64| (p >> 32, p & 0xFFFF_FFFF);

    let cap = au + 1;
    let mut q1 = Vec::with_capacity(cap);
    let mut q2 = Vec::with_capacity(cap);
    let mut q1_head: usize = 0;
    let mut q2_head: usize = 0;

    // start at A+B+C
    let mut q = 1 + b_div + c_div;
    let mut r = b_mod + c_mod;
    if r >= a {
        r -= a;
        q += 1;
    }
    // A + (B%A) + (C%A) may wrap twice? B%A + C%A < 2A, so at most one extra wrap
    // plus the +A already in q's 1. r = (A+B+C)%A = (B+C)%A. Good.

    let mut ans: u64 = 0;

    loop {
        let ru = r as usize;
        if !test(&visited, ru) {
            setb(&mut visited, ru);
            // ncr2(q)*A + r*q  (q < 2e8 < MOD, A < MOD, r < MOD)
            let ncr = q * (q - 1) / 2;
            let term = (ncr % MOD) * a % MOD + r * q % MOD;
            ans += term;
            if ans >= MOD {
                ans -= MOD;
            }
            if ans >= MOD {
                ans -= MOD;
            }

            let mut r1 = r + b_mod;
            let mut q1v = q + b_div;
            if r1 >= a {
                r1 -= a;
                q1v += 1;
            }
            let mut r2 = r + c_mod;
            let mut q2v = q + c_div;
            if r2 >= a {
                r2 -= a;
                q2v += 1;
            }

            if !test(&visited, r1 as usize) {
                q1.push(pack(q1v, r1));
            }
            if !test(&visited, r2 as usize) {
                q2.push(pack(q2v, r2));
            }
        }

        if q1_head >= q1.len() && q2_head >= q2.len() {
            break;
        }

        let v1 = if q1_head < q1.len() { q1[q1_head] } else { u64::MAX };
        let v2 = if q2_head < q2.len() { q2[q2_head] } else { u64::MAX };
        let valp = v1.min(v2);
        if q1_head < q1.len() && valp == q1[q1_head] {
            q1_head += 1;
        }
        if q2_head < q2.len() && valp == q2[q2_head] {
            q2_head += 1;
        }
        let (nq, nr) = unpack(valp);
        q = nq;
        r = nr;
    }

    println!("{}", ans);
}
