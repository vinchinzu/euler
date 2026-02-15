// Project Euler 736 - Paths to Equality
//
// Iterative deepening DFS with pruning.
// r(x,y) = (x+1, 2y), s(x,y) = (2x, y+1)

static mut RESULT: i64 = 0;

fn feasible(x: i64, y: i64, depth: i32, t: i32) -> bool {
    let mut pw_t: i128 = 1;
    for _ in 0..t {
        pw_t <<= 1;
    }
    let mut pw_dt: i128 = 1;
    for _ in 0..(depth - t) {
        pw_dt <<= 1;
    }

    let lhs1 = x as i128 * pw_t + (depth - t) as i128;
    let rhs1 = (y as i128 + t as i128) * pw_dt;
    if lhs1 > rhs1 {
        return false;
    }

    let lhs2 = y as i128 * pw_dt + t as i128;
    let rhs2 = (x as i128 + (depth - t) as i128) * pw_t;
    if lhs2 > rhs2 {
        return false;
    }

    true
}

fn search(x: i64, y: i64, depth: i32) {
    unsafe {
        if RESULT > 0 {
            return;
        }
    }
    if depth == 0 {
        if x == y {
            unsafe {
                RESULT = x;
            }
        }
        return;
    }

    let mut any_feasible = false;
    for t in 0..=depth {
        if feasible(x, y, depth, t) {
            any_feasible = true;
            break;
        }
    }
    if !any_feasible {
        return;
    }

    // s-operation: (2x, y+1)
    search(2 * x, y + 1, depth - 1);
    unsafe {
        if RESULT > 0 {
            return;
        }
    }
    // r-operation: (x+1, 2y)
    search(x + 1, 2 * y, depth - 1);
}

fn main() {
    let a: i64 = 45;
    let b: i64 = 90;

    let mut max_depth = 2;
    loop {
        search(a, b, max_depth);
        unsafe {
            if RESULT > 0 {
                break;
            }
        }
        max_depth += 2;
    }

    println!("{}", unsafe { RESULT });
}
