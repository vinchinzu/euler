// Problem 945: XOR-Product Equation
// (a ⊗ a) ⊕ (2 ⊗ a ⊗ b) ⊕ (b ⊗ b) = c ⊗ c
// F(N) = count of (a, b) with 0 ≤ a ≤ b ≤ N having a solution c.
//
// XOR-product is multiplication in GF(2)[x]. With even/odd bit splits
// Ae, Ao of a (as 12-bit polynomials), valid b satisfy
//   Ae(y) Be(y) = y Ao(y) Bo(y)
// in GF(2)[y], deg Be, Bo < 12. Solutions are polynomial multiples
// T(y) * (Be0, Bo0) of a single generator, i.e. even shifts of
// gen = interleave(Be0, Bo0). Count those in [a, N].

use rayon::prelude::*;

fn main() {
    let n: u32 = 10_000_000;
    println!("{}", solve(n));
}

fn solve(n: u32) -> u64 {
    let rest: u64 = (1..n + 1)
        .into_par_iter()
        .with_min_len(2048)
        .map(|a| count_b(a, n))
        .sum();
    rest + n as u64 + 1
}

#[inline(always)]
fn count_b(a: u32, n: u32) -> u64 {
    let (ae, ao) = split(a);
    let ao2 = ao << 1;
    let g = gf2_gcd(ae, ao2);
    let be = gf2_div(ao2, g);
    let bo = gf2_div(ae, g);
    // dim = min(12 - deg(be), 12 - deg(bo)); deg(0) = -1 so lz(0)-19 = 13
    let dim_i = (be.leading_zeros() as i32 - 19).min(bo.leading_zeros() as i32 - 19);
    if dim_i <= 0 {
        return 0;
    }
    let dim = dim_i as u32;
    let g0 = interleave(be, bo);
    if dim <= 8 {
        count_enum(g0, dim, a, n)
    } else {
        count_leq(g0, dim, n) - count_leq(g0, dim, a - 1)
    }
}

#[inline(always)]
fn count_enum(g0: u32, dim: u32, a: u32, n: u32) -> u64 {
    let lim = n - a;
    let mut x = 0u32;
    let mut ans = 0u64;
    let last = 1u32 << dim;
    for k in 1..last {
        x ^= g0 << (2 * k.trailing_zeros());
        ans += (x.wrapping_sub(a) <= lim) as u64;
    }
    ans
}

/// Count XOR-subsets of {g0, g0<<2, ..., g0<<2(dim-1)} that are <= upper.
#[inline(always)]
fn count_leq(g0: u32, dim: u32, upper: u32) -> u64 {
    let msb_g = 31 - g0.leading_zeros();
    let mut count = 0u64;
    let mut val = 0u32;
    let mut rem = dim;
    for bit_pos in (0..24u32).rev() {
        if rem > 0 && bit_pos == msb_g + 2 * (rem - 1) {
            let bv = g0 << (2 * (rem - 1));
            rem -= 1;
            let vb = (val >> bit_pos) & 1;
            let ub = (upper >> bit_pos) & 1;
            if ub == 1 {
                count += 1u64 << rem;
            }
            if vb != ub {
                val ^= bv;
            }
        } else {
            let vb = (val >> bit_pos) & 1;
            let ub = (upper >> bit_pos) & 1;
            if vb != ub {
                if vb < ub {
                    count += 1u64 << rem;
                }
                return count;
            }
        }
    }
    count + 1
}

#[inline(always)]
fn gf2_gcd(mut a: u32, mut b: u32) -> u32 {
    while b != 0 {
        let lb = b.leading_zeros();
        while a != 0 {
            let la = a.leading_zeros();
            if la > lb {
                break;
            }
            a ^= b << (lb - la);
        }
        core::mem::swap(&mut a, &mut b);
    }
    a
}

#[inline(always)]
fn gf2_div(mut a: u32, b: u32) -> u32 {
    let lb = b.leading_zeros();
    let mut q = 0u32;
    while a != 0 {
        let la = a.leading_zeros();
        if la > lb {
            break;
        }
        let sh = lb - la;
        q |= 1 << sh;
        a ^= b << sh;
    }
    q
}

#[inline(always)]
fn split(x: u32) -> (u32, u32) {
    #[cfg(all(target_arch = "x86_64", target_feature = "bmi2"))]
    unsafe {
        (
            core::arch::x86_64::_pext_u32(x, 0x5555_5555),
            core::arch::x86_64::_pext_u32(x, 0xAAAA_AAAA),
        )
    }
    #[cfg(not(all(target_arch = "x86_64", target_feature = "bmi2")))]
    {
        let mut even = x & 0x5555_5555;
        even = (even | (even >> 1)) & 0x3333_3333;
        even = (even | (even >> 2)) & 0x0F0F_0F0F;
        even = (even | (even >> 4)) & 0x00FF_00FF;
        even = (even | (even >> 8)) & 0x0000_FFFF;
        let mut odd = (x >> 1) & 0x5555_5555;
        odd = (odd | (odd >> 1)) & 0x3333_3333;
        odd = (odd | (odd >> 2)) & 0x0F0F_0F0F;
        odd = (odd | (odd >> 4)) & 0x00FF_00FF;
        odd = (odd | (odd >> 8)) & 0x0000_FFFF;
        (even, odd)
    }
}

#[inline(always)]
fn interleave(even: u32, odd: u32) -> u32 {
    #[cfg(all(target_arch = "x86_64", target_feature = "bmi2"))]
    unsafe {
        core::arch::x86_64::_pdep_u32(even, 0x5555_5555)
            | core::arch::x86_64::_pdep_u32(odd, 0xAAAA_AAAA)
    }
    #[cfg(not(all(target_arch = "x86_64", target_feature = "bmi2")))]
    {
        let mut e = even & 0x0FFF;
        e = (e | (e << 8)) & 0x00FF_00FF;
        e = (e | (e << 4)) & 0x0F0F_0F0F;
        e = (e | (e << 2)) & 0x3333_3333;
        e = (e | (e << 1)) & 0x5555_5555;
        let mut o = odd & 0x0FFF;
        o = (o | (o << 8)) & 0x00FF_00FF;
        o = (o | (o << 4)) & 0x0F0F_0F0F;
        o = (o | (o << 2)) & 0x3333_3333;
        o = (o | (o << 1)) & 0x5555_5555;
        e | (o << 1)
    }
}
