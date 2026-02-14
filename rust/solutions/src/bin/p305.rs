// Project Euler 305: Reflexive Position
use std::fmt::Write as FmtWrite;

static mut POWER10: [i64; 20] = [0; 20];

static mut PAT: [i32; 20] = [0; 20];
static mut PAT_LEN: usize = 0;
static mut KMP_FAIL: [i32; 20] = [0; 20];
static mut KMP_TRANS: [[i32; 10]; 21] = [[0; 10]; 21];

unsafe fn build_kmp_table() {
    KMP_FAIL[0] = 0;
    for i in 1..PAT_LEN {
        let mut j = KMP_FAIL[i - 1] as usize;
        while j > 0 && PAT[j] != PAT[i] {
            j = KMP_FAIL[j - 1] as usize;
        }
        if PAT[j] == PAT[i] { j += 1; }
        KMP_FAIL[i] = j as i32;
    }
    for state in 0..PAT_LEN {
        for c in 0..10 {
            let mut j = state;
            while j > 0 && PAT[j] != c as i32 {
                j = KMP_FAIL[j - 1] as usize;
            }
            if PAT[j] == c as i32 { j += 1; }
            KMP_TRANS[state][c] = j as i32;
        }
    }
    let fallback = KMP_FAIL[PAT_LEN - 1] as usize;
    for c in 0..10 {
        let mut j = fallback;
        while j > 0 && PAT[j] != c as i32 {
            j = KMP_FAIL[j - 1] as usize;
        }
        if PAT[j] == c as i32 { j += 1; }
        KMP_TRANS[PAT_LEN][c] = j as i32;
    }
}

#[derive(Clone, Copy)]
struct DpPair {
    cnt: i64,
    matches: i64,
}

static mut DP2_MEMO: [[[DpPair; 20]; 2]; 20] = [[[DpPair { cnt: 0, matches: 0 }; 20]; 2]; 20];
static mut DP2_VALID: [[[i32; 20]; 2]; 20] = [[[0; 20]; 2]; 20];
static mut DP2_EPOCH: i32 = 0;
static mut UPPER_DIGITS: [i32; 20] = [0; 20];
static mut NUM_DIGITS: usize = 0;

unsafe fn digit_dp2(pos: usize, tight: usize, kmp_state: usize) -> DpPair {
    if pos == NUM_DIGITS {
        return DpPair { cnt: 1, matches: 0 };
    }
    if DP2_VALID[pos][tight][kmp_state] == DP2_EPOCH {
        return DP2_MEMO[pos][tight][kmp_state];
    }
    let limit = if tight != 0 { UPPER_DIGITS[pos] } else { 9 };
    let start = if pos == 0 { 1 } else { 0 };
    let mut result = DpPair { cnt: 0, matches: 0 };
    for d in start..=limit {
        let new_tight = if tight != 0 && d == limit { 1 } else { 0 };
        let new_kmp = KMP_TRANS[kmp_state][d as usize] as usize;
        let match_here = if new_kmp == PAT_LEN { 1i64 } else { 0 };
        let sub = digit_dp2(pos + 1, new_tight, new_kmp);
        result.cnt += sub.cnt;
        result.matches += sub.matches + match_here * sub.cnt;
    }
    DP2_VALID[pos][tight][kmp_state] = DP2_EPOCH;
    DP2_MEMO[pos][tight][kmp_state] = result;
    result
}

unsafe fn count_nonspanning_d_digits(d: usize) -> i64 {
    if d < PAT_LEN { return 0; }
    NUM_DIGITS = d;
    for i in 0..d { UPPER_DIGITS[i] = 9; }
    DP2_EPOCH += 1;
    digit_dp2(0, 0, 0).matches
}

unsafe fn count_nonspanning_up_to_m(d: usize, m: i64) -> i64 {
    if d < PAT_LEN { return 0; }
    NUM_DIGITS = d;
    let mut buf = String::new();
    write!(buf, "{}", m).unwrap();
    for (i, ch) in buf.chars().enumerate() {
        UPPER_DIGITS[i] = (ch as u8 - b'0') as i32;
    }
    DP2_EPOCH += 1;
    digit_dp2(0, 1, 0).matches
}

unsafe fn count_spanning_for_split(a: usize, b: usize, m_val: i64) -> i64 {
    if m_val < 2 { return 0; }
    let mut p_val: i64 = 0;
    for i in 0..a { p_val = p_val * 10 + PAT[i] as i64; }
    let r_val = (p_val + 1) % POWER10[a];
    let carry = if p_val + 1 >= POWER10[a] { 1 } else { 0 };
    let mut q_val: i64 = 0;
    for i in a..PAT_LEN { q_val = q_val * 10 + PAT[i] as i64; }

    let mut total: i64 = 0;
    let mut mbuf = String::new();
    write!(mbuf, "{}", m_val).unwrap();
    let max_d2 = mbuf.len();

    for d2 in 1..=max_d2 {
        if d2 < b { continue; }
        let x_lo = if d2 == 1 { 2i64 } else { POWER10[d2 - 1].max(2) };
        let x_hi = if d2 < max_d2 { POWER10[d2] - 1 } else { m_val };
        if x_lo > x_hi { continue; }

        if d2 < a + b {
            let overlap_start = d2 - a;
            let qbuf = format!("{:0>width$}", q_val, width = b);
            let rbuf = format!("{:0>width$}", r_val, width = a);
            if carry != 0 { continue; }
            let qb = qbuf.as_bytes();
            let rb = rbuf.as_bytes();
            let mut consistent = true;
            for i in overlap_start..b {
                if qb[i] != rb[i - overlap_start] { consistent = false; break; }
            }
            if !consistent { continue; }
            let mut xbuf = vec![0u8; d2];
            for i in 0..b { xbuf[i] = qb[i]; }
            for j in 0..a {
                let pos = d2 - a + j;
                if pos >= b { xbuf[pos] = rb[j]; }
            }
            let mut x: i64 = 0;
            for i in 0..d2 { x = x * 10 + (xbuf[i] - b'0') as i64; }
            if x >= x_lo && x <= x_hi {
                let n_val = x - 1;
                if n_val >= 1 && n_val % POWER10[a] == p_val { total += 1; }
            }
            continue;
        }

        let mid_len = d2 - a - b;
        if carry == 0 {
            if b == 1 && q_val == 0 { continue; }
            let base = q_val * POWER10[mid_len + a] + r_val;
            let mut mid_lo = 0i64;
            let mut mid_hi = POWER10[mid_len] - 1;
            if base + mid_lo * POWER10[a] < x_lo {
                mid_lo = (x_lo - base + POWER10[a] - 1) / POWER10[a];
            }
            if base + mid_hi * POWER10[a] > x_hi {
                mid_hi = (x_hi - base) / POWER10[a];
            }
            if mid_lo <= mid_hi { total += mid_hi - mid_lo + 1; }
        } else {
            if b == 1 && q_val == 0 { continue; }
            let base = q_val * POWER10[mid_len + a];
            let mut mid_lo = 0i64;
            let mut mid_hi = POWER10[mid_len] - 1;
            if base + mid_lo * POWER10[a] < x_lo {
                mid_lo = (x_lo - base + POWER10[a] - 1) / POWER10[a];
            }
            if base + mid_hi * POWER10[a] > x_hi {
                mid_hi = (x_hi - base) / POWER10[a];
            }
            if mid_lo <= mid_hi { total += mid_hi - mid_lo + 1; }
        }
    }
    total
}

unsafe fn count_occurrences(m: i64) -> i64 {
    if m < 1 { return 0; }
    let mut buf = String::new();
    write!(buf, "{}", m).unwrap();
    let d_m = buf.len();
    let mut total: i64 = 0;
    for d in PAT_LEN..d_m {
        total += count_nonspanning_d_digits(d);
    }
    if d_m >= PAT_LEN {
        total += count_nonspanning_up_to_m(d_m, m);
    }
    for a in 1..PAT_LEN {
        total += count_spanning_for_split(a, PAT_LEN - a, m);
    }
    total
}

fn count_digits_up_to(n: i64) -> i64 {
    if n < 1 { return 0; }
    let mut total = 0i64;
    let mut d = 1i64;
    let mut first = 1i64;
    while first <= n {
        let last = (first * 10 - 1).min(n);
        total += (last - first + 1) * d;
        d += 1;
        first *= 10;
    }
    total
}

unsafe fn find_nth_occurrence(target: i64, n: i64) -> i64 {
    let buf = format!("{}", target);
    PAT_LEN = buf.len();
    for (i, ch) in buf.chars().enumerate() {
        PAT[i] = (ch as u8 - b'0') as i32;
    }
    build_kmp_table();

    let mut lo = 1i64;
    let mut hi = 10_000_000_000_000i64;
    while lo < hi {
        let mid = lo + (hi - lo) / 2;
        if count_occurrences(mid) >= n { hi = mid; }
        else { lo = mid + 1; }
    }

    let prev_count = count_occurrences(lo - 1);
    let mut remaining = n - prev_count;

    let mut window = String::new();
    let nums = [lo - 1, lo, lo + 1];
    let mut starts = [0usize; 3];
    for i in 0..3 {
        starts[i] = window.len();
        if nums[i] >= 1 {
            write!(window, "{}", nums[i]).unwrap();
        }
    }

    let pos_base = count_digits_up_to(lo - 2) + 1;
    let wb = window.as_bytes();
    for i in 0..=wb.len().saturating_sub(PAT_LEN) {
        let mut matched = true;
        for j in 0..PAT_LEN {
            if (wb[i + j] - b'0') as i32 != PAT[j] { matched = false; break; }
        }
        if matched {
            if i + PAT_LEN <= starts[1] { continue; }
            remaining -= 1;
            if remaining == 0 { return pos_base + i as i64; }
        }
    }
    -1
}

fn main() {
    unsafe {
        POWER10[0] = 1;
        for i in 1..20 { POWER10[i] = POWER10[i - 1] * 10; }

        let mut total = 0i64;
        let mut p3 = 1i64;
        for _k in 1..=13 {
            p3 *= 3;
            total += find_nth_occurrence(p3, p3);
        }
        println!("{}", total);
    }
}
