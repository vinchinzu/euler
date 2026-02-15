// Project Euler 665 - Proportionate Nim
// Losing positions with union-find "next free" structure.

struct NextFree { data: Vec<usize> }

impl NextFree {
    fn new(n: usize) -> Self { NextFree { data: (0..n).collect() } }
    fn get(&mut self, n: usize) -> usize {
        if n >= self.data.len() { return n; }
        if self.data[n] == n { return n; }
        self.data[n] = self.get(self.data[n]);
        self.data[n]
    }
    fn use_val(&mut self, n: usize, jump: usize) {
        if n < self.data.len() {
            let next = if n + jump < self.data.len() { self.get(n + jump) } else { n + jump };
            self.data[n] = next;
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
        if nf1.get(ni) != ni { continue; }
        let mut m = ni;
        loop {
            let old_m = m;
            m = nf1.get(m);
            m = nf2.get(m - ni) + ni;
            let idx3 = m + n - 2 * ni;
            m = nf3.get(idx3) + 2 * ni - n;
            let idx4 = 2 * m - ni;
            m = (nf4.get(idx4) + ni) / 2;
            if m == old_m { break; }
        }
        if ni + m <= n { ans += (ni + m) as i64; }
        nf1.use_val(m, 1);
        nf2.use_val(m - ni, 1);
        nf3.use_val(m + n - 2 * ni, 1);
        nf3.use_val(ni + n - 2 * m, 1);
        nf4.use_val(2 * m - ni, 2);
        nf4.use_val(2 * ni - m, 2);
    }
    println!("{}", ans);
}
