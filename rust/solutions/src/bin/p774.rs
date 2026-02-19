// Project Euler 774 - Conjunctive Sequences
// Tensor-Train / MPS representation with bidirectional Gaussian elimination compression.
//
// Count sequences of length n with each a_i in [0,b], consecutive a_i & a_{i+1} != 0.
// Uses inclusion-exclusion: conjunctive = total - disjoint, represented in TT format.
// Key fix: bidirectional reduction (reduce_left + reduce_right) after EVERY operation.

const MOD: i64 = 998244353;

fn modd(x: i64) -> i64 {
    ((x % MOD) + MOD) % MOD
}

fn powmod(mut base: i64, mut exp: i64, m: i64) -> i64 {
    let mut r = 1i64;
    base %= m;
    while exp > 0 {
        if exp & 1 == 1 {
            r = (r as i128 * base as i128 % m as i128) as i64;
        }
        base = (base as i128 * base as i128 % m as i128) as i64;
        exp >>= 1;
    }
    r
}

fn modinv(a: i64) -> i64 {
    powmod(modd(a), MOD - 2, MOD)
}

#[derive(Clone)]
struct Core {
    r_l: usize,
    r_r: usize,
    data: Vec<i64>, // r_l * 2 * r_r, layout: data[l * 2 * r_r + bit * r_r + r]
}

impl Core {
    fn new(r_l: usize, r_r: usize) -> Self {
        Core {
            r_l,
            r_r,
            data: vec![0; r_l * 2 * r_r],
        }
    }
    #[inline]
    fn get(&self, l: usize, bit: usize, r: usize) -> i64 {
        self.data[l * 2 * self.r_r + bit * self.r_r + r]
    }
    #[inline]
    fn set(&mut self, l: usize, bit: usize, r: usize, val: i64) {
        self.data[l * 2 * self.r_r + bit * self.r_r + r] = val;
    }
}

#[derive(Clone)]
struct TT {
    m: usize,
    cores: Vec<Core>,
}

impl TT {
    /// Constant function f(x) = 1 for all x in {0,1}^m.
    fn all_ones(m: usize) -> Self {
        let cores: Vec<Core> = (0..m)
            .map(|_| {
                let mut c = Core::new(1, 1);
                c.set(0, 0, 0, 1);
                c.set(0, 1, 0, 1);
                c
            })
            .collect();
        TT { m, cores }
    }

    /// Indicator function [x <= b] for x in {0,1}^m (MSB-first bit ordering).
    /// Uses digit-DP with states: 0=loose (already strictly less), 1=tight (equal so far).
    fn indicator_leq(b: i64, m: usize) -> Self {
        let bits: Vec<usize> = (0..m)
            .map(|i| ((b >> (m - 1 - i)) & 1) as usize)
            .collect();
        let mut cores = Vec::with_capacity(m);

        if m == 1 {
            let mut c = Core::new(1, 1);
            c.set(0, 0, 0, 1);
            c.set(0, 1, 0, if bits[0] >= 1 { 1 } else { 0 });
            cores.push(c);
            return TT { m, cores };
        }

        for idx in 0..m {
            let bb = bits[idx];
            // t[xbit][prev_state][next_state]
            let t: [[[i64; 2]; 2]; 2] = if bb == 0 {
                [
                    [[1, 0], [0, 1]], // xbit=0: loose->loose, tight->tight
                    [[1, 0], [0, 0]], // xbit=1: loose->loose, tight->reject
                ]
            } else {
                [
                    [[1, 0], [1, 0]], // xbit=0: loose->loose, tight->loose
                    [[1, 0], [0, 1]], // xbit=1: loose->loose, tight->tight
                ]
            };

            if idx == 0 {
                // Start in tight state
                let mut c = Core::new(1, 2);
                for xbit in 0..2 {
                    c.set(0, xbit, 0, t[xbit][1][0]);
                    c.set(0, xbit, 1, t[xbit][1][1]);
                }
                cores.push(c);
            } else if idx == m - 1 {
                // Accept both final states
                let mut c = Core::new(2, 1);
                for prev in 0..2 {
                    for xbit in 0..2 {
                        c.set(prev, xbit, 0, modd(t[xbit][prev][0] + t[xbit][prev][1]));
                    }
                }
                cores.push(c);
            } else {
                let mut c = Core::new(2, 2);
                for prev in 0..2 {
                    for xbit in 0..2 {
                        for next in 0..2 {
                            c.set(prev, xbit, next, t[xbit][prev][next]);
                        }
                    }
                }
                cores.push(c);
            }
        }
        TT { m, cores }
    }

    /// Multiply all entries by scalar c.
    fn scalar_mul(&self, c: i64) -> Self {
        let c = modd(c);
        let mut tt = self.clone();
        for j in 0..tt.cores[0].data.len() {
            tt.cores[0].data[j] =
                (tt.cores[0].data[j] as i128 * c as i128 % MOD as i128) as i64;
        }
        tt
    }

    /// TT addition: self + coef_b * other (direct sum of bond spaces).
    fn add(&self, other: &TT, coef_b: i64) -> Self {
        let coef_b = modd(coef_b);
        let m = self.m;
        let mut cores = Vec::with_capacity(m);

        for i in 0..m {
            let a = &self.cores[i];
            let b = &other.cores[i];

            if i == 0 {
                // First core: r_l=1, concatenate right bonds
                let mut c = Core::new(1, a.r_r + b.r_r);
                for bit in 0..2 {
                    for r in 0..a.r_r {
                        c.set(0, bit, r, a.get(0, bit, r));
                    }
                    for r in 0..b.r_r {
                        c.set(
                            0,
                            bit,
                            a.r_r + r,
                            (b.get(0, bit, r) as i128 * coef_b as i128 % MOD as i128) as i64,
                        );
                    }
                }
                cores.push(c);
            } else if i == m - 1 {
                // Last core: r_r=1, concatenate left bonds
                let mut c = Core::new(a.r_l + b.r_l, 1);
                for l in 0..a.r_l {
                    for bit in 0..2 {
                        c.set(l, bit, 0, a.get(l, bit, 0));
                    }
                }
                for l in 0..b.r_l {
                    for bit in 0..2 {
                        c.set(a.r_l + l, bit, 0, b.get(l, bit, 0));
                    }
                }
                cores.push(c);
            } else {
                // Middle core: block-diagonal
                let mut c = Core::new(a.r_l + b.r_l, a.r_r + b.r_r);
                for l in 0..a.r_l {
                    for bit in 0..2 {
                        for r in 0..a.r_r {
                            c.set(l, bit, r, a.get(l, bit, r));
                        }
                    }
                }
                for l in 0..b.r_l {
                    for bit in 0..2 {
                        for r in 0..b.r_r {
                            c.set(a.r_l + l, bit, a.r_r + r, b.get(l, bit, r));
                        }
                    }
                }
                cores.push(c);
            }
        }
        TT { m, cores }
    }

    /// Hadamard (element-wise) product: (self * other)(x) = self(x) * other(x).
    fn hadamard(&self, other: &TT) -> Self {
        let m = self.m;
        let mut cores = Vec::with_capacity(m);

        for i in 0..m {
            let a = &self.cores[i];
            let b = &other.cores[i];
            let r_l = a.r_l * b.r_l;
            let r_r = a.r_r * b.r_r;
            let mut c = Core::new(r_l, r_r);

            for la in 0..a.r_l {
                for lb in 0..b.r_l {
                    let l = la * b.r_l + lb;
                    for bit in 0..2 {
                        for ra in 0..a.r_r {
                            let av = a.get(la, bit, ra);
                            if av == 0 {
                                continue;
                            }
                            for rb in 0..b.r_r {
                                let bv = b.get(lb, bit, rb);
                                if bv == 0 {
                                    continue;
                                }
                                let r = ra * b.r_r + rb;
                                let old = c.get(l, bit, r);
                                c.set(
                                    l,
                                    bit,
                                    r,
                                    ((old as i128 + av as i128 * bv as i128 % MOD as i128)
                                        % MOD as i128) as i64,
                                );
                            }
                        }
                    }
                }
            }
            cores.push(c);
        }
        TT { m, cores }
    }

    /// Apply a 2x2 matrix to the bit dimension at every core independently.
    /// new[bit_out] = sum_{bit_in} mat[bit_out][bit_in] * old[bit_in]
    fn apply_local(&self, mat: &[[i64; 2]; 2]) -> Self {
        let m = self.m;
        let mut cores = Vec::with_capacity(m);

        for i in 0..m {
            let s = &self.cores[i];
            let mut c = Core::new(s.r_l, s.r_r);
            for l in 0..s.r_l {
                for r in 0..s.r_r {
                    let a0 = s.get(l, 0, r);
                    let a1 = s.get(l, 1, r);
                    c.set(
                        l,
                        0,
                        r,
                        ((mat[0][0] as i128 * a0 as i128 + mat[0][1] as i128 * a1 as i128)
                            .rem_euclid(MOD as i128)) as i64,
                    );
                    c.set(
                        l,
                        1,
                        r,
                        ((mat[1][0] as i128 * a0 as i128 + mat[1][1] as i128 * a1 as i128)
                            .rem_euclid(MOD as i128)) as i64,
                    );
                }
            }
            cores.push(c);
        }
        TT { m, cores }
    }

    /// Contract the full TT to get sum over all x of f(x).
    fn sum_all(&self) -> i64 {
        let mut vec = vec![1i64];
        for i in 0..self.m {
            let c = &self.cores[i];
            let mut newvec = vec![0i64; c.r_r];
            for l in 0..c.r_l {
                if vec[l] == 0 {
                    continue;
                }
                for bit in 0..2 {
                    for r in 0..c.r_r {
                        newvec[r] = ((newvec[r] as i128
                            + vec[l] as i128 * c.get(l, bit, r) as i128 % MOD as i128)
                            % MOD as i128) as i64;
                    }
                }
            }
            vec = newvec;
        }
        modd(vec[0])
    }

    /// Gaussian elimination to RREF on an nrows x ncols matrix (mod MOD).
    /// Returns pivot column indices.
    fn gauss_elim(mat: &mut [i64], nrows: usize, ncols: usize) -> Vec<usize> {
        let mut pivots = Vec::new();
        let mut row_ptr = 0;

        for c in 0..ncols {
            if row_ptr >= nrows {
                break;
            }
            // Find pivot in column c at or below row_ptr
            let mut piv = None;
            for rr in row_ptr..nrows {
                if mat[rr * ncols + c] != 0 {
                    piv = Some(rr);
                    break;
                }
            }
            let piv = match piv {
                Some(p) => p,
                None => continue,
            };

            // Swap pivot row into position
            if piv != row_ptr {
                for j in 0..ncols {
                    mat.swap(row_ptr * ncols + j, piv * ncols + j);
                }
            }

            // Normalize pivot row
            let inv = modinv(mat[row_ptr * ncols + c]);
            for j in c..ncols {
                mat[row_ptr * ncols + j] =
                    (mat[row_ptr * ncols + j] as i128 * inv as i128 % MOD as i128) as i64;
            }

            // Eliminate column c in ALL other rows (full RREF)
            for rr in 0..nrows {
                if rr == row_ptr {
                    continue;
                }
                let f = mat[rr * ncols + c];
                if f != 0 {
                    for j in c..ncols {
                        mat[rr * ncols + j] = modd(
                            mat[rr * ncols + j]
                                - (f as i128 * mat[row_ptr * ncols + j] as i128 % MOD as i128)
                                    as i64,
                        );
                    }
                }
            }

            pivots.push(c);
            row_ptr += 1;
        }
        pivots
    }

    /// Left-to-right sweep: compress right bond dimensions.
    /// For each core i, finds linearly independent columns of the reshaped (2*r_l) x r_r matrix.
    /// Absorbs redundant columns into core i+1 via RREF coefficients.
    fn reduce_left(&mut self) {
        let m = self.m;
        for i in 0..m - 1 {
            let r_l = self.cores[i].r_l;
            let r_r = self.cores[i].r_r;
            if r_r <= 1 {
                continue;
            }
            let nrows = 2 * r_l;

            // Reshape core i as (2*r_l) x r_r matrix: mat[2*l+bit, r]
            let mut mat = vec![0i64; nrows * r_r];
            for l in 0..r_l {
                for r in 0..r_r {
                    mat[(2 * l) * r_r + r] = self.cores[i].get(l, 0, r);
                    mat[(2 * l + 1) * r_r + r] = self.cores[i].get(l, 1, r);
                }
            }

            let pivots = Self::gauss_elim(&mut mat, nrows, r_r);
            let rank = pivots.len();

            if rank == 0 || rank == r_r {
                continue;
            }

            // New core i: keep only pivot columns from the original core
            let mut new_core = Core::new(r_l, rank);
            for l in 0..r_l {
                for (k, &p) in pivots.iter().enumerate() {
                    new_core.set(l, 0, k, self.cores[i].get(l, 0, p));
                    new_core.set(l, 1, k, self.cores[i].get(l, 1, p));
                }
            }

            // New next core: absorb linear combinations
            // new_nxt[k, bit, t] = cores[i+1][p_k, bit, t] + sum_{j non-pivot} rref[k,j] * cores[i+1][j, bit, t]
            let r_next = self.cores[i + 1].r_r;
            let mut new_nxt = Core::new(rank, r_next);

            let mut is_pivot = vec![false; r_r];
            for &p in &pivots {
                is_pivot[p] = true;
            }

            // Pivot contributions (identity)
            for (k, &p) in pivots.iter().enumerate() {
                for bit in 0..2 {
                    for t in 0..r_next {
                        new_nxt.set(k, bit, t, self.cores[i + 1].get(p, bit, t));
                    }
                }
            }

            // Non-pivot contributions via RREF coefficients
            for j in 0..r_r {
                if is_pivot[j] {
                    continue;
                }
                for k in 0..rank {
                    let coeff = mat[k * r_r + j];
                    if coeff == 0 {
                        continue;
                    }
                    for bit in 0..2 {
                        for t in 0..r_next {
                            let old = new_nxt.get(k, bit, t);
                            new_nxt.set(
                                k,
                                bit,
                                t,
                                modd(
                                    old + (coeff as i128
                                        * self.cores[i + 1].get(j, bit, t) as i128
                                        % MOD as i128)
                                        as i64,
                                ),
                            );
                        }
                    }
                }
            }

            self.cores[i] = new_core;
            self.cores[i + 1] = new_nxt;
        }
    }

    /// Right-to-left sweep: compress left bond dimensions.
    /// For each core i, finds linearly independent rows of the reshaped r_l x (2*r_r) matrix
    /// by transposing to (2*r_r) x r_l and doing column reduction.
    /// Absorbs redundant rows into core i-1 via RREF coefficients.
    fn reduce_right(&mut self) {
        let m = self.m;
        for i in (1..m).rev() {
            let r_l = self.cores[i].r_l;
            let r_r = self.cores[i].r_r;
            if r_l <= 1 {
                continue;
            }

            // Form M^T: (2*r_r) x r_l matrix where M^T[bit*r_r + r, l] = core_i[l, bit, r]
            let nrows = 2 * r_r;
            let ncols = r_l;

            let mut mat = vec![0i64; nrows * ncols];
            for l in 0..r_l {
                for r in 0..r_r {
                    mat[r * ncols + l] = self.cores[i].get(l, 0, r);
                    mat[(r_r + r) * ncols + l] = self.cores[i].get(l, 1, r);
                }
            }

            // Pivot columns of M^T = linearly independent left-bond indices
            let pivots = Self::gauss_elim(&mut mat, nrows, ncols);
            let rank = pivots.len();

            if rank == 0 || rank == r_l {
                continue;
            }

            // New core i: keep only pivot left-bond indices
            let mut new_core = Core::new(rank, r_r);
            for (k, &p) in pivots.iter().enumerate() {
                for bit in 0..2 {
                    for r in 0..r_r {
                        new_core.set(k, bit, r, self.cores[i].get(p, bit, r));
                    }
                }
            }

            // New previous core i-1: absorb linear combinations
            // For non-pivot left-bond index j: col_j of M^T = sum_k rref[k,j] * col_{p_k}
            // => core_i[j, bit, r] = sum_k rref[k,j] * core_i[p_k, bit, r]
            // => new_prev[l', bit', k] = prev[l', bit', p_k] + sum_{j non-pivot} rref[k,j] * prev[l', bit', j]
            let r_prev_l = self.cores[i - 1].r_l;
            let mut new_prev = Core::new(r_prev_l, rank);

            let mut is_pivot = vec![false; r_l];
            for &p in &pivots {
                is_pivot[p] = true;
            }

            // Pivot contributions (identity)
            for (k, &p) in pivots.iter().enumerate() {
                for l in 0..r_prev_l {
                    for bit in 0..2 {
                        new_prev.set(l, bit, k, self.cores[i - 1].get(l, bit, p));
                    }
                }
            }

            // Non-pivot contributions via RREF coefficients
            for j in 0..r_l {
                if is_pivot[j] {
                    continue;
                }
                for k in 0..rank {
                    let coeff = mat[k * ncols + j];
                    if coeff == 0 {
                        continue;
                    }
                    for l in 0..r_prev_l {
                        for bit in 0..2 {
                            let old = new_prev.get(l, bit, k);
                            new_prev.set(
                                l,
                                bit,
                                k,
                                modd(
                                    old + (coeff as i128
                                        * self.cores[i - 1].get(l, bit, j) as i128
                                        % MOD as i128)
                                        as i64,
                                ),
                            );
                        }
                    }
                }
            }

            self.cores[i] = new_core;
            self.cores[i - 1] = new_prev;
        }
    }

    /// Bidirectional compression: left sweep then right sweep.
    /// One pass in each direction yields the exact minimal-rank TT (over Z_p).
    fn reduce(&mut self) {
        self.reduce_left();
        self.reduce_right();
    }
}

fn solve(n: usize, b: i64) -> i64 {
    let m = if b == 0 {
        1
    } else {
        (64 - b.leading_zeros()) as usize
    };

    let mut mask = TT::indicator_leq(b, m);
    mask.reduce();

    let mut dp = TT::indicator_leq(b, m);
    dp.reduce();

    let ones = TT::all_ones(m);

    // r_disjoint[bit_y][bit_x]: for x&y=0, at each bit position:
    //   if y_bit=0: both x_bit=0 and x_bit=1 contribute (sum)
    //   if y_bit=1: only x_bit=0 contributes (x must have 0 here for disjointness)
    let r_disjoint: [[i64; 2]; 2] = [[1, 1], [1, 0]];

    for _step in 0..n - 1 {
        let total = dp.sum_all();
        let j = ones.scalar_mul(total);

        // apply_local preserves bond dims; reduce usually finds nothing here
        let bv = dp.apply_local(&r_disjoint);

        // add grows ranks by +1; reduce after it finds little
        let nxt = j.add(&bv, MOD - 1); // total - disjoint

        // hadamard doubles ranks (mask has rank 2); this is where reduce matters most
        let mut masked = nxt.hadamard(&mask);
        masked.reduce();

        dp = masked;
    }

    dp.sum_all()
}

fn main() {
    println!("{}", solve(123, 123456789));
}
