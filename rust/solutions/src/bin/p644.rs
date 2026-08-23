// Project Euler 644 - Squares on the Line
// Grundy intervals via exact a+b√2 event buckets; W(S) from slope events.

const A_PARAM: f64 = 200.0;
const B_PARAM: f64 = 500.0;
const SQRT2: f64 = std::f64::consts::SQRT_2;
const AMAX: u32 = 500;
const BMAX: u32 = 353;
const STRIDE: usize = BMAX as usize + 1;
const MS_MAX: usize = 4096;
const NIL: u32 = u32::MAX;

#[inline(always)]
fn sched(head: &mut [u32], next: &mut Vec<u32>, payload: &mut Vec<u32>, idx_of: &[u32], a: u32, b: u32, packed: u32) {
    if a > AMAX || b > BMAX {
        return;
    }
    // SAFETY: a <= AMAX, b <= BMAX, idx_of is (AMAX+1)*STRIDE
    let id = unsafe { *idx_of.get_unchecked(a as usize * STRIDE + b as usize) };
    if id != NIL {
        let node = next.len() as u32;
        next.push(unsafe { *head.get_unchecked(id as usize) });
        payload.push(packed);
        unsafe { *head.get_unchecked_mut(id as usize) = node; }
    }
}

fn main() {
    // All lattice points a + b√2 ≤ B_PARAM, sorted by value.
    let mut cells: Vec<(f64, u16, u16)> = Vec::with_capacity(90_000);
    for a in 0..=AMAX {
        let af = a as f64;
        if af > B_PARAM {
            break;
        }
        let b_lim = ((B_PARAM - af) / SQRT2).floor() as u32;
        let b_lim = b_lim.min(BMAX);
        for b in 0..=b_lim {
            let v = af + b as f64 * SQRT2;
            if v <= B_PARAM {
                cells.push((v, a as u16, b as u16));
            }
        }
    }
    cells.sort_unstable_by(|x, y| x.0.total_cmp(&y.0));
    let ncells = cells.len();

    let mut idx_of = vec![NIL; (AMAX as usize + 1) * STRIDE];
    for (i, &(_, a, b)) in cells.iter().enumerate() {
        idx_of[a as usize * STRIDE + b as usize] = i as u32;
    }

    // Linked-list event buckets (add: packed = value, remove: high bit set).
    let mut head = vec![NIL; ncells];
    let mut next: Vec<u32> = Vec::with_capacity(2_200_000);
    let mut payload: Vec<u32> = Vec::with_capacity(2_200_000);

    // Seed: add xor-value 0 at length 1 = (1,0).
    sched(&mut head, &mut next, &mut payload, &idx_of, 1, 0, 0);

    let mut ms_counts = [0i32; MS_MAX];
    let mut nim_a: Vec<u16> = vec![0];
    let mut nim_b: Vec<u16> = vec![0];
    let mut nim_val: Vec<i32> = vec![0];
    let mut nim_pos: Vec<f64> = vec![0.0];

    for ci in 0..ncells {
        let p = head[ci];
        if p == NIL {
            continue;
        }
        // Adds first, then removes (same as the original min-heap tie-break).
        let mut q = p;
        while q != NIL {
            let pl = unsafe { *payload.get_unchecked(q as usize) };
            if pl < 1 << 31 {
                let v = pl as usize;
                if v < MS_MAX {
                    ms_counts[v] += 1;
                }
            }
            q = unsafe { *next.get_unchecked(q as usize) };
        }
        q = p;
        while q != NIL {
            let pl = unsafe { *payload.get_unchecked(q as usize) };
            if pl >= 1 << 31 {
                let v = (pl & 0x7fff_ffff) as usize;
                if v < MS_MAX && ms_counts[v] > 0 {
                    ms_counts[v] -= 1;
                }
            }
            q = unsafe { *next.get_unchecked(q as usize) };
        }

        let mut nimber = 1i32;
        while (nimber as usize) < MS_MAX && ms_counts[nimber as usize] > 0 {
            nimber += 1;
        }
        if nimber == *nim_val.last().unwrap() {
            continue;
        }

        let (pos, ea16, eb16) = cells[ci];
        nim_a.push(ea16);
        nim_b.push(eb16);
        nim_val.push(nimber);
        nim_pos.push(pos);

        let n_entries = nim_a.len();
        let ea = ea16 as u32;
        let eb = eb16 as u32;
        let prev_last = nim_val[n_entries - 2];
        for i in 0..n_entries {
            let pa = nim_a[i] as u32;
            let pb = nim_b[i] as u32;
            let xor_val = (nimber ^ nim_val[i]) as u32;
            sched(&mut head, &mut next, &mut payload, &idx_of, ea + pa + 1, eb + pb, xor_val);
            sched(&mut head, &mut next, &mut payload, &idx_of, ea + pa, eb + pb + 1, xor_val);
            if i != 0 {
                let rem = (1u32 << 31) | ((prev_last ^ nim_val[i - 1]) as u32);
                sched(&mut head, &mut next, &mut payload, &idx_of, ea + pa + 1, eb + pb, rem);
                sched(&mut head, &mut next, &mut payload, &idx_of, ea + pa, eb + pb + 1, rem);
            }
        }
    }

    drop(head);
    drop(next);
    drop(payload);
    drop(idx_of);
    drop(cells);

    // Lattice sizes L = a + b√2 in [A_PARAM, B_PARAM], a,b ≥ 1.
    let mut sizes: Vec<(f64, u16, u16)> = Vec::with_capacity(80_000);
    for a in 1..=AMAX {
        let af = a as f64;
        if af > B_PARAM {
            break;
        }
        for b in 1..=BMAX {
            let size = af + b as f64 * SQRT2;
            if size > B_PARAM {
                break;
            }
            if size >= A_PARAM {
                sizes.push((size, a as u16, b as u16));
            }
        }
    }
    sizes.sort_unstable_by(|x, y| x.0.total_cmp(&y.0));

    let max_nimber = *nim_val.iter().max().unwrap_or(&0) as usize;
    let mut range_lists: Vec<Vec<(f64, f64)>> = vec![Vec::new(); max_nimber + 1];
    for i in 0..nim_pos.len() - 1 {
        range_lists[nim_val[i] as usize].push((nim_pos[i], nim_pos[i + 1]));
    }

    // Piecewise-linear W(S) = measure{x : g(x) = g(S − x)} via slope-change events.
    let mut events: Vec<(f64, i32)> = Vec::with_capacity(160_000);
    for intervals in &range_lists {
        let m = intervals.len();
        for i in 0..m {
            let (a1, b1) = intervals[i];
            for j in i..m {
                let (a2, b2) = intervals[j];
                let p0 = a1 + a2;
                let p3 = b1 + b2;
                if p0 > B_PARAM || p3 < A_PARAM {
                    continue;
                }
                let w = if i == j { 1 } else { 2 };
                let p1 = a1 + b2;
                let p2 = b1 + a2;
                let q1 = if p1 < p2 { p1 } else { p2 };
                let q2 = if p1 < p2 { p2 } else { p1 };
                events.push((p0, w));
                events.push((q1, -w));
                events.push((q2, -w));
                events.push((p3, w));
            }
        }
    }
    events.sort_unstable_by(|x, y| x.0.total_cmp(&y.0));

    let mut slope = 0i32;
    let mut val = 0.0f64;
    let mut prev = 0.0f64;
    let mut ei = 0usize;
    let mut prob = vec![0.0f64; (AMAX as usize + 1) * STRIDE];

    for &(size, a, b) in &sizes {
        while ei < events.len() && events[ei].0 < size {
            let (pos, delta) = events[ei];
            val += slope as f64 * (pos - prev);
            slope += delta;
            prev = pos;
            ei += 1;
        }
        let w = val + slope as f64 * (size - prev);
        prob[a as usize * STRIDE + b as usize] = w / size;
    }

    let mut ans = 0.0f64;
    for &(size, a, b) in &sizes {
        let p1 = prob[(a as usize - 1) * STRIDE + b as usize];
        let p2 = prob[a as usize * STRIDE + (b as usize - 1)];
        let l_val = size * (p1 + p2) * 0.5;
        if l_val > ans {
            ans = l_val;
        }
    }

    println!("{:.8}", ans);
}
