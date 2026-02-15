// Project Euler 847 - Digit DP with bit-level processing
// Hash map DP for (R, carries, c_states) -> count mod MOD

use std::collections::HashMap;

const MOD: i64 = 1_000_000_007;

fn pack_state(r_off: i32, carries: i32, c0: i32, c1: i32, c2: i32) -> u64 {
    let mut k = r_off as u64;
    k = k * 8 + carries as u64;
    k = k * 256 + c0 as u64;
    k = k * 256 + c1 as u64;
    k = k * 256 + c2 as u64;
    k
}

fn unpack_state(mut k: u64) -> (i32, i32, i32, i32, i32) {
    let c2 = (k & 255) as i32; k >>= 8;
    let c1 = (k & 255) as i32; k >>= 8;
    let c0 = (k & 255) as i32; k >>= 8;
    let carries = (k & 7) as i32; k >>= 3;
    let r_off = k as i32;
    (r_off, carries, c0, c1, c2)
}

fn solve_dp(limit: i64, kval: i32, checks: &[[i32; 3]], nchk: usize) -> i64 {
    let mut l_bits = [0i32; 100];
    let mut actual_bits = 0;
    {
        let mut tmp = limit;
        while tmp > 0 {
            l_bits[actual_bits] = (tmp & 1) as i32;
            actual_bits += 1;
            tmp >>= 1;
        }
    }
    let nb = actual_bits.max(kval as usize);

    let mut cur: HashMap<u64, i64> = HashMap::new();
    let mut nxt: HashMap<u64, i64> = HashMap::new();

    let init_cs = 1i32;
    let c0_init = if nchk > 0 { init_cs } else { 0 };
    let c1_init = if nchk > 1 { init_cs } else { 0 };
    let c2_init = if nchk > 2 { init_cs } else { 0 };

    let init_key = pack_state(3, 0, c0_init, c1_init, c2_init);
    cur.insert(init_key, 1);

    for j in (0..nb).rev() {
        nxt.clear();
        let limit_bit = if j < actual_bits { l_bits[j] } else { 0 };
        let has_source = j < kval as usize;

        for (&skey, &sval) in &cur {
            if sval == 0 { continue; }
            let (r_off, carries_packed, cs0, cs1, cs2) = unpack_state(skey);
            let r = r_off - 3;
            let c_a = (carries_packed >> 2) & 1;
            let c_b = (carries_packed >> 1) & 1;
            let c_c = carries_packed & 1;

            for a in 0..2i32 {
                for b in 0..2i32 {
                    for c in 0..2i32 {
                        let mut new_r = 2 * r + (a + b + c) - limit_bit;
                        if new_r >= 2 { continue; }
                        if new_r <= -3 { new_r = -3; }

                        let valid_nca: Vec<i32> = if c_a != 0 {
                            if a == 1 { vec![1] } else { continue; }
                        } else {
                            if a == 0 { vec![0, 1] } else { vec![0] }
                        };

                        let valid_ncb: Vec<i32> = if c_b != 0 {
                            if b == 1 { vec![1] } else { continue; }
                        } else {
                            if b == 0 { vec![0, 1] } else { vec![0] }
                        };

                        let valid_ncc: Vec<i32> = if c_c != 0 {
                            if c == 1 { vec![1] } else { continue; }
                        } else {
                            if c == 0 { vec![0, 1] } else { vec![0] }
                        };

                        for &nca in &valid_nca {
                            let bit_a = a;
                            let bit_a1 = a + nca - 2 * c_a;
                            for &ncb in &valid_ncb {
                                let bit_b = b;
                                let bit_b1 = b + ncb - 2 * c_b;
                                for &ncc in &valid_ncc {
                                    let bit_c = c;
                                    let bit_c1 = c + ncc - 2 * c_c;

                                    let mut possible = true;
                                    let mut new_cs = [0i32; 3];

                                    for idx in 0..nchk {
                                        let b_a = if checks[idx][0] != 0 { bit_a1 } else { bit_a };
                                        let b_b = if checks[idx][1] != 0 { bit_b1 } else { bit_b };
                                        let b_c = if checks[idx][2] != 0 { bit_c1 } else { bit_c };

                                        let prev_states = match idx {
                                            0 => cs0, 1 => cs1, _ => cs2,
                                        };

                                        let mut current_possible = 0i32;

                                        for ps in 0..8 {
                                            if prev_states & (1 << ps) == 0 { continue; }
                                            let ps_a = (ps >> 2) & 1;
                                            let ps_b = (ps >> 1) & 1;
                                            let ps_c = ps & 1;

                                            let max_owner = if has_source { 3 } else { 1 };
                                            for oi in 0..max_owner {
                                                let owner: i32 = if has_source { oi } else { -1 };
                                                let mut valid_owner = true;
                                                let mut ns_a = ps_a;
                                                let mut ns_b = ps_b;
                                                let mut ns_c = ps_c;

                                                if owner == 0 {
                                                    if ps_a == 0 && b_a == 0 { ns_a = 1; }
                                                } else if ps_a == 0 && b_a == 1 {
                                                    valid_owner = false;
                                                }
                                                if !valid_owner { continue; }

                                                if owner == 1 {
                                                    if ps_b == 0 && b_b == 0 { ns_b = 1; }
                                                } else if ps_b == 0 && b_b == 1 {
                                                    valid_owner = false;
                                                }
                                                if !valid_owner { continue; }

                                                if owner == 2 {
                                                    if ps_c == 0 && b_c == 0 { ns_c = 1; }
                                                } else if ps_c == 0 && b_c == 1 {
                                                    valid_owner = false;
                                                }
                                                if !valid_owner { continue; }

                                                let ns = (ns_a << 2) | (ns_b << 1) | ns_c;
                                                current_possible |= 1 << ns;
                                            }
                                        }

                                        if current_possible == 0 { possible = false; break; }
                                        new_cs[idx] = current_possible;
                                    }

                                    if possible {
                                        let new_r_off = new_r + 3;
                                        let new_carries = (nca << 2) | (ncb << 1) | ncc;
                                        let nc0 = if nchk > 0 { new_cs[0] } else { 0 };
                                        let nc1 = if nchk > 1 { new_cs[1] } else { 0 };
                                        let nc2 = if nchk > 2 { new_cs[2] } else { 0 };
                                        let nkey = pack_state(new_r_off, new_carries, nc0, nc1, nc2);
                                        let e = nxt.entry(nkey).or_insert(0);
                                        *e = (*e + sval) % MOD;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        std::mem::swap(&mut cur, &mut nxt);
    }

    let mut result = 0i64;
    for (&key, &val) in &cur {
        let (r_off, carries_packed, _, _, _) = unpack_state(key);
        let r = r_off - 3;
        let c_a = (carries_packed >> 2) & 1;
        let c_b = (carries_packed >> 1) & 1;
        let c_c = carries_packed & 1;
        if c_a != 0 && c_b != 0 && c_c != 0 && r <= 0 {
            result = (result + val) % MOD;
        }
    }
    result
}

fn mod_pow(mut base: i64, mut exp: i64, m: i64) -> i64 {
    let mut r = 1i64;
    base %= m;
    while exp > 0 {
        if exp & 1 == 1 { r = r * base % m; }
        base = base * base % m;
        exp >>= 1;
    }
    r
}

fn c_val_mod(n: i64) -> i64 {
    if n < 0 { return 0; }
    let a = (n + 3) % MOD;
    let b = (n + 2) % MOD;
    let c = (n + 1) % MOD;
    let inv6 = mod_pow(6, MOD - 2, MOD);
    let val = a * b % MOD * c % MOD * inv6 % MOD;
    (val - 1 + MOD) % MOD
}

fn main() {
    let mut n: i64 = 0;
    let mut p: i64 = 1;
    for _ in 0..19 {
        n += p;
        p *= 10;
    }

    let c_n_mod = c_val_mod(n);

    let mut total_h: i64 = 0;
    let mut k = 0i32;

    loop {
        let chk1 = [[0, 0, 0]];
        let t1 = solve_dp(n - 1, k, &chk1, 1);

        let chk2 = [[0, 1, 0], [1, 0, 0]];
        let t2 = solve_dp(n - 2, k, &chk2, 2);

        let chk3 = [[0, 1, 1], [1, 0, 1], [1, 1, 0]];
        let t3 = solve_dp(n - 3, k, &chk3, 3);

        let size_sk = ((3 * t1 % MOD - 3 * t2 % MOD + t3 % MOD) % MOD + MOD) % MOD;
        let term = (c_n_mod - size_sk % MOD + MOD) % MOD;

        if term == 0 { break; }

        total_h = (total_h + term) % MOD;
        k += 1;
        if k > 100 { break; }
    }

    println!("{}", total_h);
}
