// Project Euler 665 - Proportionate Nim
// Losing positions with union-find "next free" structure.
// Indices fit u32 (11N entries); iterative path-halving.

struct NextFree {
    data: Vec<u32>,
}

impl NextFree {
    fn new(n: usize) -> Self {
        let mut data = vec![0u32; n];
        for i in 0..n {
            data[i] = i as u32;
        }
        NextFree { data }
    }

    #[inline(always)]
    fn get(&mut self, n: usize) -> usize {
        let len = self.data.len();
        if n >= len {
            return n;
        }
        // SAFETY: n < len; we only deref i while i < len.
        unsafe {
            let d = self.data.as_mut_ptr();
            let mut i = n;
            loop {
                let p = *d.add(i) as usize;
                if p == i {
                    return i;
                }
                if p >= len {
                    return p;
                }
                let gp = *d.add(p) as usize;
                *d.add(i) = gp as u32;
                if gp >= len {
                    return gp;
                }
                i = gp;
            }
        }
    }

    #[inline(always)]
    fn use_val(&mut self, n: usize, jump: usize) {
        let len = self.data.len();
        if n < len {
            let next = if n + jump < len {
                self.get(n + jump)
            } else {
                n + jump
            };
            // SAFETY: n < len
            unsafe {
                *self.data.get_unchecked_mut(n) = next as u32;
            }
        }
    }
}

fn main() {
    let n = 10_000_000usize;
    let mut nf1 = NextFree::new(3 * n);
    let mut nf2 = NextFree::new(2 * n);
    let mut nf3 = NextFree::new(2 * n);
    let mut nf4 = NextFree::new(4 * n);
    let mut ans = 0i64;
    for ni in 0..n {
        if nf1.get(ni) != ni {
            continue;
        }
        let mut m = ni;
        loop {
            let old_m = m;
            m = nf1.get(m);
            m = nf2.get(m - ni) + ni;
            let idx3 = m + n - 2 * ni;
            m = nf3.get(idx3) + 2 * ni - n;
            let idx4 = 2 * m - ni;
            m = (nf4.get(idx4) + ni) / 2;
            if m == old_m {
                break;
            }
        }
        if ni + m <= n {
            ans += (ni + m) as i64;
        }
        nf1.use_val(m, 1);
        nf2.use_val(m - ni, 1);
        nf3.use_val(m + n - 2 * ni, 1);
        nf3.use_val(ni + n - 2 * m, 1);
        nf4.use_val(2 * m - ni, 2);
        nf4.use_val(2 * ni - m, 2);
    }
    println!("{}", ans);
}
