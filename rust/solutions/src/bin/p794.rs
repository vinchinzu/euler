// Project Euler 794 - Seventeen Points
// DFS with interval tracking using rational arithmetic (denominator = LCM(1..17)).

const MAXN: usize = 18;
const LCM: i64 = 12_252_240;

struct State {
    lo: [i64; MAXN],
    hi: [i64; MAXN],
}

static mut BEST_SUM: f64 = 1e18;

fn try_assignment(
    step: usize,
    st: &State,
    ambig: &[usize],
    cell_of: &mut [i32; MAXN],
    ai: usize,
    cell_used: &mut [bool; MAXN],
    n_target: usize,
) {
    if ai == ambig.len() {
        // Find free cell
        let mut free_cell = -1i32;
        for c in 0..step {
            if !cell_used[c] { free_cell = c as i32; break; }
        }
        if free_cell < 0 { return; }

        let mut ns = State { lo: [0; MAXN], hi: [0; MAXN] };
        let unit = LCM / step as i64;

        for p in 0..step - 1 {
            let c = cell_of[p] as i64;
            let cl = c * unit;
            let ch = (c + 1) * unit;
            ns.lo[p] = st.lo[p].max(cl);
            ns.hi[p] = st.hi[p].min(ch);
            if ns.lo[p] >= ns.hi[p] { return; }
        }

        ns.lo[step - 1] = free_cell as i64 * unit;
        ns.hi[step - 1] = (free_cell as i64 + 1) * unit;

        let mut cur_lo_sum = 0.0f64;
        for i in 0..step {
            cur_lo_sum += ns.lo[i] as f64;
        }
        unsafe {
            if cur_lo_sum / LCM as f64 >= BEST_SUM { return; }
        }

        if step == n_target {
            let s = cur_lo_sum / LCM as f64;
            unsafe {
                if s < BEST_SUM { BEST_SUM = s; }
            }
            return;
        }

        solve(step + 1, &ns, n_target);
        return;
    }

    let p = ambig[ai];
    let unit = LCM / step as i64;

    for opt in 0..2 {
        let c = if opt == 0 {
            (st.lo[p] / unit) as i32
        } else {
            ((st.hi[p] - 1) / unit) as i32
        };
        if c < 0 || c >= step as i32 { continue; }
        if cell_used[c as usize] { continue; }

        let cl = c as i64 * unit;
        let ch = (c as i64 + 1) * unit;
        let ol = st.lo[p].max(cl);
        let oh = st.hi[p].min(ch);
        if ol >= oh { continue; }

        cell_of[p] = c;
        cell_used[c as usize] = true;

        try_assignment(step, st, ambig, cell_of, ai + 1, cell_used, n_target);

        cell_used[c as usize] = false;
    }
}

fn solve(step: usize, st: &State, n_target: usize) {
    let unit = LCM / step as i64;

    let mut cell_of = [-1i32; MAXN];
    let mut cell_used = [false; MAXN];
    let mut ambig = Vec::new();

    for p in 0..step - 1 {
        let c_lo = (st.lo[p] / unit) as i32;
        let mut c_hi = ((st.hi[p] - 1) / unit) as i32;
        if c_lo < 0 { continue; }
        if c_hi >= step as i32 { c_hi = step as i32 - 1; }

        if c_lo == c_hi {
            cell_of[p] = c_lo;
            if cell_used[c_lo as usize] { return; }
            cell_used[c_lo as usize] = true;
        } else {
            ambig.push(p);
        }
    }

    try_assignment(step, st, &ambig, &mut cell_of, 0, &mut cell_used, n_target);
}

fn main() {
    let n_target = 17;

    let mut s0 = State { lo: [0; MAXN], hi: [0; MAXN] };
    s0.lo[0] = 0;
    s0.hi[0] = LCM;

    solve(2, &s0, n_target);

    unsafe {
        println!("{:.12}", *std::ptr::addr_of!(BEST_SUM));
    }
}
