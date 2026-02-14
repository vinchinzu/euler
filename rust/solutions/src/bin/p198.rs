// Project Euler 198: Ambiguous Numbers
const LIMIT_Q: i64 = 100_000_000;
const HALF_Q: i64 = LIMIT_Q / 2;
const MAX_STACK: usize = 10_000_000;

fn count_between(hl0: i64, kl0: i64, hr0: i64, kr0: i64) -> i64 {
    let mut stack_hl = vec![0i64; MAX_STACK];
    let mut stack_kl = vec![0i64; MAX_STACK];
    let mut stack_hr = vec![0i64; MAX_STACK];
    let mut stack_kr = vec![0i64; MAX_STACK];
    let mut sp = 0usize;

    stack_hl[sp] = hl0; stack_kl[sp] = kl0;
    stack_hr[sp] = hr0; stack_kr[sp] = kr0;
    sp += 1;

    let mut total = 0i64;

    while sp > 0 {
        sp -= 1;
        let mut hl = stack_hl[sp];
        let mut kl = stack_kl[sp];
        let hr = stack_hr[sp];
        let kr = stack_kr[sp];
        let initial_branch = hl == 0 && kl == 1;
        let mut hr_local = hr;
        let mut kr_local = kr;

        loop {
            let hm = hl + hr_local;
            let km = kl + kr_local;

            if km > HALF_Q { break; }
            if 100 * hm >= km { break; }

            let max_partner = HALF_Q / km;

            if !(hl == 0 && kl == 1) {
                if kl <= max_partner { total += 1; }
            }

            if kr_local <= max_partner { total += 1; }

            let right_blocked = kr_local > max_partner;
            let left_blocked = kl > max_partner;

            if !right_blocked {
                if sp < MAX_STACK {
                    stack_hl[sp] = hm; stack_kl[sp] = km;
                    stack_hr[sp] = hr_local; stack_kr[sp] = kr_local;
                    sp += 1;
                }
            }

            if right_blocked && initial_branch { break; }
            if left_blocked { break; }

            hr_local = hm;
            kr_local = km;
        }
    }

    total
}

fn main() {
    let case1 = HALF_Q - 50;
    let case2 = count_between(0, 1, 1, 100);
    println!("{}", case1 + case2);
}
