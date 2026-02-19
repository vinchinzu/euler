// Project Euler 566 - Cake Cutting
//
// Given a circular cake with icing on top, if we repeatedly cut slices of
// lengths 360/a, 360/b, 360/sqrt(c), each time flipping, find F(a,b,c) =
// minimum flips until all icing returns to top.
// Compute sum F(a,b,c) for 9 <= a < b < c <= 53.

use rayon::prelude::*;
use std::collections::HashSet;

#[derive(Clone, Copy, PartialEq, Eq)]
struct Rad {
    a: i64,
    b: i64,
}

impl Rad {
    #[inline(always)]
    fn add(self, o: Rad) -> Rad {
        Rad { a: self.a + o.a, b: self.b + o.b }
    }
    #[inline(always)]
    fn sub(self, o: Rad) -> Rad {
        Rad { a: self.a - o.a, b: self.b - o.b }
    }
    #[inline(always)]
    fn val(self, sc: f64) -> f64 {
        self.a as f64 + self.b as f64 * sc
    }
    #[inline(always)]
    fn cmp_rad(self, o: Rad, sc: f64) -> i32 {
        let vx = self.val(sc);
        let vy = o.val(sc);
        if vx < vy { -1 } else if vx > vy { 1 } else { 0 }
    }
    #[inline(always)]
    fn min_rad(self, o: Rad, sc: f64) -> Rad {
        if self.cmp_rad(o, sc) <= 0 { self } else { o }
    }
    #[inline(always)]
    fn max_rad(self, o: Rad, sc: f64) -> Rad {
        if self.cmp_rad(o, sc) >= 0 { self } else { o }
    }
    #[inline(always)]
    fn one(d: i64) -> Rad { Rad { a: d, b: 0 } }
    #[inline(always)]
    fn zero() -> Rad { Rad { a: 0, b: 0 } }
}

#[derive(Clone, Copy)]
struct Interval {
    start: Rad,
    end: Rad,
}

#[inline(always)]
fn is_sq(n: i32) -> bool {
    let r = (n as f64).sqrt() as i32;
    r * r == n || (r + 1) * (r + 1) == n
}

#[inline(always)]
fn isqrt_i(n: i32) -> i32 {
    let mut r = (n as f64).sqrt() as i32;
    while r * r > n { r -= 1; }
    while (r + 1) * (r + 1) <= n { r += 1; }
    r
}

#[inline(always)]
fn gcd_ll(mut a: i64, mut b: i64) -> i64 {
    if a < 0 { a = -a; }
    if b < 0 { b = -b; }
    while b != 0 { let t = b; b = a % b; a = t; }
    a
}

#[inline(always)]
fn lcm_ll(a: i64, b: i64) -> i64 { a / gcd_ll(a, b) * b }

#[inline]
fn ext_gcd(a: i64, b: i64) -> (i64, i64) {
    if b == 0 { return (1, 0); }
    let (x1, y1) = ext_gcd(b, a % b);
    (y1, x1 - (a / b) * y1)
}

#[inline]
fn general_crt(a1: i64, m1: i64, a2: i64, m2: i64) -> i64 {
    let g = gcd_ll(m1, m2);
    if (a1 - a2) % g != 0 { return -1; }
    let (lx, _) = ext_gcd(m1, m2);
    let lcm = m1 / g * m2;
    let l = lcm as i128;
    let step = lx as i128 * ((a1 - a2) / g) as i128 % l;
    let r = (a1 as i128 - m1 as i128 * step) % l;
    ((r + l) % l) as i64
}

/// Reusable buffers for order_compute (one per thread)
struct State {
    intervals: Vec<Interval>,
    flip_arr: Vec<i32>,
    stride_angles: Vec<Rad>,
    stride_flipped: Vec<bool>,
    shift_results: Vec<i32>,
    orders: Vec<i64>,
    new_orders: Vec<i64>,
    order_set: HashSet<i64>,
    to_add: Vec<Interval>,
    merged_buf: Vec<Interval>,
}

impl State {
    fn new() -> Self {
        State {
            intervals: Vec::new(),
            flip_arr: Vec::new(),
            stride_angles: Vec::new(),
            stride_flipped: Vec::new(),
            shift_results: Vec::new(),
            orders: Vec::new(),
            new_orders: Vec::new(),
            order_set: HashSet::new(),
            to_add: Vec::new(),
            merged_buf: Vec::new(),
        }
    }
}

fn advance(intervals: &[Interval], cp: &mut Rad, sc: f64) {
    for iv in intervals {
        if iv.start.cmp_rad(*cp, sc) <= 0 && cp.cmp_rad(iv.end, sc) < 0 {
            *cp = iv.end;
            return;
        }
    }
}

/// Batch-insert sorted, pre-merged intervals into the main sorted interval set
fn batch_insert_intervals(intervals: &mut Vec<Interval>, add: &[Interval], merged_buf: &mut Vec<Interval>, sc: f64) {
    if add.is_empty() { return; }
    merged_buf.clear();
    let mut i = 0;
    let mut j = 0;
    while i < intervals.len() || j < add.len() {
        let next = if i < intervals.len() && (j >= add.len() || intervals[i].start.cmp_rad(add[j].start, sc) <= 0) {
            let iv = intervals[i]; i += 1; iv
        } else {
            let iv = add[j]; j += 1; iv
        };
        if let Some(last) = merged_buf.last_mut() {
            if next.start.cmp_rad(last.end, sc) <= 0 {
                last.end = last.end.max_rad(next.end, sc);
                continue;
            }
        }
        merged_buf.push(next);
    }
    std::mem::swap(intervals, merged_buf);
}

fn identical_shifts(flip_arr: &[i32], len: usize, k: usize, results: &mut Vec<i32>) {
    results.clear();
    let h: i64 = 3;
    let sublen = len / k;
    let mut pow_h: i64 = 1;
    for _ in 0..sublen { pow_h = pow_h.wrapping_mul(h); }

    let mut target: i64 = 0;
    let mut i = 0;
    while i < len {
        target = target.wrapping_mul(h).wrapping_add(flip_arr[i] as i64);
        i += k;
    }

    let factor = 1i64.wrapping_sub(pow_h);
    for init in 0..k {
        let mut hv: i64 = 0;
        let mut j = init;
        while j < len { hv = hv.wrapping_mul(h).wrapping_add(flip_arr[j] as i64); j += k; }
        j = init;
        while j < len {
            if hv == target { results.push(j as i32); }
            let v = flip_arr[j] as i64;
            hv = hv.wrapping_mul(h).wrapping_add(factor.wrapping_mul(v));
            j += k;
        }
    }
}

fn order_compute(st: &mut State, flip_sizes: &[Rad; 3], d: i64, sc: f64) -> i64 {
    let one = Rad::one(d);
    let n_flips = 3usize;
    st.orders.clear();
    st.orders.push(0);
    let mut period: i64 = 1;

    st.intervals.clear();
    let mut cp = Rad::zero();

    loop {
        advance(&st.intervals, &mut cp, sc);
        if cp.cmp_rad(one, sc) >= 0 { break; }

        let start_angle = cp;
        st.flip_arr.clear();
        st.stride_angles.clear();
        st.stride_flipped.clear();
        let mut uncut_region = one;
        let mut pos_angle = start_angle;
        let mut pos_flipped = false;
        let mut first = true;

        loop {
            if !first && pos_angle == start_angle && !pos_flipped {
                break;
            }
            first = false;
            for fi in 0..n_flips {
                st.flip_arr.push(if pos_flipped { 1 } else { 0 });
                if fi == 0 {
                    st.stride_angles.push(pos_angle);
                    st.stride_flipped.push(pos_flipped);
                }

                let size = flip_sizes[fi];
                let cmp_val = pos_angle.cmp_rad(size, sc);
                if cmp_val < (if pos_flipped { 1 } else { 0 }) {
                    uncut_region = uncut_region.min_rad(
                        if pos_flipped { pos_angle } else { size.sub(pos_angle) }, sc);
                    pos_angle = one.sub(pos_angle);
                    pos_flipped = !pos_flipped;
                } else {
                    uncut_region = uncut_region.min_rad(
                        if pos_flipped { pos_angle.sub(size) } else { one.sub(pos_angle) }, sc);
                    pos_angle = pos_angle.sub(size);
                }
            }
        }

        let n_pos = st.flip_arr.len();
        identical_shifts(&st.flip_arr, n_pos, n_flips, &mut st.shift_results);

        let curr_period = n_pos as i64;
        st.new_orders.clear();
        st.order_set.clear();
        for &ov in st.orders.iter() {
            for &sv in st.shift_results.iter() {
                let nv = general_crt(ov, period, sv as i64, curr_period);
                if nv != -1 && st.order_set.insert(nv) {
                    st.new_orders.push(nv);
                }
            }
        }
        period = lcm_ll(period, curr_period);
        std::mem::swap(&mut st.orders, &mut st.new_orders);

        // Collect non-flipped stride-3 intervals, sort, pre-merge, batch-insert
        st.to_add.clear();
        for i in 0..st.stride_angles.len() {
            if !st.stride_flipped[i] {
                let s = st.stride_angles[i];
                let e = s.add(uncut_region);
                st.to_add.push(Interval { start: s, end: e });
            }
        }
        st.to_add.sort_by(|a, b| {
            let va = a.start.val(sc);
            let vb = b.start.val(sc);
            va.partial_cmp(&vb).unwrap()
        });
        // Pre-merge adjacent/overlapping intervals
        let mut pre_merged: Vec<Interval> = Vec::new();
        for &iv in &st.to_add {
            if let Some(last) = pre_merged.last_mut() {
                if iv.start.cmp_rad(last.end, sc) <= 0 {
                    last.end = last.end.max_rad(iv.end, sc);
                    continue;
                }
            }
            pre_merged.push(iv);
        }
        batch_insert_intervals(&mut st.intervals, &pre_merged, &mut st.merged_buf, sc);
    }

    let mut min_order = period;
    for &o in st.orders.iter() {
        if o > 0 && o < min_order { min_order = o; }
    }
    min_order
}

fn f_func(st: &mut State, a: i32, b: i32, c: i32) -> i64 {
    let mut fs = [Rad::zero(); 3];
    let (d, sc);
    if is_sq(c) {
        let s = isqrt_i(c);
        sc = 1.0;
        d = a as i64 * b as i64 * s as i64;
        fs[0] = Rad { a: d / a as i64, b: 0 };
        fs[1] = Rad { a: d / b as i64, b: 0 };
        fs[2] = Rad { a: d / s as i64, b: 0 };
    } else {
        sc = (c as f64).sqrt();
        d = a as i64 * b as i64 * c as i64;
        fs[0] = Rad { a: d / a as i64, b: 0 };
        fs[1] = Rad { a: d / b as i64, b: 0 };
        fs[2] = Rad { a: 0, b: d / c as i64 };
    }
    order_compute(st, &fs, d, sc)
}

thread_local! {
    static TLS: std::cell::RefCell<State> = std::cell::RefCell::new(State::new());
}

fn main() {
    let nn = 53i32;
    let ans: i64 = (9..nn)
        .into_par_iter()
        .map(|a| {
            let mut s = 0i64;
            TLS.with(|cell| {
                let st = &mut *cell.borrow_mut();
                for b in (a + 1)..nn {
                    for c in (b + 1)..=nn {
                        s += f_func(st, a, b, c);
                    }
                }
            });
            s
        })
        .sum();
    println!("{}", ans);
}
