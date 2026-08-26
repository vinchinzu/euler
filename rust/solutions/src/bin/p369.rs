// Badugi counting: DP over reachable matchings of the 4-suit side.
// Each rank chooses a 4-bit suit subset. `reach` is a 16-bit bitset of
// currently matchable suit-masks; only 68 such bitsets arise.

const NRANKS: usize = 13;
const MAX_N: usize = 13;
const MAX_STATES: usize = 80;
const FULL_BIT: u16 = 1 << 15;
const POP: [usize; 16] = [0, 1, 1, 2, 1, 2, 2, 3, 1, 2, 2, 3, 2, 3, 3, 4];

fn update_reachable(reach: u16, suits: u32) -> u16 {
    let mut new_reach = reach as u32;
    let mut r = reach;
    while r != 0 {
        let m = r.trailing_zeros();
        r &= r - 1;
        let mut b = suits & !m & 0xF;
        while b != 0 {
            let s = b.trailing_zeros();
            b &= b - 1;
            new_reach |= 1 << (m | (1 << s));
        }
    }
    new_reach as u16
}

fn find_state(states: &[u16], n: usize, s: u16) -> Option<usize> {
    states[..n].iter().position(|&x| x == s)
}

fn main() {
    let mut states = [0u16; MAX_STATES];
    let mut nstates = 1usize;
    states[0] = 1; // only the empty suit-mask is matchable

    let mut q = 0usize;
    while q < nstates {
        let reach = states[q];
        for suits in 0..16u32 {
            let nxt = update_reachable(reach, suits);
            if find_state(&states, nstates, nxt).is_none() {
                states[nstates] = nxt;
                nstates += 1;
            }
        }
        q += 1;
    }

    let mut trans = [[0u8; 16]; MAX_STATES];
    for i in 0..nstates {
        for suits in 0..16u32 {
            let nxt = update_reachable(states[i], suits);
            trans[i][suits as usize] = find_state(&states, nstates, nxt).unwrap() as u8;
        }
    }

    let mut dp = [[0i64; MAX_N + 1]; MAX_STATES];
    let mut ndp = [[0i64; MAX_N + 1]; MAX_STATES];
    dp[0][0] = 1;

    for _ in 0..NRANKS {
        for row in ndp.iter_mut().take(nstates) {
            *row = [0; MAX_N + 1];
        }
        for i in 0..nstates {
            let arr = dp[i];
            if arr.iter().all(|&x| x == 0) {
                continue;
            }
            for suits in 0..16 {
                let j = trans[i][suits] as usize;
                let add = POP[suits];
                let targ = &mut ndp[j];
                for k in 0..=MAX_N - add {
                    let v = arr[k];
                    if v != 0 {
                        targ[k + add] += v;
                    }
                }
            }
        }
        std::mem::swap(&mut dp, &mut ndp);
    }

    let mut total = 0i64;
    for i in 0..nstates {
        if states[i] & FULL_BIT != 0 {
            total += dp[i][4..=MAX_N].iter().sum::<i64>();
        }
    }
    println!("{}", total);
}
