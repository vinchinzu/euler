// Project Euler 566 - Cake Cutting
//
// Given a circular cake with icing on top, if we repeatedly cut slices of
// lengths 360/a, 360/b, 360/sqrt(c), each time flipping, find F(a,b,c) =
// minimum flips until all icing returns to top.
// Compute sum F(a,b,c) for 9 <= a < b < c <= 53.

const MAX_POSITIONS: usize = 200_000;
const MAX_INTERVALS: usize = 50_000;
const MAX_ORDERS: usize = 100_000;

static mut GLOB_SQRT_C: f64 = 0.0;
static mut GLOB_D: i64 = 0;

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
    fn val(self) -> f64 {
        unsafe { self.a as f64 + self.b as f64 * GLOB_SQRT_C }
    }
    #[inline(always)]
    fn cmp_rad(self, other: Rad) -> i32 {
        let vx = self.val();
        let vy = other.val();
        if vx < vy { -1 }
        else if vx > vy { 1 }
        else { 0 }
    }
    #[inline(always)]
    fn min_rad(self, other: Rad) -> Rad {
        if self.cmp_rad(other) <= 0 { self } else { other }
    }
    #[inline(always)]
    fn max_rad(self, other: Rad) -> Rad {
        if self.cmp_rad(other) >= 0 { self } else { other }
    }
    #[inline(always)]
    fn one() -> Rad {
        Rad { a: unsafe { GLOB_D }, b: 0 }
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

fn ext_gcd(a: i64, b: i64, x: &mut i64, y: &mut i64) {
    if b == 0 {
        *x = 1;
        *y = 0;
        return;
    }
    let mut x1: i64 = 0;
    let mut y1: i64 = 0;
    ext_gcd(b, a % b, &mut x1, &mut y1);
    *x = y1;
    *y = x1 - (a / b) * y1;
}

fn general_crt(a1: i64, m1: i64, a2: i64, m2: i64) -> i64 {
    let g = gcd_ll(m1, m2);
    if (a1 - a2) % g != 0 {
        return -1;
    }
    let mut lx: i64 = 0;
    let mut ly: i64 = 0;
    ext_gcd(m1, m2, &mut lx, &mut ly);
    let lcm = m1 / g * m2;
    mod_ll(a1 - m1 * ((lx as i128 * ((a1 - a2) / g) as i128 % lcm as i128) as i64), lcm)
}

// Pre-allocated buffers for order_compute
struct OrderState {
    intervals: [Interval; MAX_INTERVALS],
    n_intervals: usize,
    current_point: Rad,
    positions: [Pos; MAX_POSITIONS],
    n_positions: usize,
    shift_results: [i32; MAX_POSITIONS],
    n_shift_results: usize,
    orders: [i64; MAX_ORDERS],
    new_orders: [i64; MAX_ORDERS],
    new_ints: [Interval; MAX_INTERVALS],
    flip_arr: [i32; MAX_POSITIONS],
}

impl OrderState {
    fn new() -> Box<Self> {
        // Use Box to avoid stack overflow with large arrays
        unsafe {
            let layout = std::alloc::Layout::new::<Self>();
            let ptr = std::alloc::alloc_zeroed(layout) as *mut Self;
            Box::from_raw(ptr)
        }
    }

    fn intervals_init(&mut self) {
        self.n_intervals = 0;
        self.current_point = Rad::zero();
    }

    fn advance_to_next_unprocessed(&mut self) {
        for i in 0..self.n_intervals {
            let iv = self.intervals[i];
            if iv.start.cmp_rad(self.current_point) <= 0
                && self.current_point.cmp_rad(iv.end) < 0
            {
                self.current_point = iv.end;
                return;
            }
        }
    }

    fn process_interval(&mut self, start: Rad, end: Rad) {
        let mut ms = start;
        let mut me = end;
        let mut nn = 0usize;
        for i in 0..self.n_intervals {
            let iv = self.intervals[i];
            if iv.end.cmp_rad(start) < 0 || end.cmp_rad(iv.start) < 0 {
                self.new_ints[nn] = iv;
                nn += 1;
            } else {
                ms = ms.min_rad(iv.start);
                me = me.max_rad(iv.end);
            }
        }
        self.new_ints[nn] = Interval { start: ms, end: me };
        nn += 1;
        // Insertion sort by start (small arrays)
        for i in 0..nn - 1 {
            for j in (i + 1)..nn {
                if self.new_ints[i].start.cmp_rad(self.new_ints[j].start) > 0 {
                    let tmp = self.new_ints[i];
                    self.new_ints[i] = self.new_ints[j];
                    self.new_ints[j] = tmp;
                }
            }
        }
        self.intervals[..nn].copy_from_slice(&self.new_ints[..nn]);
        self.n_intervals = nn;
    }

    fn intervals_done(&self) -> bool {
        self.current_point.cmp_rad(Rad::one()) >= 0
    }

    fn identical_shifts(&mut self, len: usize, k: usize) {
        self.n_shift_results = 0;
        let h: i64 = 3;
        let sublen = len / k;
        let mut pow_h: i64 = 1;
        for _ in 0..sublen {
            pow_h *= h;
        }

        let mut target_hash: i64 = 0;
        let mut i = 0;
        while i < len {
            target_hash = target_hash * h + self.flip_arr[i] as i64;
            i += k;
        }

        for initial_shift in 0..k {
            let mut hash_val: i64 = 0;
            let mut i = initial_shift;
            while i < len {
                hash_val = hash_val * h + self.flip_arr[i] as i64;
                i += k;
            }
            i = initial_shift;
            while i < len {
                if hash_val == target_hash {
                    self.shift_results[self.n_shift_results] = i as i32;
                    self.n_shift_results += 1;
                }
                hash_val = hash_val * h + (1 - pow_h) * self.flip_arr[i] as i64;
                i += k;
            }
        }
    }
}

fn order_compute(state: &mut OrderState, flip_sizes: &[Rad; 3], n_flips: usize) -> i32 {
    let mut n_orders: usize = 1;
    state.orders[0] = 0;
    let mut period: i64 = 1;

    state.intervals_init();

    loop {
        state.advance_to_next_unprocessed();
        if state.intervals_done() {
            break;
        }

        let start_pos = Pos { angle: state.current_point, flipped: false };
        state.n_positions = 0;
        let mut uncut_region = Rad::one();
        let mut pos = start_pos;
        let mut first = true;

        loop {
            if !first
                && pos.angle.cmp_rad(start_pos.angle) == 0
                && pos.flipped == start_pos.flipped
            {
                break;
            }
            first = false;
            for fi in 0..n_flips {
                if state.n_positions >= MAX_POSITIONS {
                    break;
                }
                state.positions[state.n_positions] = pos;
                state.n_positions += 1;
                let size = flip_sizes[fi];
                let cmp_val = pos.angle.cmp_rad(size);
                if cmp_val < (if pos.flipped { 1 } else { 0 }) {
                    uncut_region = uncut_region.min_rad(
                        if pos.flipped { pos.angle } else { size.sub(pos.angle) },
                    );
                    pos = Pos {
                        angle: Rad::one().sub(pos.angle),
                        flipped: !pos.flipped,
                    };
                } else {
                    uncut_region = uncut_region.min_rad(
                        if pos.flipped {
                            pos.angle.sub(size)
                        } else {
                            Rad::one().sub(pos.angle)
                        },
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
            state.flip_arr[i] = if state.positions[i].flipped { 1 } else { 0 };
        }

        state.identical_shifts(state.n_positions, n_flips);

        let curr_period = state.n_positions as i64;
        let mut n_new: usize = 0;
        for oi in 0..n_orders {
            for si in 0..state.n_shift_results {
                let nv = general_crt(
                    state.orders[oi],
                    period,
                    state.shift_results[si] as i64,
                    curr_period,
                );
                if nv != -1 {
                    // Check if already in new_orders
                    let mut found = false;
                    for k in 0..n_new {
                        if state.new_orders[k] == nv {
                            found = true;
                            break;
                        }
                    }
                    if !found && n_new < MAX_ORDERS {
                        state.new_orders[n_new] = nv;
                        n_new += 1;
                    }
                }
            }
        }
        period = lcm_ll(period, curr_period);
        state.orders[..n_new].copy_from_slice(&state.new_orders[..n_new]);
        n_orders = n_new;

        // Process regions
        let mut i = 0;
        while i < state.n_positions {
            if !state.positions[i].flipped {
                let start = state.positions[i].angle;
                let end = start.add(uncut_region);
                state.process_interval(start, end);
            }
            i += n_flips;
        }
    }

    let mut min_order = period;
    for i in 0..n_orders {
        if state.orders[i] > 0 && state.orders[i] < min_order {
            min_order = state.orders[i];
        }
    }
    min_order as i32
}

fn f_func(state: &mut OrderState, a: i32, b: i32, c: i32) -> i32 {
    let mut flip_sizes = [Rad::zero(); 3];
    if is_sq(c) {
        let sc = isqrt_i(c);
        unsafe {
            GLOB_SQRT_C = 1.0;
            GLOB_D = a as i64 * b as i64 * sc as i64;
        }
        let d = unsafe { GLOB_D };
        flip_sizes[0] = Rad { a: d / a as i64, b: 0 };
        flip_sizes[1] = Rad { a: d / b as i64, b: 0 };
        flip_sizes[2] = Rad { a: d / sc as i64, b: 0 };
    } else {
        unsafe {
            GLOB_SQRT_C = (c as f64).sqrt();
            GLOB_D = a as i64 * b as i64 * c as i64;
        }
        let d = unsafe { GLOB_D };
        flip_sizes[0] = Rad { a: d / a as i64, b: 0 };
        flip_sizes[1] = Rad { a: d / b as i64, b: 0 };
        flip_sizes[2] = Rad { a: 0, b: d / c as i64 };
    }
    order_compute(state, &flip_sizes, 3)
}

fn main() {
    let nn = 53;
    let mut state = OrderState::new();
    let mut ans: i64 = 0;
    for a in 9..nn {
        for b in (a + 1)..nn {
            for c in (b + 1)..=nn {
                ans += f_func(&mut state, a, b, c) as i64;
            }
        }
    }
    println!("{}", ans);
}
