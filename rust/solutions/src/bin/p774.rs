// Project Euler 774 - Conjunctive Sequences
// Tensor-Train / MPS with left-sweep Gaussian elimination compression.
//
// Optimizations:
// - i32 storage (MOD < 2^30) → 2× denser cache lines in GE/hadamard
// - Extended-Euclid modinv (no Fermat powmod)
// - Pre-allocated scratch buffers; capacity-preserving mat reuse
// - unsafe get_unchecked + raw ptr GE inner loops
// - Avoid cloning ones each step: scale first core into a reusable buffer

const MOD: i32 = 998244353;

#[inline(always)]
fn modd(x: i32) -> i32 {
    let r = x % MOD;
    r + (((r >> 31) as u32 & MOD as u32) as i32)
}

#[inline(always)]
fn mulmod(a: i32, b: i32) -> i32 {
    ((a as i64 * b as i64) % MOD as i64) as i32
}

/// Extended Euclidean modular inverse.
#[inline(always)]
fn modinv(a: i32) -> i32 {
    let mut a = a % MOD;
    if a < 0 {
        a += MOD;
    }
    let mut t = 0i32;
    let mut newt = 1i32;
    let mut r = MOD;
    let mut newr = a;
    while newr != 0 {
        let q = r / newr;
        let tmp = newt;
        newt = t - q * newt;
        t = tmp;
        let tmp = newr;
        newr = r - q * newr;
        r = tmp;
    }
    if t < 0 {
        t + MOD
    } else {
        t
    }
}

struct Core {
    r_l: usize,
    r_r: usize,
    data: Vec<i32>,
}

impl Core {
    fn new(r_l: usize, r_r: usize) -> Self {
        Core {
            r_l,
            r_r,
            data: vec![0; r_l * 2 * r_r],
        }
    }
    fn with_capacity_like(r_l: usize, r_r: usize, cap_hint: usize) -> Self {
        let need = r_l * 2 * r_r;
        let mut data = Vec::with_capacity(need.max(cap_hint));
        data.resize(need, 0);
        Core { r_l, r_r, data }
    }
    #[inline(always)]
    fn get(&self, l: usize, bit: usize, r: usize) -> i32 {
        // SAFETY: callers ensure l < r_l, bit < 2, r < r_r
        unsafe { *self.data.get_unchecked(l * 2 * self.r_r + bit * self.r_r + r) }
    }
    #[inline(always)]
    fn set(&mut self, l: usize, bit: usize, r: usize, val: i32) {
        let i = l * 2 * self.r_r + bit * self.r_r + r;
        // SAFETY: callers ensure indices in range
        unsafe {
            *self.data.get_unchecked_mut(i) = val;
        }
    }
}

impl Clone for Core {
    fn clone(&self) -> Self {
        Core {
            r_l: self.r_l,
            r_r: self.r_r,
            data: self.data.clone(),
        }
    }
}

struct TT {
    m: usize,
    cores: Vec<Core>,
}

impl Clone for TT {
    fn clone(&self) -> Self {
        TT {
            m: self.m,
            cores: self.cores.clone(),
        }
    }
}

thread_local! {
    static SCRATCH: std::cell::RefCell<Scratch> = std::cell::RefCell::new(Scratch::new());
}

struct Scratch {
    mat: Vec<i32>,
    pivots: Vec<usize>,
    is_pivot: Vec<bool>,
    sum_vec: Vec<i32>,
    sum_new: Vec<i32>,
}

impl Scratch {
    fn new() -> Self {
        Scratch {
            mat: Vec::with_capacity(1 << 16),
            pivots: Vec::with_capacity(256),
            is_pivot: Vec::with_capacity(256),
            sum_vec: Vec::with_capacity(256),
            sum_new: Vec::with_capacity(256),
        }
    }
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
            let t: [[[i32; 2]; 2]; 2] = if bb == 0 {
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

    fn scalar_mul(&self, c: i32) -> Self {
        let c = modd(c);
        let mut tt = self.clone();
        for v in tt.cores[0].data.iter_mut() {
            *v = mulmod(*v, c);
        }
        tt
    }

    fn add(&self, other: &TT, coef_b: i32) -> Self {
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
                            if av == 0 {
                                continue;
                            }
                            let c_base = l * 2 * r_r + bit * r_r + ra * b_r_r;
                            for rb in 0..b_r_r {
                                let bv = b.get(lb, bit, rb);
                                if bv == 0 {
                                    continue;
                                }
                                // SAFETY: c_base + rb < r_l * 2 * r_r
                                unsafe {
                                    let p = c.data.get_unchecked_mut(c_base + rb);
                                    *p = modd(*p + mulmod(av, bv));
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

    fn apply_local(&self, mat: &[[i32; 2]; 2]) -> Self {
        let m = self.m;
        let mut cores = Vec::with_capacity(m);
        for i in 0..m {
            let s = &self.cores[i];
            let mut c = Core::with_capacity_like(s.r_l, s.r_r, s.data.len());
            for l in 0..s.r_l {
                for r in 0..s.r_r {
                    let a0 = s.get(l, 0, r);
                    let a1 = s.get(l, 1, r);
                    c.set(
                        l,
                        0,
                        r,
                        modd(mulmod(mat[0][0], a0) + mulmod(mat[0][1], a1)),
                    );
                    c.set(
                        l,
                        1,
                        r,
                        modd(mulmod(mat[1][0], a0) + mulmod(mat[1][1], a1)),
                    );
                }
            }
            cores.push(c);
        }
        TT { m, cores }
    }

    fn sum_all(&self) -> i32 {
        SCRATCH.with(|sc| {
            let mut sc = sc.borrow_mut();
            sc.sum_vec.clear();
            sc.sum_vec.push(1i32);
            for i in 0..self.m {
                let c = &self.cores[i];
                sc.sum_new.clear();
                sc.sum_new.resize(c.r_r, 0);
                for l in 0..c.r_l {
                    let vl = sc.sum_vec[l];
                    if vl == 0 {
                        continue;
                    }
                    for bit in 0..2 {
                        for r in 0..c.r_r {
                            sc.sum_new[r] = modd(sc.sum_new[r] + mulmod(vl, c.get(l, bit, r)));
                        }
                    }
                }
                let tmp = std::mem::take(&mut sc.sum_new);
                sc.sum_new = std::mem::replace(&mut sc.sum_vec, tmp);
            }
            modd(sc.sum_vec[0])
        })
    }

    fn gauss_elim(mat: &mut [i32], nrows: usize, ncols: usize, pivots: &mut Vec<usize>) {
        pivots.clear();
        let mut row_ptr = 0;
        for c in 0..ncols {
            if row_ptr >= nrows {
                break;
            }
            let mut piv = usize::MAX;
            unsafe {
                for rr in row_ptr..nrows {
                    if *mat.get_unchecked(rr * ncols + c) != 0 {
                        piv = rr;
                        break;
                    }
                }
            }
            if piv == usize::MAX {
                continue;
            }
            if piv != row_ptr {
                unsafe {
                    let pa = mat.as_mut_ptr().add(row_ptr * ncols);
                    let pb = mat.as_mut_ptr().add(piv * ncols);
                    for j in 0..ncols {
                        let tmp = *pa.add(j);
                        *pa.add(j) = *pb.add(j);
                        *pb.add(j) = tmp;
                    }
                }
            }
            let pivot_base = row_ptr * ncols;
            let inv = modinv(unsafe { *mat.get_unchecked(pivot_base + c) });
            unsafe {
                let prow = mat.as_mut_ptr().add(pivot_base);
                for j in c..ncols {
                    let p = prow.add(j);
                    *p = mulmod(*p, inv);
                }
            }
            unsafe {
                let prow = mat.as_ptr().add(pivot_base);
                for rr in 0..nrows {
                    if rr == row_ptr {
                        continue;
                    }
                    let rr_base = rr * ncols;
                    let f = *mat.get_unchecked(rr_base + c);
                    if f == 0 {
                        continue;
                    }
                    let row = mat.as_mut_ptr().add(rr_base);
                    for j in c..ncols {
                        let p = row.add(j);
                        let piv_val = *prow.add(j);
                        // f,piv_val in [0,MOD); product fits i64
                        let mut x = *p as i64 - (f as i64 * piv_val as i64 % MOD as i64);
                        if x < 0 {
                            x += MOD as i64;
                        }
                        *p = x as i32;
                    }
                }
            }
            pivots.push(c);
            row_ptr += 1;
        }
    }

    fn reduce_left(&mut self) {
        let m = self.m;
        SCRATCH.with(|sc| {
            let mut sc = sc.borrow_mut();
            for i in 0..m - 1 {
                let r_l = self.cores[i].r_l;
                let r_r = self.cores[i].r_r;
                if r_r <= 1 {
                    continue;
                }
                let nrows = 2 * r_l;
                let need = nrows * r_r;
                if sc.mat.len() < need {
                    sc.mat.resize(need, 0);
                }
                unsafe {
                    let mptr = sc.mat.as_mut_ptr();
                    let cptr = self.cores[i].data.as_ptr();
                    let rr = r_r;
                    for l in 0..r_l {
                        let src0 = cptr.add(l * 2 * rr);
                        let src1 = cptr.add(l * 2 * rr + rr);
                        let dst0 = mptr.add((2 * l) * rr);
                        let dst1 = mptr.add((2 * l + 1) * rr);
                        core::ptr::copy_nonoverlapping(src0, dst0, rr);
                        core::ptr::copy_nonoverlapping(src1, dst1, rr);
                    }
                }
                let mut pivots = std::mem::take(&mut sc.pivots);
                let mat_slice = &mut sc.mat[..need];
                Self::gauss_elim(mat_slice, nrows, r_r, &mut pivots);
                let rank = pivots.len();
                if rank == 0 || rank == r_r {
                    sc.pivots = pivots;
                    continue;
                }

                let mut new_core = Core::new(r_l, rank);
                for l in 0..r_l {
                    for (k, &p) in pivots.iter().enumerate() {
                        new_core.set(l, 0, k, self.cores[i].get(l, 0, p));
                        new_core.set(l, 1, k, self.cores[i].get(l, 1, p));
                    }
                }

                let r_next = self.cores[i + 1].r_r;
                let mut new_nxt = Core::new(rank, r_next);
                sc.is_pivot.clear();
                sc.is_pivot.resize(r_r, false);
                for &p in &pivots {
                    sc.is_pivot[p] = true;
                }

                for (k, &p) in pivots.iter().enumerate() {
                    for bit in 0..2 {
                        for t in 0..r_next {
                            new_nxt.set(k, bit, t, self.cores[i + 1].get(p, bit, t));
                        }
                    }
                }

                for j in 0..r_r {
                    if sc.is_pivot[j] {
                        continue;
                    }
                    for k in 0..rank {
                        let coeff = unsafe { *sc.mat.get_unchecked(k * r_r + j) };
                        if coeff == 0 {
                            continue;
                        }
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
                sc.pivots = pivots;

                self.cores[i] = new_core;
                self.cores[i + 1] = new_nxt;
            }
        });
    }
}

fn solve(n: usize, b: i64) -> i32 {
    let m = if b == 0 {
        1
    } else {
        (64 - b.leading_zeros()) as usize
    };

    let mut mask = TT::indicator_leq(b, m);
    mask.reduce_left();

    let mut dp = TT::indicator_leq(b, m);
    dp.reduce_left();

    let ones = TT::all_ones(m);
    let r_disjoint: [[i32; 2]; 2] = [[1, 1], [1, 0]];

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
