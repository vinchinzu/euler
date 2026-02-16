// Problem 945: XOR-Product Equation
// (a ⊗ a) ⊕ (2 ⊗ a ⊗ b) ⊕ (b ⊗ b) = c ⊗ c
// F(N) = count of (a, b) with 0 ≤ a ≤ b ≤ N having a solution c.
//
// Key insight: XOR-product is multiplication in GF(2)[x].
// The equation reduces to: A_e*B_e = x*A_o*B_o in GF(2)[x]
// where A_e, A_o are the even/odd bit extractions of a, similarly for B.
// For each a, solve the linear system over GF(2) for valid (be, bo),
// convert to b via bit interleaving, count b in [a, N].

fn main() {
    let n: u32 = 10_000_000;
    let answer = solve(n);
    println!("{}", answer);
}

fn solve(n: u32) -> u64 {
    let mut total: u64 = 0;
    for a in 0..=n {
        let (ae, ao) = split(a);
        let basis = get_basis(ae, ao);
        let cnt_n = count_xor_leq(&basis, n);
        let cnt_a = if a > 0 { count_xor_leq(&basis, a - 1) } else { 0 };
        total += (cnt_n - cnt_a) as u64;
    }
    total
}

#[inline]
fn split(n: u32) -> (u32, u32) {
    // Extract even-indexed bits and odd-indexed bits
    let mut ae: u32 = 0;
    let mut ao: u32 = 0;
    // Use parallel bit extraction
    // Even bits: positions 0,2,4,...,22 -> compress to 0,1,2,...,11
    // Odd bits: positions 1,3,5,...,23 -> compress to 0,1,2,...,11
    for i in 0..12 {
        ae |= ((n >> (2 * i)) & 1) << i;
        ao |= ((n >> (2 * i + 1)) & 1) << i;
    }
    (ae, ao)
}

#[inline]
fn interleave(ae: u32, ao: u32) -> u32 {
    let mut n: u32 = 0;
    for i in 0..12 {
        n |= ((ae >> i) & 1) << (2 * i);
        n |= ((ao >> i) & 1) << (2 * i + 1);
    }
    n
}

fn get_basis(ae: u32, ao: u32) -> Vec<u32> {
    // Build the linear system: for each product bit k (0..23),
    // sum_{i+j=k} ae_i * be_j  XOR  sum_{i+j=k-1} ao_i * bo_j = 0
    // Variables: be_0..be_11 (bits 0..11), bo_0..bo_11 (bits 12..23)

    let mut pivot: [u32; 24] = [0; 24]; // pivot[col] = row bitmask (0 if no pivot)
    let mut has_pivot = [false; 24];

    for k in 0..24u32 {
        let mut eq: u32 = 0;
        // ae * be contribution at bit k
        for i in 0..12u32 {
            if ae & (1 << i) != 0 {
                let j = k.wrapping_sub(i);
                if j < 12 {
                    eq ^= 1 << j;
                }
            }
        }
        // ao * bo contribution at bit k-1
        if k >= 1 {
            for i in 0..12u32 {
                if ao & (1 << i) != 0 {
                    let j = (k - 1).wrapping_sub(i);
                    if j < 12 {
                        eq ^= 1 << (12 + j);
                    }
                }
            }
        }
        if eq == 0 {
            continue;
        }

        // Forward elimination
        let mut cur = eq;
        loop {
            if cur == 0 {
                break;
            }
            let msb = 31 - cur.leading_zeros();
            if has_pivot[msb as usize] {
                cur ^= pivot[msb as usize];
            } else {
                pivot[msb as usize] = cur;
                has_pivot[msb as usize] = true;
                break;
            }
        }
    }

    // Back-substitution: for each pivot col (high to low), reduce higher pivots
    for pc in (0..24).rev() {
        if !has_pivot[pc] {
            continue;
        }
        for other_pc in (pc + 1)..24 {
            if has_pivot[other_pc] && (pivot[other_pc] & (1 << pc)) != 0 {
                pivot[other_pc] ^= pivot[pc];
            }
        }
    }

    // Build null space basis, convert to b
    let mut basis_b = Vec::new();
    for fv in 0..24u32 {
        if has_pivot[fv as usize] {
            continue;
        }
        let mut vec: u32 = 1 << fv;
        for pc in 0..24u32 {
            if has_pivot[pc as usize] && (pivot[pc as usize] & (1 << fv)) != 0 {
                vec |= 1 << pc;
            }
        }
        let be = vec & 0xFFF;
        let bo = (vec >> 12) & 0xFFF;
        let b = interleave(be, bo);
        basis_b.push(b);
    }
    basis_b
}

fn count_xor_leq(basis: &[u32], upper: u32) -> u64 {
    // Count XOR-subsets of basis that are <= upper.
    // First, reduce basis to greedy/canonical form (each with unique MSB).
    let mut reduced: Vec<u32> = Vec::new();
    for &v in basis {
        let mut cur = v;
        for &rv in &reduced {
            cur = cur.min(cur ^ rv);
        }
        if cur > 0 {
            reduced.push(cur);
            let new_len = reduced.len();
            let new_val = reduced[new_len - 1];
            for i in 0..new_len - 1 {
                reduced[i] = reduced[i].min(reduced[i] ^ new_val);
            }
        }
    }
    reduced.retain(|&v| v > 0);
    reduced.sort_unstable_by(|a, b| b.cmp(a)); // descending

    let d = reduced.len();
    let mut count: u64 = 0;
    let mut val: u32 = 0;
    let mut bidx: usize = 0;

    let max_bit = if reduced.is_empty() {
        if upper == 0 { 1 } else { 32 - upper.leading_zeros() }
    } else {
        let max_basis = 32 - reduced[0].leading_zeros();
        let max_upper = if upper == 0 { 1 } else { 32 - upper.leading_zeros() };
        max_basis.max(max_upper)
    };

    for bit_pos in (0..max_bit).rev() {
        let ub = (upper >> bit_pos) & 1;
        let has_bv = bidx < d && (32 - reduced[bidx].leading_zeros() - 1) == bit_pos;

        if has_bv {
            let bv = reduced[bidx];
            bidx += 1;
            let vb = (val >> bit_pos) & 1;

            if vb == 0 {
                // No use: bit stays 0. Use: bit becomes 1.
                if ub == 1 {
                    // 0 < 1. Don't use path: strictly less, all remaining free.
                    count += 1u64 << (d - bidx);
                    // Continue on use path (bit=1=ub, tight)
                    val ^= bv;
                }
                // else ub=0: don't use (tight), use exceeds.
            } else {
                // vb=1. No use: bit stays 1. Use: bit becomes 0.
                if ub == 1 {
                    // Use: 0 < 1=ub. Strictly less, all free.
                    count += 1u64 << (d - bidx);
                    // Continue on no-use path (bit=1=ub, tight)
                } else {
                    // ub=0. No use: 1>0 exceeds. Must use.
                    val ^= bv;
                }
            }
        } else {
            let vb = (val >> bit_pos) & 1;
            if vb > ub {
                return count;
            } else if vb < ub {
                count += 1u64 << (d - bidx);
                return count;
            }
        }
    }

    count += 1; // val <= upper
    count
}
