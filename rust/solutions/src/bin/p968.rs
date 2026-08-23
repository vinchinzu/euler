// Project Euler 968 - Quintic Pair Sums
//
// Digit DP over binary digits of the five variables. Each of the 10 pair
// inequalities x_u+x_v <= U_uv is tracked by a carry in {0,1,2}, giving
// 3^10 = 59049 states. 100 independent blocks are summed in parallel.
//
// P(2,...,2) = 7120, P(1..10) ≡ 799809376 (mod 10^9+7).

use rayon::prelude::*;

const M: u64 = 1_000_000_007;
const NS: usize = 59049; // 3^10
const NBIT: usize = 31;
const EDGES: [(u8, u8); 10] = [
    (0, 1),
    (0, 2),
    (0, 3),
    (0, 4),
    (1, 2),
    (1, 3),
    (1, 4),
    (2, 3),
    (2, 4),
    (3, 4),
];
const PRIMES: [u64; 5] = [2, 3, 5, 7, 11];

#[inline(always)]
fn mul(a: u64, b: u64) -> u64 {
    (a * b) % M
}

#[inline(always)]
fn add(a: u64, b: u64) -> u64 {
    let s = a + b;
    if s >= M {
        s - M
    } else {
        s
    }
}

struct Tables {
    decode: Vec<[u8; 10]>,
    wtab: [[u64; 32]; NBIT],
    pair_sum: [[u8; 10]; 32],
}

fn build_tables() -> Tables {
    let mut decode = vec![[0u8; 10]; NS];
    for s in 0..NS {
        let mut x = s as u32;
        for e in 0..10 {
            decode[s][e] = (x % 3) as u8;
            x /= 3;
        }
    }

    let mut pair_sum = [[0u8; 10]; 32];
    for mask in 0..32u32 {
        for e in 0..10 {
            let (u, v) = EDGES[e];
            pair_sum[mask as usize][e] = (((mask >> u) & 1) + ((mask >> v) & 1)) as u8;
        }
    }

    let mut wtab = [[1u64; 32]; NBIT];
    let mut pk = PRIMES;
    for k in 0..NBIT {
        for mask in 0..32u32 {
            let mut w = 1u64;
            for i in 0..5 {
                if mask & (1 << i) != 0 {
                    w = mul(w, pk[i]);
                }
            }
            wtab[k][mask as usize] = w;
        }
        if k + 1 < NBIT {
            for i in 0..5 {
                pk[i] = mul(pk[i], pk[i]);
            }
        }
    }

    Tables {
        decode,
        wtab,
        pair_sum,
    }
}

#[inline(always)]
fn next_state(carries: &[u8; 10], pair: &[u8; 10], ubits: u32) -> usize {
    let mut s = 0usize;
    let mut p3 = 1usize;
    for e in 0..10 {
        let n = pair[e] as i32 + carries[e] as i32 - ((ubits >> e) & 1) as i32;
        let cp = ((n + 1) >> 1) as usize;
        s += cp * p3;
        p3 *= 3;
    }
    s
}

fn eval_p(u: &[u64; 10], t: &Tables) -> u64 {
    let mut ubits = [0u32; NBIT];
    for e in 0..10 {
        let mut x = u[e];
        for k in 0..NBIT {
            ubits[k] |= ((x & 1) as u32) << e;
            x >>= 1;
        }
    }

    let mut cur = vec![0u64; NS];
    let mut nxt = vec![0u64; NS];
    cur[0] = 1;
    let mut active: Vec<usize> = vec![0];
    let mut next_active: Vec<usize> = Vec::with_capacity(4096);

    for k in 0..NBIT {
        next_active.clear();
        let ub = ubits[k];
        for &s in &active {
            let w = unsafe { *cur.get_unchecked(s) };
            if w == 0 {
                continue;
            }
            let carries = unsafe { t.decode.get_unchecked(s) };
            for mask in 0..32 {
                let s2 = next_state(carries, unsafe { t.pair_sum.get_unchecked(mask) }, ub);
                let addv = mul(w, unsafe { *t.wtab.get_unchecked(k).get_unchecked(mask) });
                let slot = unsafe { nxt.get_unchecked_mut(s2) };
                if *slot == 0 {
                    next_active.push(s2);
                }
                *slot = add(*slot, addv);
            }
            unsafe {
                *cur.get_unchecked_mut(s) = 0;
            }
        }
        std::mem::swap(&mut cur, &mut nxt);
        std::mem::swap(&mut active, &mut next_active);
    }

    cur[0]
}

fn main() {
    let t = build_tables();

    // Sample checks (digit DP is cheap on tiny U; guards the previous wrong closed form).
    if eval_p(&[2; 10], &t) != 7120 || eval_p(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 10], &t) != 799809376
    {
        eprintln!("p968 sample mismatch");
        std::process::exit(1);
    }

    let mut a = vec![0u64; 1001];
    a[0] = 1;
    a[1] = 7;
    for n in 2..=1000 {
        a[n] = add(mul(7, a[n - 1]), mul(a[n - 2], a[n - 2]));
    }

    let blocks: Vec<[u64; 10]> = (0..100)
        .map(|n| {
            let mut xs = [0u64; 10];
            for i in 0..10 {
                xs[i] = a[10 * n + i];
            }
            xs
        })
        .collect();

    let total = blocks.par_iter().map(|b| eval_p(b, &t)).reduce(|| 0, add);
    println!("{}", total);
}
