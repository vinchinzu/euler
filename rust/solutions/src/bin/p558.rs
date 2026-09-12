// Project Euler 558 - Irrational Base
//
// Compact 192-bit limbs instead of BigUint. After taking a[i] the remainder
// is < a[i-2], so the greedy scan jumps to i-3 (exponents differ by ≥ 3).

use rayon::prelude::*;

const N_VAL: u64 = 5_000_000;
const L_VAL: usize = 200;
const TOTAL: usize = 2 * L_VAL;

#[derive(Clone, Copy)]
struct U192 {
    lo: u128,
    hi: u64,
}

impl U192 {
    #[inline(always)]
    fn from_u64(x: u64) -> Self {
        Self { lo: x as u128, hi: 0 }
    }

    #[inline(always)]
    fn overflowing_add(self, other: Self) -> (Self, bool) {
        let (lo, c1) = self.lo.overflowing_add(other.lo);
        let (hi1, c2) = self.hi.overflowing_add(other.hi);
        let (hi, c3) = hi1.overflowing_add(c1 as u64);
        (Self { lo, hi }, c2 || c3)
    }

    #[inline(always)]
    fn ge(self, other: Self) -> bool {
        if self.hi != other.hi {
            self.hi > other.hi
        } else {
            self.lo >= other.lo
        }
    }

    #[inline(always)]
    fn sub(self, other: Self) -> Self {
        let (lo, b1) = self.lo.overflowing_sub(other.lo);
        let hi = self.hi - other.hi - b1 as u64;
        Self { lo, hi }
    }

    #[inline(always)]
    fn mul_u64(self, m: u64) -> Self {
        let m128 = m as u128;
        let a0 = (self.lo as u64) as u128;
        let a1 = self.lo >> 64;
        let a2 = self.hi as u128;
        let p0 = a0 * m128;
        let p1 = a1 * m128;
        let p2 = a2 * m128;
        let t1 = p1 + (p0 >> 64);
        let t2 = p2 + (t1 >> 64);
        Self {
            lo: (p0 as u64 as u128) | ((t1 as u64 as u128) << 64),
            hi: t2 as u64,
        }
    }
}

fn main() {
    let mut a = [U192::from_u64(0); TOTAL];
    let mut alen = TOTAL;
    for i in 0..TOTAL {
        if i < 3 {
            a[i] = U192::from_u64((i + 1) as u64);
        } else {
            let (s, ov) = a[i - 1].overflowing_add(a[i - 3]);
            if ov {
                alen = i;
                break;
            }
            a[i] = s;
        }
    }

    let a_l = a[L_VAL];
    let max_t = a_l.mul_u64(N_VAL).mul_u64(N_VAL);
    let mut imax = 0usize;
    for i in 0..alen {
        if max_t.ge(a[i]) {
            imax = i;
        } else {
            break;
        }
    }

    let a = &a[..alen];
    let ans: i64 = (1..(N_VAL as usize + 1))
        .into_par_iter()
        .with_min_len(256)
        .map(|j| {
            let j = j as u64;
            let mut target = a_l.mul_u64(j).mul_u64(j);
            let mut count = 0i64;
            let mut i = imax;
            loop {
                // SAFETY: i < alen by construction (imax < alen, only decreases).
                let ai = unsafe { *a.get_unchecked(i) };
                if target.ge(ai) {
                    target = target.sub(ai);
                    count += 1;
                    if i < 3 {
                        break;
                    }
                    i -= 3;
                } else if i == 0 {
                    break;
                } else {
                    i -= 1;
                }
            }
            count
        })
        .sum();

    println!("{ans}");
}
