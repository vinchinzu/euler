// Project Euler 552 - Chinese Remainder Theorem / Garner's Algorithm
//
// For each prime p_i, check if any partial CRT reconstruction A_n (n < i)
// is divisible by p_i.

use euler_utils::primes_up_to;
use rayon::prelude::*;

#[inline(always)]
fn barrett(x: u64, p: u64, m: u64) -> u64 {
    let q = ((x as u128 * m as u128) >> 64) as u64;
    x - q * p
}

fn power(mut base: u64, mut exp: u64, modulus: u64) -> u64 {
    let mut result: u64 = 1;
    base %= modulus;
    while exp > 0 {
        if exp & 1 == 1 {
            result = result * base % modulus;
        }
        base = base * base % modulus;
        exp >>= 1;
    }
    result
}

struct PrimeState {
    prime: u32,
    m: u64,
    a: u32,
    prod: u32,
    good: bool,
}

#[derive(Clone, Copy)]
#[repr(C)]
struct Item {
    garner: u32,
    prime: u32,
}

fn main() {
    let n = 300_000;
    let primes = primes_up_to(n);
    let l = primes.len();

    let mut states: Vec<PrimeState> = primes
        .iter()
        .map(|&p| {
            let p_u64 = p as u64;
            let m = ((1u128 << 64).div_ceil(p_u64 as u128)) as u64;
            PrimeState {
                prime: p as u32,
                m,
                a: 0,
                prod: 1,
                good: false,
            }
        })
        .collect();

    let mut items: Vec<Item> = primes
        .iter()
        .map(|&p| Item {
            garner: 0,
            prime: p as u32,
        })
        .collect();

    const BLOCK_SIZE: usize = 512;

    let mut k = 0;
    while k < l {
        let block_end = (k + BLOCK_SIZE).min(l);

        // Step 1: Sequentially compute garner[i] for i in k..block_end
        for i in k..block_end {
            let p = states[i].prime as u64;
            let m = states[i].m;
            let mut cur_a = states[i].a as u64;
            let mut cur_prod = states[i].prod as u64;
            let mut cur_good = states[i].good;

            let sub = &items[k..i];
            if cur_good {
                for item in sub {
                    cur_a = barrett(cur_a + cur_prod * item.garner as u64, p, m);
                    cur_prod = barrett(cur_prod * item.prime as u64, p, m);
                }
            } else {
                let mut it = sub.iter();
                for item in it.by_ref() {
                    cur_a = barrett(cur_a + cur_prod * item.garner as u64, p, m);
                    cur_prod = barrett(cur_prod * item.prime as u64, p, m);
                    if cur_a == 0 {
                        cur_good = true;
                        break;
                    }
                }
                if cur_good {
                    for item in it {
                        cur_a = barrett(cur_a + cur_prod * item.garner as u64, p, m);
                        cur_prod = barrett(cur_prod * item.prime as u64, p, m);
                    }
                }
            }

            if cur_prod != 0 {
                let need = (i as u64 + 1 + p - cur_a) % p;
                let inv = power(cur_prod, p - 2, p);
                items[i].garner = (need * inv % p) as u32;
            } else {
                items[i].garner = 0;
            }

            states[i].a = cur_a as u32;
            states[i].prod = cur_prod as u32;
            states[i].good = cur_good;
        }

        // Step 2: Parallel update future primes in [block_end..l] with [k..block_end]
        if block_end < l {
            let items_block = &items[k..block_end];

            states[block_end..].par_iter_mut().with_min_len(64).for_each(|st| {
                let p = st.prime as u64;
                let m = st.m;
                let mut cur_a = st.a as u64;
                let mut cur_prod = st.prod as u64;
                let mut cur_good = st.good;

                if cur_good {
                    for item in items_block {
                        cur_a = barrett(cur_a + cur_prod * item.garner as u64, p, m);
                        cur_prod = barrett(cur_prod * item.prime as u64, p, m);
                    }
                } else {
                    let mut it = items_block.iter();
                    for item in it.by_ref() {
                        cur_a = barrett(cur_a + cur_prod * item.garner as u64, p, m);
                        cur_prod = barrett(cur_prod * item.prime as u64, p, m);
                        if cur_a == 0 {
                            cur_good = true;
                            break;
                        }
                    }
                    if cur_good {
                        for item in it {
                            cur_a = barrett(cur_a + cur_prod * item.garner as u64, p, m);
                            cur_prod = barrett(cur_prod * item.prime as u64, p, m);
                        }
                    }
                }

                st.a = cur_a as u32;
                st.prod = cur_prod as u32;
                st.good = cur_good;
            });
        }

        k = block_end;
    }

    let ans: u64 = states.iter().filter(|st| st.good).map(|st| st.prime as u64).sum();
    println!("{ans}");
}
