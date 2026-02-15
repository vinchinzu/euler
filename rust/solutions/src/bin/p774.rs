// Project Euler 774 - Conjunctive Sequences
// Uses Tensor-Train / MPS representation with Gaussian elimination compression.

const MOD: i64 = 998244353;

fn modd(x: i64) -> i64 { ((x % MOD) + MOD) % MOD }

fn powmod(mut base: i64, mut exp: i64, m: i64) -> i64 {
    let mut r = 1i64;
    base %= m;
    while exp > 0 {
        if exp & 1 == 1 { r = (r as i128 * base as i128 % m as i128) as i64; }
        base = (base as i128 * base as i128 % m as i128) as i64;
        exp >>= 1;
    }
    r
}

fn modinv(a: i64) -> i64 { powmod(modd(a), MOD - 2, MOD) }

#[derive(Clone)]
struct Core {
    r_l: usize,
    r_r: usize,
    data: Vec<i64>, // r_l * 2 * r_r
}

impl Core {
    fn new(r_l: usize, r_r: usize) -> Self {
        Core { r_l, r_r, data: vec![0; r_l * 2 * r_r] }
    }
    fn get(&self, l: usize, bit: usize, r: usize) -> i64 {
        self.data[l * 2 * self.r_r + bit * self.r_r + r]
    }
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
    fn all_ones(m: usize) -> Self {
        let cores: Vec<Core> = (0..m).map(|_| {
            let mut c = Core::new(1, 1);
            c.set(0, 0, 0, 1); c.set(0, 1, 0, 1);
            c
        }).collect();
        TT { m, cores }
    }

    fn indicator_leq(b: i64, m: usize) -> Self {
        let bits: Vec<usize> = (0..m).map(|i| ((b >> (m - 1 - i)) & 1) as usize).collect();
        let mut cores = Vec::with_capacity(m);

        if m == 1 {
            let mut c = Core::new(1, 1);
            let bb = bits[0];
            c.set(0, 0, 0, if bb == 0 { 1 } else { 1 });
            c.set(0, 1, 0, if bb == 0 { 0 } else { 1 });
            cores.push(c);
            return TT { m, cores };
        }

        for idx in 0..m {
            let bb = bits[idx];
            // T[xbit][prev][next]: state 0=loose, 1=tight
            let t: [[[i64; 2]; 2]; 2] = if bb == 0 {
                [[[1,0],[0,1]], [[1,0],[0,0]]]
            } else {
                [[[1,0],[1,0]], [[1,0],[0,1]]]
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
        let sz = tt.cores[0].data.len();
        for j in 0..sz {
            tt.cores[0].data[j] = (tt.cores[0].data[j] as i128 * c as i128 % MOD as i128) as i64;
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
                    for r in 0..a.r_r { c.set(0, bit, r, a.get(0, bit, r)); }
                    for r in 0..b.r_r {
                        c.set(0, bit, a.r_r + r, (b.get(0, bit, r) as i128 * coef_b as i128 % MOD as i128) as i64);
                    }
                }
                cores.push(c);
            } else if i == m - 1 {
                let mut c = Core::new(a.r_l + b.r_l, 1);
                for l in 0..a.r_l { for bit in 0..2 { c.set(l, bit, 0, a.get(l, bit, 0)); } }
                for l in 0..b.r_l { for bit in 0..2 { c.set(a.r_l + l, bit, 0, b.get(l, bit, 0)); } }
                cores.push(c);
            } else {
                let mut c = Core::new(a.r_l + b.r_l, a.r_r + b.r_r);
                for l in 0..a.r_l { for bit in 0..2 { for r in 0..a.r_r { c.set(l, bit, r, a.get(l, bit, r)); } } }
                for l in 0..b.r_l { for bit in 0..2 { for r in 0..b.r_r { c.set(a.r_l + l, bit, a.r_r + r, b.get(l, bit, r)); } } }
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

            for la in 0..a.r_l {
                for lb in 0..b.r_l {
                    let l = la * b.r_l + lb;
                    for bit in 0..2 {
                        for ra in 0..a.r_r {
                            let av = a.get(la, bit, ra);
                            if av == 0 { continue; }
                            for rb in 0..b.r_r {
                                let bv = b.get(lb, bit, rb);
                                if bv == 0 { continue; }
                                let r = ra * b.r_r + rb;
                                let old = c.get(l, bit, r);
                                c.set(l, bit, r, (old as i128 + av as i128 * bv as i128 % MOD as i128) as i64 % MOD);
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
                    c.set(l, 0, r, (mat[0][0] as i128 * a0 as i128 + mat[0][1] as i128 * a1 as i128).rem_euclid(MOD as i128) as i64);
                    c.set(l, 1, r, (mat[1][0] as i128 * a0 as i128 + mat[1][1] as i128 * a1 as i128).rem_euclid(MOD as i128) as i64);
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
                if vec[l] == 0 { continue; }
                for bit in 0..2 {
                    for r in 0..c.r_r {
                        newvec[r] = (newvec[r] as i128 + vec[l] as i128 * c.get(l, bit, r) as i128 % MOD as i128) as i64 % MOD;
                    }
                }
            }
            vec = newvec;
        }
        modd(vec[0])
    }

    fn reduce_left(&mut self) {
        let m = self.m;
        for i in 0..m - 1 {
            let r_l = self.cores[i].r_l;
            let r_r = self.cores[i].r_r;
            let nrows = 2 * r_l;

            let mut mat = vec![0i64; nrows * r_r];
            for l in 0..r_l {
                for r in 0..r_r {
                    mat[(2 * l) * r_r + r] = self.cores[i].get(l, 0, r);
                    mat[(2 * l + 1) * r_r + r] = self.cores[i].get(l, 1, r);
                }
            }

            let mut pivots = Vec::new();
            let mut row_ptr = 0;

            for c in 0..r_r {
                if row_ptr >= nrows { break; }
                let mut piv = None;
                for rr in row_ptr..nrows {
                    if mat[rr * r_r + c] != 0 { piv = Some(rr); break; }
                }
                let piv = match piv { Some(p) => p, None => continue };

                if piv != row_ptr {
                    for j in 0..r_r { mat.swap(row_ptr * r_r + j, piv * r_r + j); }
                }

                let inv = modinv(mat[row_ptr * r_r + c]);
                for j in c..r_r { mat[row_ptr * r_r + j] = (mat[row_ptr * r_r + j] as i128 * inv as i128 % MOD as i128) as i64; }

                for rr in (row_ptr + 1)..nrows {
                    let f = mat[rr * r_r + c];
                    if f != 0 {
                        for j in c..r_r {
                            mat[rr * r_r + j] = modd(mat[rr * r_r + j] - (f as i128 * mat[row_ptr * r_r + j] as i128 % MOD as i128) as i64);
                        }
                    }
                }

                pivots.push(c);
                row_ptr += 1;
            }

            let rank = pivots.len();

            // Back elimination
            for k in (0..rank).rev() {
                let c = pivots[k];
                for rr in 0..k {
                    let f = mat[rr * r_r + c];
                    if f != 0 {
                        for j in c..r_r {
                            mat[rr * r_r + j] = modd(mat[rr * r_r + j] - (f as i128 * mat[k * r_r + j] as i128 % MOD as i128) as i64);
                        }
                    }
                }
            }

            if rank == r_r { continue; }

            // Build new core
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
                        let old = new_nxt.get(k, bit, t);
                        new_nxt.set(k, bit, t, modd(old + self.cores[i + 1].get(p, bit, t)));
                    }
                }
            }

            for j in 0..r_r {
                if is_pivot[j] { continue; }
                for (k, _) in pivots.iter().enumerate() {
                    let coeff = mat[k * r_r + j];
                    if coeff == 0 { continue; }
                    for bit in 0..2 {
                        for t in 0..r_next {
                            let old = new_nxt.get(k, bit, t);
                            new_nxt.set(k, bit, t, modd(old + (coeff as i128 * self.cores[i + 1].get(j, bit, t) as i128 % MOD as i128) as i64));
                        }
                    }
                }
            }

            self.cores[i] = new_core;
            self.cores[i + 1] = new_nxt;
        }
    }
}

fn main() {
    let n = 123;
    let b: i64 = 123456789;

    let m = if b == 0 { 1 } else {
        let mut bits = 1;
        let mut tmp = b;
        while tmp > 1 { bits += 1; tmp >>= 1; }
        bits
    };

    let mut mask = TT::indicator_leq(b, m);
    mask.reduce_left();

    let mut dp = TT::indicator_leq(b, m);
    dp.reduce_left();

    let ones = TT::all_ones(m);

    let r_disjoint: [[i64; 2]; 2] = [[1, 1], [1, 0]];

    for _ in 0..n - 1 {
        let total = dp.sum_all();
        let j = ones.scalar_mul(total);
        let bv = dp.apply_local(&r_disjoint);
        let nxt = j.add(&bv, MOD - 1);
        let mut masked = nxt.hadamard(&mask);
        masked.reduce_left();
        dp = masked;
    }

    println!("{}", dp.sum_all());
}
