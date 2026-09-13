use std::ptr;

const N_RANKS: usize = 13;
const N_SUITS: usize = 4;

static mut MEMO: [[f64; 5]; 2400] = [[0.0; 5]; 2400];
static mut MEMO_VALID: [[bool; 5]; 2400] = [[false; 5]; 2400];
static mut STATE_IDX: [[[[u16; 14]; 14]; 14]; 14] = [[[[0u16; 14]; 14]; 14]; 14];

fn init_states() {
    let mut idx = 0u16;
    unsafe {
        let state_ptr = ptr::addr_of_mut!(STATE_IDX);
        for c4 in 0..=N_RANKS {
            for c3 in 0..=N_RANKS - c4 {
                for c2 in 0..=N_RANKS - c4 - c3 {
                    for c1 in 0..=N_RANKS - c4 - c3 - c2 {
                        (*state_ptr)[c1][c2][c3][c4] = idx;
                        idx += 1;
                    }
                }
            }
        }
    }
}

#[inline(always)]
fn e_val(c0: usize, c1: usize, c2: usize, c3: usize, c4: usize, curr_count: usize) -> f64 {
    let total = c1 + 2 * c2 + 3 * c3 + 4 * c4;
    if total == 0 {
        return 0.0;
    }

    let si = unsafe {
        let state_ptr = ptr::addr_of!(STATE_IDX);
        (*state_ptr)[c1][c2][c3][c4] as usize
    };

    unsafe {
        let valid_ptr = ptr::addr_of!(MEMO_VALID);
        if (*valid_ptr)[si][curr_count] {
            let memo_ptr = ptr::addr_of!(MEMO);
            return (*memo_ptr)[si][curr_count];
        }
    }

    let mut result = 1.0;
    let total_f = total as f64;

    if c1 > 0 {
        let avail = if curr_count == 1 { c1 - 1 } else { c1 };
        if avail > 0 {
            let future = e_val(c0 + 1, c1 - 1, c2, c3, c4, 0);
            result += future * avail as f64 / total_f;
        }
    }

    if c2 > 0 {
        let avail = if curr_count == 2 { c2 - 1 } else { c2 };
        if avail > 0 {
            let future = e_val(c0, c1 + 1, c2 - 1, c3, c4, 1);
            result += future * (2.0 * avail as f64) / total_f;
        }
    }

    if c3 > 0 {
        let avail = if curr_count == 3 { c3 - 1 } else { c3 };
        if avail > 0 {
            let future = e_val(c0, c1, c2 + 1, c3 - 1, c4, 2);
            result += future * (3.0 * avail as f64) / total_f;
        }
    }

    if c4 > 0 {
        let avail = if curr_count == 4 { c4 - 1 } else { c4 };
        if avail > 0 {
            let future = e_val(c0, c1, c2, c3 + 1, c4 - 1, 3);
            result += future * (4.0 * avail as f64) / total_f;
        }
    }

    unsafe {
        let memo_ptr = ptr::addr_of_mut!(MEMO);
        let valid_ptr = ptr::addr_of_mut!(MEMO_VALID);
        (*memo_ptr)[si][curr_count] = result;
        (*valid_ptr)[si][curr_count] = true;
    }
    result
}

fn main() {
    init_states();
    let answer = e_val(0, 0, 0, 0, 13, 0);
    println!("{:.8}", answer);
}
