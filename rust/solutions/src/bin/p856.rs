// Project Euler 856 - Expected cards drawn until consecutive pair
// Memoized DP over rank-count states

const N_RANKS: usize = 13;
const N_SUITS: usize = 4;

static mut MEMO: [[f64; 5]; 2400] = [[0.0; 5]; 2400];
static mut MEMO_VALID: [[bool; 5]; 2400] = [[false; 5]; 2400];
static mut STATE_IDX: [[[[i32; 14]; 14]; 14]; 14] = [[[[-1i32; 14]; 14]; 14]; 14];
static mut NUM_STATES: i32 = 0;

fn init_states() {
    unsafe {
        for c4 in 0..=N_RANKS {
            for c3 in 0..=N_RANKS - c4 {
                for c2 in 0..=N_RANKS - c4 - c3 {
                    for c1 in 0..=N_RANKS - c4 - c3 - c2 {
                        STATE_IDX[c1][c2][c3][c4] = NUM_STATES;
                        NUM_STATES += 1;
                    }
                }
            }
        }
    }
}

fn e_val(c0: usize, c1: usize, c2: usize, c3: usize, c4: usize, curr_count: usize) -> f64 {
    let total = c1 + 2 * c2 + 3 * c3 + 4 * c4;
    if total == 0 { return 0.0; }

    let si = unsafe { STATE_IDX[c1][c2][c3][c4] } as usize;
    unsafe {
        if MEMO_VALID[si][curr_count] { return MEMO[si][curr_count]; }
    }

    let counts = [c0, c1, c2, c3, c4];
    let mut result = 1.0;

    for k in 1..=N_SUITS {
        let mut available = counts[k] as i32;
        if curr_count == k { available -= 1; }
        if available <= 0 { continue; }

        let mut nc = counts;
        nc[k - 1] += 1;
        nc[k] -= 1;

        let future = e_val(nc[0], nc[1], nc[2], nc[3], nc[4], k - 1);
        result += future * k as f64 * available as f64 / total as f64;
    }

    unsafe {
        MEMO[si][curr_count] = result;
        MEMO_VALID[si][curr_count] = true;
    }
    result
}

fn main() {
    init_states();
    let answer = e_val(0, 0, 0, 0, 13, 0);
    println!("{:.8}", answer);
}
