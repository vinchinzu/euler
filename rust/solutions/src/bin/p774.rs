// Project Euler 774 - Conjunctive Sequences
// Tensor-Train / MPS representation with left-sweep Gaussian elimination compression.
//
// Count sequences of length n with each a_i in [0,b], consecutive a_i & a_{i+1} != 0.
// Uses inclusion-exclusion: conjunctive = total - disjoint, represented in TT format.
//
// Optimizations over original:
// - Left-sweep only (no reduce_right), matching C reference approach
// - Pure i64 arithmetic (MOD < 2^30, all products fit in i64)
// - unsafe get_unchecked in proven-safe hot loops

const MOD: i64 = 998244353;

#[inline(always)]
fn modd(x: i64) -> i64 {
    let r = x % MOD;
    r + (r >> 63 & MOD)
}

#[inline(always)]
fn mulmod(a: i64, b: i64) -> i64 {
    a * b % MOD
}

fn powmod(mut base: i64, mut exp: i64) -> i64 {
    let mut r = 1i64;
    base %= MOD;
    while exp > 0 {
        if exp & 1 == 1 {
            r = r * base % MOD;
        }
        base = base * base % MOD;
        exp >>= 1;
    }
    r
}

#[inline]
fn modinv(a: i64) -> i64 {
    powmod(modd(a), MOD - 2)
}

#[derive(Clone)]
struct Core {
    r_l: usize,
    r_r: usize,
    data: Vec<i64>,
}

impl Core {
    fn new(r_l: usize, r_r: usize) -> Self {
        Core {
            r_l,
            r_r,
            data: vec![0; r_l * 2 * r_r],
        }
    }
    #[inline(always)]
    fn get(&self, l: usize, bit: usize, r: usize) -> i64 {
        // SAFETY: callers ensure l < r_l, bit < 2, r < r_r
        unsafe { *self.data.get_unchecked(l * 2 * self.r_r + bit * self.r_r + r) }
    }
    #[inline(always)]
    fn set(&mut self, l: usize, bit: usize, r: usize, val: i64) {
        let i = l * 2 * self.r_r + bit * self.r_r + r;
        // SAFETY: callers ensure l < r_l, bit < 2, r < r_r
        unsafe { *self.data.get_unchecked_mut(i) = val; }
    }
}

#[derive(Clone)]
struct TT {
    m: usize,
    cores: Vec<Core>,
}

impl TT {
    fn all_ones(m: usize) -> Self {
        let cores = (0..m)
            .map(|_| {
                let mut c = Core::new(1, 1);
                c.set(0, 0, 0, 1);
                c.set(0, 1, 0, 1);
                c
            })
            .collect();
        TT { m, cores }
    }

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
            let t: [[[i64; 2]; 2]; 2] = if bb == 0 {
                [[[1, 0], [0, 1]], [[1, 0], [0, 0]]]
            } else {
                [[[1, 0], [1, 0]], [[1, 0], [0, 1]]]
            };
            if idx == 0 {
                let mut c = Core::new(1, 2);
                for xbit in 0..2 {
                    c.set(0, xbit, 0, t[xbit][1][0]);
                    c.set(0, xbit, 1, t[xbit][1][1]);
                }
                cores.push(c);
            } else if idx == m - 1 {
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

    fn scalar_mul(&self, c: i64) -> Self {
        let c = modd(c);
        let mut tt = self.clone();
        for v in tt.cores[0].data.iter_mut() {
            *v = mulmod(*v, c);
        }
        tt
    }

    fn add(&self, other: &TT, coef_b: i64) -> Self {
        let coef_b = modd(coef_b);
        let m = self.m;
        let mut cores = Vec::with_capacity(m);
        for i in 0..m {
            let a = &self.cores[i];
            let b = &other.cores[i];
            if i == 0 {
                let mut c = Core::new(1, a.r_r + b.r_r);
                for bit in 0..2 {
                    for r in 0..a.r_r {
                        c.set(0, bit, r, a.get(0, bit, r));
                    }
                    for r in 0..b.r_r {
                        c.set(0, bit, a.r_r + r, mulmod(b.get(0, bit, r), coef_b));
                    }
                }
                cores.push(c);
            } else if i == m - 1 {
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

    fn hadamard(&self, other: &TT) -> Self {
        let m = self.m;
        let mut cores = Vec::with_capacity(m);
        for i in 0..m {
            let a = &self.cores[i];
            let b = &other.cores[i];
            let r_l = a.r_l * b.r_l;
            let r_r = a.r_r * b.r_r;
            let mut c = Core::new(r_l, r_r);
            let b_r_l = b.r_l;
            let b_r_r = b.r_r;
            for la in 0..a.r_l {
                for lb in 0..b_r_l {
                    let l = la * b_r_l + lb;
                    for bit in 0..2usize {
                        for ra in 0..a.r_r {
                            let av = a.get(la, bit, ra);
                            if av == 0 { continue; }
                            let c_base = l * 2 * r_r + bit * r_r + ra * b_r_r;
                            for rb in 0..b_r_r {
                                let bv = b.get(lb, bit, rb);
                                if bv == 0 { continue; }
                                // SAFETY: c_base + rb < r_l * 2 * r_r
                                unsafe {
                                    let p = c.data.get_unchecked_mut(c_base + rb);
                                    *p = (*p + mulmod(av, bv)) % MOD;
                                }
                            }
                        }
                    }
                }
            }
            cores.push(c);
        }
        TT { m, cores }
    }

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
                    c.set(l, 0, r, (mat[0][0] * a0 + mat[0][1] * a1) % MOD);
                    c.set(l, 1, r, (mat[1][0] * a0 + mat[1][1] * a1) % MOD);
                }
            }
            cores.push(c);
        }
        TT { m, cores }
    }

    fn sum_all(&self) -> i64 {
        let mut vec = vec![1i64];
        for i in 0..self.m {
            let c = &self.cores[i];
            let mut newvec = vec![0i64; c.r_r];
            for l in 0..c.r_l {
                let vl = vec[l];
                if vl == 0 { continue; }
                for bit in 0..2 {
                    for r in 0..c.r_r {
                        newvec[r] = (newvec[r] + mulmod(vl, c.get(l, bit, r))) % MOD;
                    }
                }
            }
            vec = newvec;
        }
        modd(vec[0])
    }

    fn gauss_elim(mat: &mut [i64], nrows: usize, ncols: usize) -> Vec<usize> {
        let mut pivots = Vec::new();
        let mut row_ptr = 0;
        for c in 0..ncols {
            if row_ptr >= nrows { break; }
            let mut piv = usize::MAX;
            for rr in row_ptr..nrows {
                // SAFETY: rr * ncols + c < nrows * ncols = mat.len()
                if unsafe { *mat.get_unchecked(rr * ncols + c) } != 0 {
                    piv = rr; break;
                }
            }
            if piv == usize::MAX { continue; }
            if piv != row_ptr {
                for j in 0..ncols {
                    mat.swap(row_ptr * ncols + j, piv * ncols + j);
                }
            }
            let pivot_base = row_ptr * ncols;
            let inv = modinv(unsafe { *mat.get_unchecked(pivot_base + c) });
            for j in c..ncols {
                unsafe {
                    let p = mat.get_unchecked_mut(pivot_base + j);
                    *p = mulmod(*p, inv);
                }
            }
            for rr in 0..nrows {
                if rr == row_ptr { continue; }
                let rr_base = rr * ncols;
                let f = unsafe { *mat.get_unchecked(rr_base + c) };
                if f != 0 {
                    for j in c..ncols {
                        unsafe {
                            let piv_val = *mat.get_unchecked(pivot_base + j);
                            let p = mat.get_unchecked_mut(rr_base + j);
                            *p = modd(*p - mulmod(f, piv_val));
                        }
                    }
                }
            }
            pivots.push(c);
            row_ptr += 1;
        }
        pivots
    }

    fn reduce_left(&mut self) {
        let m = self.m;
        for i in 0..m - 1 {
            let r_l = self.cores[i].r_l;
            let r_r = self.cores[i].r_r;
            if r_r <= 1 { continue; }
            let nrows = 2 * r_l;
            let mut mat = vec![0i64; nrows * r_r];
            for l in 0..r_l {
                for r in 0..r_r {
                    mat[(2 * l) * r_r + r] = self.cores[i].get(l, 0, r);
                    mat[(2 * l + 1) * r_r + r] = self.cores[i].get(l, 1, r);
                }
            }
            let pivots = Self::gauss_elim(&mut mat, nrows, r_r);
            let rank = pivots.len();
            if rank == 0 || rank == r_r { continue; }

            let mut new_core = Core::new(r_l, rank);
            for l in 0..r_l {
                for (k, &p) in pivots.iter().enumerate() {
                    new_core.set(l, 0, k, self.cores[i].get(l, 0, p));
                    new_core.set(l, 1, k, self.cores[i].get(l, 1, p));
                }
            }

            let r_next = self.cores[i + 1].r_r;
            let mut new_nxt = Core::new(rank, r_next);
            let mut is_pivot = vec![false; r_r];
            for &p in &pivots { is_pivot[p] = true; }

            for (k, &p) in pivots.iter().enumerate() {
                for bit in 0..2 {
                    for t in 0..r_next {
                        new_nxt.set(k, bit, t, self.cores[i + 1].get(p, bit, t));
                    }
                }
            }

            for j in 0..r_r {
                if is_pivot[j] { continue; }
                for k in 0..rank {
                    let coeff = unsafe { *mat.get_unchecked(k * r_r + j) };
                    if coeff == 0 { continue; }
                    for bit in 0..2 {
                        let dst_base = k * 2 * r_next + bit * r_next;
                        let src_base = j * 2 * r_next + bit * r_next;
                        for t in 0..r_next {
                            unsafe {
                                let src = *self.cores[i + 1].data.get_unchecked(src_base + t);
                                let dst = new_nxt.data.get_unchecked_mut(dst_base + t);
                                *dst = modd(*dst + mulmod(coeff, src));
                            }
                        }
                    }
                }
            }

            self.cores[i] = new_core;
            self.cores[i + 1] = new_nxt;
        }
    }
}

fn solve(n: usize, b: i64) -> i64 {
    let m = if b == 0 { 1 } else { (64 - b.leading_zeros()) as usize };

    let mut mask = TT::indicator_leq(b, m);
    mask.reduce_left();

    let mut dp = TT::indicator_leq(b, m);
    dp.reduce_left();

    let ones = TT::all_ones(m);
    let r_disjoint: [[i64; 2]; 2] = [[1, 1], [1, 0]];

    for _step in 0..n - 1 {
        let total = dp.sum_all();
        let j = ones.scalar_mul(total);
        let bv = dp.apply_local(&r_disjoint);
        let nxt = j.add(&bv, MOD - 1);
        let mut masked = nxt.hadamard(&mask);
        masked.reduce_left();
        dp = masked;
    }

    dp.sum_all()
}

fn main() {
    println!("{}", solve(123, 123456789));
}
