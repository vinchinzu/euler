// Project Euler 566 - Cake Cutting
//
// Given a circular cake with icing on top, if we repeatedly cut slices of
// lengths 360/a, 360/b, 360/sqrt(c), each time flipping, find F(a,b,c) =
// minimum flips until all icing returns to top.
// Compute sum F(a,b,c) for 9 <= a < b < c <= 53.

use rayon::prelude::*;
use std::cell::UnsafeCell;

const MAX_POSITIONS: usize = 200_000;
const MAX_INTERVALS: usize = 50_000;
const MAX_ORDERS: usize = 100_000;

#[derive(Clone, Copy)]
struct Rad {
    a: i64,
    b: i64,
}

impl Rad {
    #[inline(always)]
    fn add(self, other: Rad) -> Rad {
        Rad { a: self.a + other.a, b: self.b + other.b }
    }
    #[inline(always)]
    fn sub(self, other: Rad) -> Rad {
        Rad { a: self.a - other.a, b: self.b - other.b }
    }
    #[inline(always)]
    fn val(self, sqrt_c: f64) -> f64 {
        self.a as f64 + self.b as f64 * sqrt_c
    }
    #[inline(always)]
    fn cmp_rad(self, other: Rad, sqrt_c: f64) -> i32 {
        let vx = self.val(sqrt_c);
        let vy = other.val(sqrt_c);
        if vx < vy { -1 }
        else if vx > vy { 1 }
        else { 0 }
    }
    #[inline(always)]
    fn min_rad(self, other: Rad, sqrt_c: f64) -> Rad {
        if self.cmp_rad(other, sqrt_c) <= 0 { self } else { other }
    }
    #[inline(always)]
    fn max_rad(self, other: Rad, sqrt_c: f64) -> Rad {
        if self.cmp_rad(other, sqrt_c) >= 0 { self } else { other }
    }
    #[inline(always)]
    fn one(d: i64) -> Rad {
        Rad { a: d, b: 0 }
    }
    #[inline(always)]
    fn zero() -> Rad {
        Rad { a: 0, b: 0 }
    }
}

#[derive(Clone, Copy)]
struct Pos {
    angle: Rad,
    flipped: bool,
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
    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }
    a
}

#[inline(always)]
fn lcm_ll(a: i64, b: i64) -> i64 {
    a / gcd_ll(a, b) * b
}

#[inline(always)]
fn mod_ll(a: i64, m: i64) -> i64 {
    ((a % m) + m) % m
}

#[inline]
fn ext_gcd(a: i64, b: i64) -> (i64, i64) {
    if b == 0 {
        return (1, 0);
    }
    let (x1, y1) = ext_gcd(b, a % b);
    (y1, x1 - (a / b) * y1)
}

#[inline]
fn general_crt(a1: i64, m1: i64, a2: i64, m2: i64) -> i64 {
    let g = gcd_ll(m1, m2);
    if (a1 - a2) % g != 0 {
        return -1;
    }
    let (lx, _) = ext_gcd(m1, m2);
    let lcm = m1 / g * m2;
    mod_ll(a1 - m1 * ((lx as i128 * ((a1 - a2) / g) as i128 % lcm as i128) as i64), lcm)
}

// Pre-allocated buffers for order_compute
struct OrderState {
    intervals: Box<[Interval]>,
    n_intervals: usize,
    current_point: Rad,
    positions: Box<[Pos]>,
    n_positions: usize,
    shift_results: Box<[i32]>,
    n_shift_results: usize,
    orders: Box<[i64]>,
    new_orders: Box<[i64]>,
    new_ints: Box<[Interval]>,
    flip_arr: Box<[i32]>,
}

impl OrderState {
    fn new() -> Self {
        let dummy_iv = Interval { start: Rad::zero(), end: Rad::zero() };
        let dummy_pos = Pos { angle: Rad::zero(), flipped: false };
        OrderState {
            intervals: vec![dummy_iv; MAX_INTERVALS].into_boxed_slice(),
            n_intervals: 0,
            current_point: Rad::zero(),
            positions: vec![dummy_pos; MAX_POSITIONS].into_boxed_slice(),
            n_positions: 0,
            shift_results: vec![0i32; MAX_POSITIONS].into_boxed_slice(),
            n_shift_results: 0,
            orders: vec![0i64; MAX_ORDERS].into_boxed_slice(),
            new_orders: vec![0i64; MAX_ORDERS].into_boxed_slice(),
            new_ints: vec![dummy_iv; MAX_INTERVALS].into_boxed_slice(),
            flip_arr: vec![0i32; MAX_POSITIONS].into_boxed_slice(),
        }
    }

    #[inline]
    fn intervals_init(&mut self) {
        self.n_intervals = 0;
        self.current_point = Rad::zero();
    }

    #[inline]
    fn advance_to_next_unprocessed(&mut self, sqrt_c: f64) {
        for i in 0..self.n_intervals {
            let iv = unsafe { *self.intervals.get_unchecked(i) };
            if iv.start.cmp_rad(self.current_point, sqrt_c) <= 0
                && self.current_point.cmp_rad(iv.end, sqrt_c) < 0
            {
                self.current_point = iv.end;
                return;
            }
        }
    }

    fn process_interval(&mut self, start: Rad, end: Rad, sqrt_c: f64) {
        let mut ms = start;
        let mut me = end;
        let mut nn = 0usize;
        for i in 0..self.n_intervals {
            let iv = unsafe { *self.intervals.get_unchecked(i) };
            if iv.end.cmp_rad(start, sqrt_c) < 0 || end.cmp_rad(iv.start, sqrt_c) < 0 {
                unsafe { *self.new_ints.get_unchecked_mut(nn) = iv; }
                nn += 1;
            } else {
                ms = ms.min_rad(iv.start, sqrt_c);
                me = me.max_rad(iv.end, sqrt_c);
            }
        }
        unsafe { *self.new_ints.get_unchecked_mut(nn) = Interval { start: ms, end: me }; }
        nn += 1;
        for i in 0..nn.saturating_sub(1) {
            for j in (i + 1)..nn {
                unsafe {
                    if self.new_ints.get_unchecked(i).start.cmp_rad(
                        self.new_ints.get_unchecked(j).start, sqrt_c) > 0 {
                        let tmp = *self.new_ints.get_unchecked(i);
                        *self.new_ints.get_unchecked_mut(i) = *self.new_ints.get_unchecked(j);
                        *self.new_ints.get_unchecked_mut(j) = tmp;
                    }
                }
            }
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                self.new_ints.as_ptr(),
                self.intervals.as_mut_ptr(),
                nn,
            );
        }
        self.n_intervals = nn;
    }

    #[inline]
    fn intervals_done(&self, d: i64, sqrt_c: f64) -> bool {
        self.current_point.cmp_rad(Rad::one(d), sqrt_c) >= 0
    }

    fn identical_shifts(&mut self, len: usize, k: usize) {
        self.n_shift_results = 0;
        let h: i64 = 3;
        let sublen = len / k;
        let mut pow_h: i64 = 1;
        for _ in 0..sublen {
            pow_h = pow_h.wrapping_mul(h);
        }

        let mut target_hash: i64 = 0;
        let mut i = 0;
        while i < len {
            target_hash = target_hash.wrapping_mul(h).wrapping_add(unsafe { *self.flip_arr.get_unchecked(i) } as i64);
            i += k;
        }

        let factor = 1i64.wrapping_sub(pow_h);
        for initial_shift in 0..k {
            let mut hash_val: i64 = 0;
            let mut i = initial_shift;
            while i < len {
                hash_val = hash_val.wrapping_mul(h).wrapping_add(unsafe { *self.flip_arr.get_unchecked(i) } as i64);
                i += k;
            }
            i = initial_shift;
            while i < len {
                if hash_val == target_hash {
                    unsafe { *self.shift_results.get_unchecked_mut(self.n_shift_results) = i as i32; }
                    self.n_shift_results += 1;
                }
                let v = unsafe { *self.flip_arr.get_unchecked(i) } as i64;
                hash_val = hash_val.wrapping_mul(h).wrapping_add(factor.wrapping_mul(v));
                i += k;
            }
        }
    }
}

fn order_compute(state: &mut OrderState, flip_sizes: &[Rad; 3], d: i64, sqrt_c: f64) -> i32 {
    let one = Rad::one(d);
    let n_flips = 3usize;
    let mut n_orders: usize = 1;
    state.orders[0] = 0;
    let mut period: i64 = 1;

    state.intervals_init();

    loop {
        state.advance_to_next_unprocessed(sqrt_c);
        if state.intervals_done(d, sqrt_c) {
            break;
        }

        let start_pos = Pos { angle: state.current_point, flipped: false };
        state.n_positions = 0;
        let mut uncut_region = one;
        let mut pos = start_pos;
        let mut first = true;

        loop {
            if !first
                && pos.angle.cmp_rad(start_pos.angle, sqrt_c) == 0
                && pos.flipped == start_pos.flipped
            {
                break;
            }
            first = false;
            for fi in 0..n_flips {
                if state.n_positions >= MAX_POSITIONS {
                    break;
                }
                unsafe { *state.positions.get_unchecked_mut(state.n_positions) = pos; }
                state.n_positions += 1;
                let size = flip_sizes[fi];
                let cmp_val = pos.angle.cmp_rad(size, sqrt_c);
                if cmp_val < (if pos.flipped { 1 } else { 0 }) {
                    uncut_region = uncut_region.min_rad(
                        if pos.flipped { pos.angle } else { size.sub(pos.angle) },
                        sqrt_c,
                    );
                    pos = Pos {
                        angle: one.sub(pos.angle),
                        flipped: !pos.flipped,
                    };
                } else {
                    uncut_region = uncut_region.min_rad(
                        if pos.flipped {
                            pos.angle.sub(size)
                        } else {
                            one.sub(pos.angle)
                        },
                        sqrt_c,
                    );
                    pos = Pos {
                        angle: pos.angle.sub(size),
                        flipped: pos.flipped,
                    };
                }
            }
        }

        // Build flip array
        for i in 0..state.n_positions {
            unsafe {
                *state.flip_arr.get_unchecked_mut(i) = if state.positions.get_unchecked(i).flipped { 1 } else { 0 };
            }
        }

        state.identical_shifts(state.n_positions, n_flips);

        let curr_period = state.n_positions as i64;
        let mut n_new: usize = 0;
        for oi in 0..n_orders {
            let order_val = unsafe { *state.orders.get_unchecked(oi) };
            for si in 0..state.n_shift_results {
                let nv = general_crt(
                    order_val,
                    period,
                    unsafe { *state.shift_results.get_unchecked(si) } as i64,
                    curr_period,
                );
                if nv != -1 {
                    let mut found = false;
                    for k in 0..n_new {
                        if unsafe { *state.new_orders.get_unchecked(k) } == nv {
                            found = true;
                            break;
                        }
                    }
                    if !found && n_new < MAX_ORDERS {
                        unsafe { *state.new_orders.get_unchecked_mut(n_new) = nv; }
                        n_new += 1;
                    }
                }
            }
        }
        period = lcm_ll(period, curr_period);
        unsafe {
            std::ptr::copy_nonoverlapping(
                state.new_orders.as_ptr(),
                state.orders.as_mut_ptr(),
                n_new,
            );
        }
        n_orders = n_new;

        // Process regions
        let mut i = 0;
        while i < state.n_positions {
            let p = unsafe { *state.positions.get_unchecked(i) };
            if !p.flipped {
                let start = p.angle;
                let end = start.add(uncut_region);
                state.process_interval(start, end, sqrt_c);
            }
            i += n_flips;
        }
    }

    let mut min_order = period;
    for i in 0..n_orders {
        let v = unsafe { *state.orders.get_unchecked(i) };
        if v > 0 && v < min_order {
            min_order = v;
        }
    }
    min_order as i32
}

fn f_func(state: &mut OrderState, a: i32, b: i32, c: i32) -> i32 {
    let mut flip_sizes = [Rad::zero(); 3];
    let (d, sqrt_c_val);
    if is_sq(c) {
        let sc = isqrt_i(c);
        sqrt_c_val = 1.0;
        d = a as i64 * b as i64 * sc as i64;
        flip_sizes[0] = Rad { a: d / a as i64, b: 0 };
        flip_sizes[1] = Rad { a: d / b as i64, b: 0 };
        flip_sizes[2] = Rad { a: d / sc as i64, b: 0 };
    } else {
        sqrt_c_val = (c as f64).sqrt();
        d = a as i64 * b as i64 * c as i64;
        flip_sizes[0] = Rad { a: d / a as i64, b: 0 };
        flip_sizes[1] = Rad { a: d / b as i64, b: 0 };
        flip_sizes[2] = Rad { a: 0, b: d / c as i64 };
    }
    order_compute(state, &flip_sizes, d, sqrt_c_val)
}

// Thread-local wrapper for OrderState
struct TlsState(UnsafeCell<OrderState>);
unsafe impl Sync for TlsState {}

thread_local! {
    static TLS_STATE: TlsState = TlsState(UnsafeCell::new(OrderState::new()));
}

fn main() {
    let nn: i32 = 53;

    // Build all work items as (a,b,c) triples for maximum parallelism granularity
    let mut work: Vec<(i32, i32, i32)> = Vec::new();
    for a in 9..nn {
        for b in (a + 1)..nn {
            for c in (b + 1)..=nn {
                work.push((a, b, c));
            }
        }
    }

    let ans: i64 = work.par_iter()
        .map(|&(a, b, c)| {
            TLS_STATE.with(|tls| {
                let state = unsafe { &mut *tls.0.get() };
                f_func(state, a, b, c) as i64
            })
        })
        .sum();

    println!("{}", ans);
}
