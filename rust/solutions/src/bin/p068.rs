// Project Euler 68: Magic 5-gon ring
// Find the maximum 16-digit string for a magic 5-gon ring using digits 1-10.

fn append_num(acc: u64, x: u8) -> u64 {
    if x == 10 {
        acc * 100 + 10
    } else {
        acc * 10 + x as u64
    }
}

fn consider(inner: &[u8; 5], used: &[bool; 11], best: &mut u64) {
    let mut lo = 0u32;
    let mut hi = 100u32;
    for i in 0..5 {
        let pair = inner[i] as u32 + inner[(i + 1) % 5] as u32;
        lo = lo.max(pair + 1);
        hi = hi.min(pair + 10);
    }
    if lo > hi {
        return;
    }

    for s in lo..=hi {
        let mut outer = [0u8; 5];
        let mut taken = *used;
        let mut has10 = false;
        let mut ok = true;
        for i in 0..5 {
            let o = s - inner[i] as u32 - inner[(i + 1) % 5] as u32;
            if !(1..=10).contains(&o) || taken[o as usize] {
                ok = false;
                break;
            }
            taken[o as usize] = true;
            outer[i] = o as u8;
            if o == 10 {
                has10 = true;
            }
        }
        if !ok || !has10 {
            continue;
        }

        let start = (0..5).min_by_key(|&i| outer[i]).unwrap();
        let mut cand = 0u64;
        for k in 0..5 {
            let i = (start + k) % 5;
            cand = append_num(cand, outer[i]);
            cand = append_num(cand, inner[i]);
            cand = append_num(cand, inner[(i + 1) % 5]);
        }
        // 16-digit strings only (10 is an outer node)
        if cand >= 1_000_000_000_000_000 && cand > *best {
            *best = cand;
        }
    }
}

fn rec(depth: usize, inner: &mut [u8; 5], used: &mut [bool; 11], best: &mut u64) {
    if depth == 5 {
        consider(inner, used, best);
        return;
    }
    for v in 1..=10u8 {
        if !used[v as usize] {
            used[v as usize] = true;
            inner[depth] = v;
            rec(depth + 1, inner, used, best);
            used[v as usize] = false;
        }
    }
}

fn main() {
    let mut best = 0u64;
    let mut inner = [0u8; 5];
    let mut used = [false; 11];
    rec(0, &mut inner, &mut used, &mut best);
    println!("{best}");
}
