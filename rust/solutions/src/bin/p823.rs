// Project Euler 823 - Factor Shuffle
//
// After a transient, the k-th smallest prime factor of the number added at
// round t is periodic with period k. Simulate with a linked factor pool (no
// Vec clones), detect the cycles, then jump to m = 10^16.

const N: usize = 10_000;
const MOD: u64 = 1_234_567_891;
const M: u64 = 10_000_000_000_000_000;
const K_EXTRA: usize = 10;
const MAX_ROUNDS: u32 = 200_000;

struct Node {
    prime: i32,
    next: i32,
}

fn sort_extracted(ext: &mut [(i32, i32)], cnt: &mut [u32], out: &mut [(i32, i32)]) {
    let n = ext.len();
    if n <= 1024 {
        ext.sort_unstable_by_key(|&(p, _)| p);
        return;
    }
    for &(p, _) in ext.iter() {
        cnt[p as usize] += 1;
    }
    let mut sum = 0u32;
    for c in cnt.iter_mut() {
        let t = *c;
        *c = sum;
        sum += t;
    }
    for &pair in ext.iter() {
        let p = pair.0 as usize;
        let i = cnt[p] as usize;
        out[i] = pair;
        cnt[p] += 1;
    }
    ext.copy_from_slice(&out[..n]);
    cnt.fill(0);
}

fn main() {
    let mut spf = vec![0i32; N + 1];
    for i in 0..=N {
        spf[i] = i as i32;
    }
    let lim = N.isqrt();
    for i in 2..=lim {
        if spf[i] == i as i32 {
            let mut j = i * i;
            while j <= N {
                if spf[j] == j as i32 {
                    spf[j] = i as i32;
                }
                j += i;
            }
        }
    }

    let mut nodes: Vec<Node> = Vec::with_capacity(4 * N);
    let mut piles: Vec<i32> = Vec::with_capacity(N);
    let mut tmp = [0i32; 16];

    for n in 2..=N {
        let mut fc = 0usize;
        let mut x = n;
        while x > 1 {
            let p = spf[x];
            tmp[fc] = p;
            fc += 1;
            x = (x as i32 / p) as usize;
        }
        let mut head = -1i32;
        let mut tail = -1i32;
        for i in 0..fc {
            let idx = nodes.len() as i32;
            nodes.push(Node {
                prime: tmp[i],
                next: -1,
            });
            if head < 0 {
                head = idx;
            } else {
                nodes[tail as usize].next = idx;
            }
            tail = idx;
        }
        piles.push(head);
    }

    let total_factors = nodes.len();
    piles.reserve(total_factors);
    let k_lim = (2 * total_factors).isqrt() + K_EXTRA;
    // Two full periods of the longest column is enough to accept the cycle.
    let streak_needed = (k_lim * 2) as u32;

    // Flat triangle of ring buffers: column k occupies k slots.
    let mut off = vec![0usize; k_lim + 1];
    let mut acc = 0usize;
    for k in 1..=k_lim {
        off[k] = acc;
        acc += k;
    }
    let mut flat = vec![1i32; acc];
    let mut hd = vec![0usize; k_lim + 1];
    let mut fill = vec![0usize; k_lim + 1];

    let mut extracted: Vec<(i32, i32)> = Vec::with_capacity(total_factors);
    let mut ext_out: Vec<(i32, i32)> = vec![(0, 0); total_factors];
    let mut cnt = vec![0u32; N + 1];

    let mut t = 0u32;
    let mut stable = 0u32;
    let mut patterns: Vec<Vec<i32>> = Vec::new();
    let mut kmax = 0usize;
    let mut found = false;

    while t < MAX_ROUNDS {
        t += 1;
        let np = piles.len();
        // SAFETY: np heads are valid node indices; compact writes only to i <= r.
        // Every slot in 0..np is written before the slice is read.
        unsafe {
            extracted.reserve(np);
            extracted.set_len(np);
            let mut w = 0usize;
            for r in 0..np {
                let h = *piles.get_unchecked(r);
                let node = nodes.get_unchecked(h as usize);
                *extracted.get_unchecked_mut(r) = (node.prime, h);
                let nxt = node.next;
                if nxt >= 0 {
                    *piles.get_unchecked_mut(w) = nxt;
                    w += 1;
                }
            }
            piles.set_len(w);
        }

        sort_extracted(&mut extracted, &mut cnt, &mut ext_out);

        if !extracted.is_empty() {
            let elen = extracted.len();
            // SAFETY: extracted node ids are heads popped this round, hence in-range.
            unsafe {
                for i in 0..elen - 1 {
                    let idx = extracted.get_unchecked(i).1 as usize;
                    nodes.get_unchecked_mut(idx).next = extracted.get_unchecked(i + 1).1;
                }
                nodes.get_unchecked_mut(extracted.get_unchecked(elen - 1).1 as usize).next = -1;
                piles.push(extracted.get_unchecked(0).1);
            }
        }

        let mm = extracted.len().min(k_lim);
        if (t as usize) <= k_lim {
            for kk in 1..=k_lim {
                let v = if kk <= mm { extracted[kk - 1].0 } else { 1 };
                let base = off[kk];
                let fl = fill[kk];
                if fl < kk {
                    flat[base + fl] = v;
                    fill[kk] = fl + 1;
                } else {
                    let s = hd[kk];
                    flat[base + s] = v;
                    hd[kk] = if s + 1 == kk { 0 } else { s + 1 };
                }
            }
            stable = 0;
            continue;
        }

        let mut all_ok = true;
        for kk in 1..=k_lim {
            let v = if kk <= mm { extracted[kk - 1].0 } else { 1 };
            let s = hd[kk];
            let slot = off[kk] + s;
            if flat[slot] != v {
                all_ok = false;
            }
            flat[slot] = v;
            hd[kk] = if s + 1 == kk { 0 } else { s + 1 };
        }

        if all_ok {
            stable += 1;
            if stable >= streak_needed {
                patterns = vec![Vec::new(); k_lim + 1];
                for kk in 1..=k_lim {
                    let s = hd[kk];
                    let base = off[kk];
                    let mut pat = Vec::with_capacity(kk);
                    for i in 0..kk {
                        pat.push(flat[base + (s + i) % kk]);
                    }
                    if pat.iter().any(|&x| x != 1) {
                        kmax = kk;
                    }
                    patterns[kk] = pat;
                }
                found = true;
                break;
            }
        } else {
            stable = 0;
        }
    }

    if !found {
        panic!("periodicity not detected in {MAX_ROUNDS} rounds");
    }

    // x(u,k) = patterns[k][(u - t - 1) % k] for u > t.
    // After M rounds the number added at M-d has been divided d times.
    let r0 = M - t as u64 - 1;
    let mut total = 0u64;

    for d in 0..kmax {
        let r = r0 - d as u64;
        if patterns[d + 1][(r % (d as u64 + 1)) as usize] == 1 {
            continue;
        }
        let mut prod = 1u64;
        for k in ((d + 1)..=kmax).rev() {
            let v = patterns[k][(r % k as u64) as usize] as u64;
            if v != 1 {
                prod = prod * v % MOD;
            }
        }
        total += prod;
        if total >= MOD {
            total -= MOD;
        }
    }

    println!("{total}");
}
