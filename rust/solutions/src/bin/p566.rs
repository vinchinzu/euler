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
    fn add(self, other: Rad) -> Rad {
        Rad { a: self.a + other.a, b: self.b + other.b }
    }
    fn sub(self, other: Rad) -> Rad {
        Rad { a: self.a - other.a, b: self.b - other.b }
    }
    fn val(self) -> f64 {
        unsafe { self.a as f64 + self.b as f64 * GLOB_SQRT_C }
    }
    fn cmp(self, other: Rad) -> std::cmp::Ordering {
        self.val().partial_cmp(&other.val()).unwrap_or(std::cmp::Ordering::Equal)
    }
    fn min(self, other: Rad) -> Rad {
        if self.cmp(other) != std::cmp::Ordering::Greater { self } else { other }
    }
    fn max(self, other: Rad) -> Rad {
        if self.cmp(other) != std::cmp::Ordering::Less { self } else { other }
    }
    fn one() -> Rad {
        Rad { a: unsafe { GLOB_D }, b: 0 }
    }
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

fn is_sq(n: i32) -> bool {
    let r = (n as f64).sqrt() as i32;
    r * r == n || (r + 1) * (r + 1) == n
}

fn isqrt_i(n: i32) -> i32 {
    let mut r = (n as f64).sqrt() as i32;
    while r * r > n { r -= 1; }
    while (r + 1) * (r + 1) <= n { r += 1; }
    r
}

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

fn lcm_ll(a: i64, b: i64) -> i64 {
    a / gcd_ll(a, b) * b
}

fn mod_ll(a: i64, m: i64) -> i64 {
    ((a % m) + m) % m
}

fn ext_gcd(a: i64, b: i64) -> (i64, i64) {
    if b == 0 {
        return (1, 0);
    }
    let (x1, y1) = ext_gcd(b, a % b);
    (y1, x1 - (a / b) * y1)
}

fn general_crt(a1: i64, m1: i64, a2: i64, m2: i64) -> i64 {
    let g = gcd_ll(m1, m2);
    if (a1 - a2) % g != 0 {
        return -1;
    }
    let (lx, _) = ext_gcd(m1, m2);
    let lcm = m1 / g * m2;
    mod_ll(a1 - m1 * ((lx as i128 * ((a1 - a2) / g) as i128 % lcm as i128) as i64), lcm)
}

fn identical_shifts(vals: &[i32], k: usize) -> Vec<i32> {
    let mut results = Vec::new();
    let h: i64 = 3;
    let len = vals.len();
    let sublen = len / k;
    let mut pow_h: i64 = 1;
    for _ in 0..sublen {
        pow_h *= h;
    }

    let mut target_hash: i64 = 0;
    for i in (0..len).step_by(k) {
        target_hash = target_hash * h + vals[i] as i64;
    }

    for initial_shift in 0..k {
        let mut hash_val: i64 = 0;
        for i in (initial_shift..len).step_by(k) {
            hash_val = hash_val * h + vals[i] as i64;
        }
        for i in (initial_shift..len).step_by(k) {
            if hash_val == target_hash {
                results.push(i as i32);
            }
            hash_val = hash_val * h + (1 - pow_h) * vals[i] as i64;
        }
    }
    results
}

fn order_compute(flip_sizes: &[Rad; 3], n_flips: usize) -> i32 {
    let mut orders: Vec<i64> = vec![0];
    let mut period: i64 = 1;

    let mut intervals: Vec<Interval> = Vec::new();
    let mut current_point = Rad::zero();

    let advance = |current: &mut Rad, intervals: &[Interval]| {
        for iv in intervals {
            if iv.start.cmp(*current) != std::cmp::Ordering::Greater
                && current.cmp(iv.end) == std::cmp::Ordering::Less
            {
                *current = iv.end;
                return;
            }
        }
    };

    let process_interval = |intervals: &mut Vec<Interval>, start: Rad, end: Rad| {
        let mut ms = start;
        let mut me = end;
        let mut new_ints: Vec<Interval> = Vec::new();
        for iv in intervals.iter() {
            if iv.end.cmp(start) == std::cmp::Ordering::Less
                || end.cmp(iv.start) == std::cmp::Ordering::Less
            {
                new_ints.push(*iv);
            } else {
                ms = ms.min(iv.start);
                me = me.max(iv.end);
            }
        }
        new_ints.push(Interval { start: ms, end: me });
        new_ints.sort_by(|a, b| a.start.cmp(b.start));
        *intervals = new_ints;
    };

    let intervals_done = |current: &Rad| -> bool {
        current.cmp(Rad::one()) != std::cmp::Ordering::Less
    };

    loop {
        advance(&mut current_point, &intervals);
        if intervals_done(&current_point) {
            break;
        }

        let start_pos = Pos { angle: current_point, flipped: false };
        let mut positions: Vec<Pos> = Vec::new();
        let mut uncut_region = Rad::one();
        let mut pos = start_pos;
        let mut first = true;

        loop {
            if !first && pos.angle.cmp(start_pos.angle) == std::cmp::Ordering::Equal && pos.flipped == start_pos.flipped {
                break;
            }
            first = false;
            for fi in 0..n_flips {
                if positions.len() >= MAX_POSITIONS {
                    break;
                }
                positions.push(pos);
                let _remaining = Rad::one().sub(pos.angle);
                let size = flip_sizes[fi];
                let cmp_val = pos.angle.cmp(size);
                let threshold = if pos.flipped { std::cmp::Ordering::Less } else { std::cmp::Ordering::Less };
                if cmp_val == std::cmp::Ordering::Less || (cmp_val == std::cmp::Ordering::Equal && pos.flipped) {
                    uncut_region = uncut_region.min(
                        if pos.flipped { pos.angle } else { size.sub(pos.angle) }
                    );
                    pos = Pos {
                        angle: Rad::one().sub(pos.angle),
                        flipped: !pos.flipped,
                    };
                } else {
                    uncut_region = uncut_region.min(
                        if pos.flipped { pos.angle.sub(size) } else { Rad::one().sub(pos.angle) }
                    );
                    pos = Pos {
                        angle: pos.angle.sub(size),
                        flipped: pos.flipped,
                    };
                }
            }
        }

        let flip_arr: Vec<i32> = positions.iter().map(|p| if p.flipped { 1 } else { 0 }).collect();

        let shift_results = identical_shifts(&flip_arr, n_flips);

        let curr_period = positions.len() as i64;
        let mut new_orders: Vec<i64> = Vec::new();
        for &oi in &orders {
            for &si in &shift_results {
                let nv = general_crt(oi, period, si as i64, curr_period);
                if nv != -1 && !new_orders.contains(&nv) && new_orders.len() < MAX_ORDERS {
                    new_orders.push(nv);
                }
            }
        }
        period = lcm_ll(period, curr_period);
        orders = new_orders;

        for i in (0..positions.len()).step_by(n_flips) {
            if !positions[i].flipped {
                process_interval(
                    &mut intervals,
                    positions[i].angle,
                    positions[i].angle.add(uncut_region),
                );
            }
        }
    }

    let mut min_order = period;
    for &o in &orders {
        if o > 0 && o < min_order {
            min_order = o;
        }
    }
    min_order as i32
}

fn f_func(a: i32, b: i32, c: i32) -> i32 {
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
    order_compute(&flip_sizes, 3)
}

fn main() {
    let nn = 53;
    let mut ans: i64 = 0;
    for a in 9..nn {
        for b in (a + 1)..nn {
            for c in (b + 1)..=nn {
                ans += f_func(a, b, c) as i64;
            }
        }
    }
    println!("{}", ans);
}
