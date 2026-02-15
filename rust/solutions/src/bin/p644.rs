// Project Euler 644 - Squares on the Line
// Event-driven nimber simulation with sweep-line probability computation

use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::cmp::Ordering;

const A_PARAM: f64 = 200.0;
const B_PARAM: f64 = 500.0;
const SQRT2: f64 = std::f64::consts::SQRT_2;
const EPS: f64 = 1e-10;
const MS_MAX: usize = 4096;

#[derive(Clone)]
struct Event {
    pos: f64,
    is_remove: bool,
    counter: i32,
    value: i32,
}

impl PartialEq for Event {
    fn eq(&self, other: &Self) -> bool {
        self.pos == other.pos && self.is_remove == other.is_remove && self.counter == other.counter
    }
}
impl Eq for Event {}

impl PartialOrd for Event {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> { Some(self.cmp(other)) }
}
impl Ord for Event {
    fn cmp(&self, other: &Self) -> Ordering {
        // Min-heap: reverse ordering
        other.pos.partial_cmp(&self.pos).unwrap_or(Ordering::Equal)
            .then_with(|| (other.is_remove as u8).cmp(&(self.is_remove as u8)))
            .then_with(|| other.counter.cmp(&self.counter))
    }
}

#[derive(Clone)]
struct Event2 {
    pos: f64,
    is_remove: bool,
    pid: usize,
}

impl PartialEq for Event2 {
    fn eq(&self, other: &Self) -> bool { self.pos == other.pos && self.is_remove == other.is_remove }
}
impl Eq for Event2 {}

impl PartialOrd for Event2 {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> { Some(self.cmp(other)) }
}
impl Ord for Event2 {
    fn cmp(&self, other: &Self) -> Ordering {
        other.pos.partial_cmp(&self.pos).unwrap_or(Ordering::Equal)
            .then_with(|| (other.is_remove as u8).cmp(&(self.is_remove as u8)))
    }
}

struct RangePair {
    r1_lo: f64, r1_hi: f64,
    r2_lo: f64, r2_hi: f64,
}

fn main() {
    let mut counter = 0i32;

    // Phase 1: Compute nimbers
    let mut ms_counts = [0i32; MS_MAX];
    let mut nim_pos: Vec<f64> = vec![0.0];
    let mut nim_val: Vec<i32> = vec![0];

    let mut heap1: BinaryHeap<Event> = BinaryHeap::new();
    heap1.push(Event { pos: 1.0, is_remove: false, counter: { let c = counter; counter += 1; c }, value: 0 });

    while let Some(ev) = heap1.pop() {
        if ev.pos > B_PARAM { break; }

        if ev.is_remove {
            if (ev.value as usize) < MS_MAX && ms_counts[ev.value as usize] > 0 {
                ms_counts[ev.value as usize] -= 1;
            }
        } else {
            if (ev.value as usize) < MS_MAX {
                ms_counts[ev.value as usize] += 1;
            }
        }

        if let Some(next) = heap1.peek() {
            if (next.pos - ev.pos).abs() < EPS { continue; }
        }

        let mut nimber = 1i32;
        while (nimber as usize) < MS_MAX && ms_counts[nimber as usize] > 0 { nimber += 1; }

        if nimber == *nim_val.last().unwrap() { continue; }

        nim_pos.push(ev.pos);
        nim_val.push(nimber);

        let n_entries = nim_pos.len();
        for i in 0..n_entries {
            let pos = nim_pos[i];
            let xor_val = nimber ^ nim_val[i];
            heap1.push(Event { pos: ev.pos + pos + 1.0, is_remove: false, counter: { let c = counter; counter += 1; c }, value: xor_val });
            heap1.push(Event { pos: ev.pos + pos + SQRT2, is_remove: false, counter: { let c = counter; counter += 1; c }, value: xor_val });
            if pos != 0.0 {
                let prev_last_nimber = nim_val[nim_val.len() - 2];
                let prev_pos_nimber = if i > 0 { nim_val[i - 1] } else { 0 };
                let new_nimber = prev_last_nimber ^ prev_pos_nimber;
                heap1.push(Event { pos: ev.pos + pos + 1.0, is_remove: true, counter: { let c = counter; counter += 1; c }, value: new_nimber });
                heap1.push(Event { pos: ev.pos + pos + SQRT2, is_remove: true, counter: { let c = counter; counter += 1; c }, value: new_nimber });
            }
        }
    }

    // Phase 2: Compute sizes
    let mut sizes: Vec<f64> = Vec::new();
    for a in 1..=B_PARAM as i32 {
        let mut b = 1;
        loop {
            let size = a as f64 + b as f64 * SQRT2;
            if size > B_PARAM { break; }
            if size >= A_PARAM { sizes.push(size); }
            b += 1;
        }
    }
    sizes.sort_by(|a, b| a.partial_cmp(b).unwrap());

    // Phase 3: Build ranges grouped by nimber value
    let max_nimber = *nim_val.iter().max().unwrap_or(&0) as usize;

    let mut range_lists: Vec<Vec<(f64, f64)>> = vec![Vec::new(); max_nimber + 1];
    for i in 0..nim_pos.len() - 1 {
        let v = nim_val[i] as usize;
        range_lists[v].push((nim_pos[i], nim_pos[i + 1]));
    }

    // Phase 4: Build RangePair events
    let mut pairs: Vec<RangePair> = Vec::new();
    let mut heap2: BinaryHeap<Event2> = BinaryHeap::new();

    for v in 0..=max_nimber {
        let nc = range_lists[v].len();
        for j in 0..nc {
            for k in 0..nc {
                let (r1_lo, r1_hi) = range_lists[v][j];
                let (r2_lo, r2_hi) = range_lists[v][k];
                let pid = pairs.len();
                pairs.push(RangePair { r1_lo, r1_hi, r2_lo, r2_hi });
                heap2.push(Event2 { pos: r1_lo + r2_lo, is_remove: false, pid });
                heap2.push(Event2 { pos: r1_hi + r2_hi, is_remove: true, pid });
            }
        }
    }

    // Phase 5: For each size, compute probability
    let mut active_bits = vec![false; pairs.len()];
    let mut active_list: Vec<usize> = Vec::new();
    let mut prob_map: HashMap<i64, f64> = HashMap::new();

    for si in 0..sizes.len() {
        let size = sizes[si];

        while let Some(top) = heap2.peek() {
            if top.pos >= size { break; }
            let ev = heap2.pop().unwrap();
            if !ev.is_remove {
                active_bits[ev.pid] = true;
                active_list.push(ev.pid);
            } else {
                active_bits[ev.pid] = false;
            }
        }

        let mut prob = 0.0f64;
        for &pid in &active_list {
            if !active_bits[pid] { continue; }
            let rp = &pairs[pid];
            let mut lo = rp.r1_lo;
            if size - rp.r2_hi > lo { lo = size - rp.r2_hi; }
            let mut hi = rp.r1_hi;
            if size - rp.r2_lo < hi { hi = size - rp.r2_lo; }
            let intersection = hi - lo;
            if intersection > 0.0 {
                prob += intersection / size;
            }
        }
        let key = (size * 1e8).round() as i64;
        prob_map.insert(key, prob);
    }

    // Phase 6: Find maximum L * f(L)
    let mut ans = 0.0f64;
    for &size in &sizes {
        let s1 = size - 1.0;
        let s2 = size - SQRT2;
        let k1 = (s1 * 1e8).round() as i64;
        let k2 = (s2 * 1e8).round() as i64;
        let p1 = *prob_map.get(&k1).unwrap_or(&0.0);
        let p2 = *prob_map.get(&k2).unwrap_or(&0.0);
        let l_val = size * (p1 + p2) / 2.0;
        if l_val > ans { ans = l_val; }
    }

    println!("{:.8}", ans);
}
