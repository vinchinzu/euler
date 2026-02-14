// Project Euler 186 - Connectedness of a network
// Union-Find with lagged Fibonacci generator.

const MOD: i64 = 1_000_000;
const TOTAL_USERS: usize = MOD as usize;
const TARGET: usize = TOTAL_USERS * 99 / 100;
const PRIME_MINISTER: usize = 524287;

struct UnionFind {
    parent: Vec<usize>,
    sz: Vec<usize>,
}

impl UnionFind {
    fn new(n: usize) -> Self {
        UnionFind {
            parent: (0..n).collect(),
            sz: vec![1; n],
        }
    }

    fn find(&mut self, mut x: usize) -> usize {
        while self.parent[x] != x {
            self.parent[x] = self.parent[self.parent[x]];
            x = self.parent[x];
        }
        x
    }

    fn unite(&mut self, a: usize, b: usize) {
        let ra = self.find(a);
        let rb = self.find(b);
        if ra == rb {
            return;
        }
        if self.sz[ra] < self.sz[rb] {
            self.parent[ra] = rb;
            self.sz[rb] += self.sz[ra];
        } else {
            self.parent[rb] = ra;
            self.sz[ra] += self.sz[rb];
        }
    }
}

struct LFG {
    buf: [i64; 55],
    k: usize,
}

impl LFG {
    fn new() -> Self {
        let mut buf = [0i64; 55];
        for k in 1..=55usize {
            let kk = k as i64;
            let val = (100003 - 200003 * kk + 300007 * kk * kk * kk) % MOD;
            buf[k - 1] = if val < 0 { val + MOD } else { val };
        }
        LFG { buf, k: 1 }
    }

    fn next(&mut self) -> usize {
        let value = if self.k <= 55 {
            self.buf[self.k - 1]
        } else {
            let val = (self.buf[(self.k - 24 - 1) % 55] + self.buf[(self.k - 55 - 1) % 55]) % MOD;
            self.buf[(self.k - 1) % 55] = val;
            val
        };
        self.k += 1;
        value as usize
    }
}

fn main() {
    let mut uf = UnionFind::new(TOTAL_USERS);
    let mut lfg = LFG::new();

    let mut successful_calls = 0u32;

    loop {
        let caller = lfg.next();
        let called = lfg.next();
        if caller == called {
            continue;
        }

        successful_calls += 1;
        uf.unite(caller, called);

        let pm_root = uf.find(PRIME_MINISTER);
        if uf.sz[pm_root] >= TARGET {
            break;
        }
    }

    println!("{}", successful_calls);
}
