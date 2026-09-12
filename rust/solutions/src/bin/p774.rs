// Project Euler 774 - Conjunctive Sequences
// Bit-stripping recurrence (cirosantilli): D(length, bound, left, right)
// counts sequences with endpoint bit-sharing obligations. Odd bounds
// reduce by stripping the low bit (Fibonacci weights); even bounds peel
// the exceptional top value via a first-occurrence convolution.
// All lengths 0..=N for a (bound, left, right) triple are filled together.

use std::collections::HashMap;

const MOD: u32 = 998244353;
const N: usize = 123;
const TOP: i32 = -1;
const ODD: i32 = -2;

#[inline(always)]
fn madd(a: u32, b: u32) -> u32 {
    let s = a + b;
    if s >= MOD { s - MOD } else { s }
}

#[inline(always)]
fn mmul(a: u32, b: u32) -> u32 {
    ((a as u64) * (b as u64) % (MOD as u64)) as u32
}

#[inline(always)]
fn satisfies(state: i32, value: i32) -> bool {
    if state == TOP {
        true
    } else if state == ODD {
        value & 1 == 1
    } else {
        (value & state) != 0
    }
}

#[inline(always)]
fn phi_even(state: i32) -> i32 {
    if state == TOP {
        TOP
    } else if state == ODD {
        0
    } else {
        state / 2
    }
}

#[inline(always)]
fn phi_odd(state: i32) -> i32 {
    if state == TOP || state == ODD {
        TOP
    } else if state & 1 != 0 {
        TOP
    } else {
        state / 2
    }
}

struct Memo {
    map: HashMap<(i32, i32, i32), Vec<u32>>,
    fib: [u32; N + 2],
}

impl Memo {
    fn new() -> Self {
        let mut fib = [0u32; N + 2];
        fib[1] = 1;
        for i in 2..=N + 1 {
            fib[i] = madd(fib[i - 1], fib[i - 2]);
        }
        Memo {
            map: HashMap::with_capacity(512),
            fib,
        }
    }

    #[inline(always)]
    fn fib(&self, index: i32) -> u32 {
        if index >= 0 {
            self.fib[index as usize]
        } else {
            let idx = (-index) as usize;
            if idx & 1 == 1 {
                self.fib[idx]
            } else if self.fib[idx] == 0 {
                0
            } else {
                MOD - self.fib[idx]
            }
        }
    }

    fn get(&mut self, bound: i32, left: i32, right: i32) -> Vec<u32> {
        if let Some(v) = self.map.get(&(bound, left, right)) {
            return v.clone();
        }
        let v = self.compute(bound, left, right);
        self.map.insert((bound, left, right), v.clone());
        v
    }

    fn compute(&mut self, bound: i32, left: i32, right: i32) -> Vec<u32> {
        let mut arr = vec![0u32; N + 1];
        if left == 0 || right == 0 {
            return arr;
        }
        arr[0] = if left == TOP && right == TOP { 1 } else { 0 };

        if bound <= 1 {
            if bound < 0 {
                return arr;
            }
            let mut n1 = 0u32;
            for value in 0..=bound {
                if satisfies(left, value) && satisfies(right, value) {
                    n1 += 1;
                }
            }
            arr[1] = n1;
            if bound == 0 {
                return arr;
            }
            let rest = if satisfies(left, 1) && satisfies(right, 1) {
                1
            } else {
                0
            };
            for i in 2..=N {
                arr[i] = rest;
            }
            return arr;
        }

        if bound % 2 == 0 {
            self.compute_even(bound, left, right, &mut arr);
        } else {
            self.compute_odd(bound, left, right, &mut arr);
        }
        arr
    }

    fn compute_even(&mut self, bound: i32, left: i32, right: i32, arr: &mut [u32]) {
        let a = self.get(bound - 1, left, right);
        let p = self.get(bound - 1, left, bound);
        let p0 = if satisfies(left, bound) { 1u32 } else { 0 };
        let s0 = if satisfies(right, bound) { 1u32 } else { 0 };

        if left == bound {
            for l in 1..=N {
                let mut t = a[l];
                t = madd(
                    t,
                    mmul(p0, if l == 1 { s0 } else { arr[l - 1] }),
                );
                for k in 1..l {
                    let suffix = l - 1 - k;
                    let sc = if suffix == 0 { s0 } else { arr[suffix] };
                    t = madd(t, mmul(p[k], sc));
                }
                arr[l] = t;
            }
            return;
        }

        let s = self.get(bound, bound, right);
        for l in 1..=N {
            let mut t = a[l];
            t = madd(t, mmul(p0, if l == 1 { s0 } else { s[l - 1] }));
            for k in 1..l {
                let suffix = l - 1 - k;
                let sc = if suffix == 0 { s0 } else { s[suffix] };
                t = madd(t, mmul(p[k], sc));
            }
            arr[l] = t;
        }
    }

    fn compute_odd(&mut self, bound: i32, left: i32, right: i32, arr: &mut [u32]) {
        let reduced = (bound - 1) / 2;
        let le = phi_even(left);
        let lo = phi_odd(left);
        let re = phi_even(right);
        let ro = phi_odd(right);

        let ee = self.get(reduced, le, re);
        let eo = self.get(reduced, le, ro);
        let oe = self.get(reduced, lo, re);
        let oo = self.get(reduced, lo, ro);
        let ot = self.get(reduced, lo, TOP);
        let et = self.get(reduced, le, TOP);

        let mut prefix = [0u32; N + 1];
        for cut in 1..=N {
            let mut p = mmul(ot[cut], self.fib(cut as i32 - 2));
            if cut > 1 {
                p = madd(p, mmul(et[cut], self.fib(cut as i32 - 1)));
            }
            prefix[cut] = p;
        }

        let mut base = [0u32; N + 1];
        for l in 1..=N {
            let mut t = mmul(ee[l], self.fib(l as i32));
            t = madd(t, mmul(eo[l], self.fib(l as i32 - 1)));
            t = madd(t, mmul(oe[l], self.fib(l as i32 - 1)));
            t = madd(t, mmul(oo[l], self.fib(l as i32 - 2)));
            base[l] = t;
        }

        if left == ODD {
            for l in 1..=N {
                let mut t = base[l];
                for cut in 1..l {
                    t = madd(t, mmul(prefix[cut], arr[l - cut]));
                }
                arr[l] = t;
            }
            return;
        }

        let u = self.get(bound, ODD, right);
        for l in 1..=N {
            let mut t = base[l];
            for cut in 1..l {
                t = madd(t, mmul(prefix[cut], u[l - cut]));
            }
            arr[l] = t;
        }
    }
}

fn solve(n: usize, b: i32) -> u32 {
    let mut memo = Memo::new();
    let arr = memo.get(b, TOP, TOP);
    arr[n]
}

fn main() {
    println!("{}", solve(123, 123456789));
}
