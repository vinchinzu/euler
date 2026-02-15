// Project Euler 790 - Clock Grid
// Coordinate compression + segment tree with lazy shifts.

const NOPS: usize = 100_000;
const KK: i64 = 50_515_093;
const T: usize = 12;

fn main() {
    // Generate random sequence
    let mut s = vec![0i64; 4 * NOPS];
    s[0] = 290797;
    for i in 1..4 * NOPS {
        s[i] = s[i - 1] * s[i - 1] % KK;
    }

    // Parse operations
    let mut q_x1 = vec![0i32; NOPS];
    let mut q_x2 = vec![0i32; NOPS];
    let mut q_y1 = vec![0i32; NOPS];
    let mut q_y2 = vec![0i32; NOPS];

    let mut xs_raw: Vec<i32> = vec![0, KK as i32];
    let mut ys_raw: Vec<i32> = vec![0, KK as i32];

    for t in 0..NOPS {
        let base = 4 * t;
        let (a, b, c, d) = (s[base] as i32, s[base + 1] as i32, s[base + 2] as i32, s[base + 3] as i32);
        if a <= b { q_x1[t] = a; q_x2[t] = b + 1; }
        else { q_x1[t] = b; q_x2[t] = a + 1; }
        if c <= d { q_y1[t] = c; q_y2[t] = d + 1; }
        else { q_y1[t] = d; q_y2[t] = c + 1; }

        xs_raw.push(q_x1[t]); xs_raw.push(q_x2[t]);
        ys_raw.push(q_y1[t]); ys_raw.push(q_y2[t]);
    }

    xs_raw.sort_unstable();
    xs_raw.dedup();
    ys_raw.sort_unstable();
    ys_raw.dedup();

    let nx = xs_raw.len();
    let ny_intervals = ys_raw.len() - 1;

    // Binary search helper
    let bsearch_y = |val: i32| -> usize {
        ys_raw.binary_search(&val).unwrap()
    };

    // Segment tree
    let mut seg_l = 1;
    while seg_l < ny_intervals { seg_l *= 2; }
    let tree_size = 2 * seg_l;

    let mut hc = vec![0i64; tree_size * T];
    let mut shifts = vec![0i32; tree_size];

    // Initialize leaves
    for i in 0..ny_intervals {
        hc[(seg_l + i) * T] = (ys_raw[i + 1] - ys_raw[i]) as i64;
    }

    // Build tree
    fn merge(idx: usize, hc: &mut [i64], shifts: &[i32]) {
        let left = 2 * idx;
        let right = 2 * idx + 1;
        let sl = shifts[left];
        let sr = shifts[right];
        for h in 0..T {
            hc[idx * T + h] = hc[left * T + ((h as i32 - sl) % T as i32 + T as i32) as usize % T]
                + hc[right * T + ((h as i32 - sr) % T as i32 + T as i32) as usize % T];
        }
    }

    for i in (1..seg_l).rev() {
        merge(i, &mut hc, &shifts);
    }

    // Update function
    fn update(from_idx: usize, to_idx: usize, diff: i32, index: usize, low: usize, high: usize, hc: &mut [i64], shifts: &mut [i32]) {
        if from_idx >= high || to_idx <= low { return; }
        if from_idx <= low && to_idx >= high {
            shifts[index] += diff;
            return;
        }
        let mid = (low + high) >> 1;
        update(from_idx, to_idx, diff, 2 * index, low, mid, hc, shifts);
        update(from_idx, to_idx, diff, 2 * index + 1, mid, high, hc, shifts);
        // Merge
        let left = 2 * index;
        let right = 2 * index + 1;
        let sl = shifts[left];
        let sr = shifts[right];
        for h in 0..T {
            hc[index * T + h] = hc[left * T + ((h as i32 - sl) % T as i32 + T as i32) as usize % T]
                + hc[right * T + ((h as i32 - sr) % T as i32 + T as i32) as usize % T];
        }
    }

    // Events
    struct Event {
        x: i32,
        yi1: usize,
        yi2: usize,
        diff: i32,
    }

    let mut events: Vec<Event> = Vec::with_capacity(2 * NOPS);
    for t in 0..NOPS {
        let yi1 = bsearch_y(q_y1[t]);
        let yi2 = bsearch_y(q_y2[t]);
        events.push(Event { x: q_x1[t], yi1, yi2, diff: 1 });
        events.push(Event { x: q_x2[t], yi1, yi2, diff: -1 });
    }
    events.sort_by_key(|e| e.x);

    let mut hval = [0i64; T];
    for i in 0..T { hval[i] = i as i64; }
    hval[0] = T as i64;

    let mut ans: i64 = 0;
    let mut prev_x: i32 = 0;
    let mut ev_idx = 0;

    for xi in 0..nx {
        let x = xs_raw[xi];
        let dx = (x - prev_x) as i64;
        if dx > 0 {
            let s1 = shifts[1];
            for h in 0..T {
                ans += hval[h] * hc[T + ((h as i32 - s1) % T as i32 + T as i32) as usize % T] * dx;
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
