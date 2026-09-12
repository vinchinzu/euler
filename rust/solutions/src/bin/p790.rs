// Project Euler 790 - Clock Grid
// Coordinate compression + segment tree with lazy shifts.

const NOPS: usize = 100_000;
const KK: i64 = 50_515_093;
const T: usize = 12;

#[inline(always)]
fn merge_node(idx: usize, hc: &mut [i64], shifts: &[i32]) {
    let left = idx * 2;
    let right = left + 1;
    let sl = unsafe { *shifts.get_unchecked(left) } as usize;
    let sr = unsafe { *shifts.get_unchecked(right) } as usize;
    let bo = idx * T;
    let bl = left * T;
    let br = right * T;
    // shifts are kept in 0..11, so (h - s) mod 12 = h + 12 - s, then one wrap
    unsafe {
        for h in 0..T {
            let mut il = h + T - sl;
            if il >= T {
                il -= T;
            }
            let mut ir = h + T - sr;
            if ir >= T {
                ir -= T;
            }
            *hc.get_unchecked_mut(bo + h) =
                *hc.get_unchecked(bl + il) + *hc.get_unchecked(br + ir);
        }
    }
}

fn update(
    from_idx: usize,
    to_idx: usize,
    diff: i32,
    index: usize,
    low: usize,
    high: usize,
    hc: &mut [i64],
    shifts: &mut [i32],
) {
    if from_idx >= high || to_idx <= low {
        return;
    }
    if from_idx <= low && to_idx >= high {
        // diff is ±1; keep shift in 0..11
        let s = unsafe { *shifts.get_unchecked(index) } + diff;
        unsafe {
            *shifts.get_unchecked_mut(index) = if s >= 12 {
                0
            } else if s < 0 {
                11
            } else {
                s
            };
        }
        return;
    }
    let mid = (low + high) >> 1;
    update(from_idx, to_idx, diff, 2 * index, low, mid, hc, shifts);
    update(from_idx, to_idx, diff, 2 * index + 1, mid, high, hc, shifts);
    merge_node(index, hc, shifts);
}

fn main() {
    let mut s = vec![0i64; 4 * NOPS];
    s[0] = 290797;
    for i in 1..4 * NOPS {
        s[i] = s[i - 1] * s[i - 1] % KK;
    }

    let mut q_x1 = vec![0i32; NOPS];
    let mut q_x2 = vec![0i32; NOPS];
    let mut q_y1 = vec![0i32; NOPS];
    let mut q_y2 = vec![0i32; NOPS];

    let mut xs_raw: Vec<i32> = Vec::with_capacity(2 * NOPS + 2);
    let mut ys_raw: Vec<i32> = Vec::with_capacity(2 * NOPS + 2);
    xs_raw.push(0);
    xs_raw.push(KK as i32);
    ys_raw.push(0);
    ys_raw.push(KK as i32);

    for t in 0..NOPS {
        let base = 4 * t;
        let (a, b, c, d) = (s[base] as i32, s[base + 1] as i32, s[base + 2] as i32, s[base + 3] as i32);
        if a <= b {
            q_x1[t] = a;
            q_x2[t] = b + 1;
        } else {
            q_x1[t] = b;
            q_x2[t] = a + 1;
        }
        if c <= d {
            q_y1[t] = c;
            q_y2[t] = d + 1;
        } else {
            q_y1[t] = d;
            q_y2[t] = c + 1;
        }
        xs_raw.push(q_x1[t]);
        xs_raw.push(q_x2[t]);
        ys_raw.push(q_y1[t]);
        ys_raw.push(q_y2[t]);
    }

    xs_raw.sort_unstable();
    xs_raw.dedup();
    ys_raw.sort_unstable();
    ys_raw.dedup();

    let nx = xs_raw.len();
    let ny_intervals = ys_raw.len() - 1;

    let mut seg_l = 1;
    while seg_l < ny_intervals {
        seg_l *= 2;
    }
    let tree_size = 2 * seg_l;

    let mut hc = vec![0i64; tree_size * T];
    let mut shifts = vec![0i32; tree_size];

    for i in 0..ny_intervals {
        hc[(seg_l + i) * T] = (ys_raw[i + 1] - ys_raw[i]) as i64;
    }

    for i in (1..seg_l).rev() {
        merge_node(i, &mut hc, &shifts);
    }

    struct Event {
        x: i32,
        yi1: usize,
        yi2: usize,
        diff: i32,
    }

    let mut events: Vec<Event> = Vec::with_capacity(2 * NOPS);
    for t in 0..NOPS {
        let yi1 = ys_raw.binary_search(&q_y1[t]).unwrap();
        let yi2 = ys_raw.binary_search(&q_y2[t]).unwrap();
        events.push(Event { x: q_x1[t], yi1, yi2, diff: 1 });
        events.push(Event { x: q_x2[t], yi1, yi2, diff: -1 });
    }
    events.sort_unstable_by_key(|e| e.x);

    let mut hval = [0i64; T];
    for i in 0..T {
        hval[i] = i as i64;
    }
    hval[0] = T as i64;

    let mut ans: i64 = 0;
    let mut prev_x: i32 = 0;
    let mut ev_idx = 0;

    for xi in 0..nx {
        let x = xs_raw[xi];
        let dx = (x - prev_x) as i64;
        if dx > 0 {
            let s1 = shifts[1] as usize;
            for h in 0..T {
                let mut ih = h + T - s1;
                if ih >= T {
                    ih -= T;
                }
                ans += hval[h] * hc[T + ih] * dx;
            }
        }

        while ev_idx < events.len() && events[ev_idx].x == x {
            let e = &events[ev_idx];
            update(e.yi1, e.yi2, e.diff, 1, 0, seg_l, &mut hc, &mut shifts);
            ev_idx += 1;
        }

        prev_x = x;
    }

    println!("{}", ans);
}
