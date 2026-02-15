// Project Euler 419: Look and say sequence
// Conway's Cosmological Theorem: every look-and-say sequence eventually decays
// into 92 independent "atoms". Build transition matrix and use matrix exponentiation.

use std::collections::HashMap;

const MOD: i64 = 1 << 30; // 2^30
const MAX_STR: usize = 4096;

fn look_and_say(s: &str) -> String {
    let bytes = s.as_bytes();
    let mut out = String::new();
    let mut i = 0;
    while i < bytes.len() {
        let ch = bytes[i];
        let mut j = i + 1;
        while j < bytes.len() && bytes[j] == ch {
            j += 1;
        }
        out.push_str(&(j - i).to_string());
        out.push(ch as char);
        i = j;
    }
    out
}

fn can_split_at(s: &str, pos: usize, l: usize) -> bool {
    if pos >= s.len() || pos == 0 {
        return true;
    }
    let c = s.as_bytes()[pos - 1];
    let mut buf = s[pos..].to_string();
    for _ in 0..l {
        if buf.is_empty() {
            return true;
        }
        if buf.as_bytes()[0] == c {
            return false;
        }
        buf = look_and_say(&buf);
        if buf.len() > MAX_STR - 2 {
            break;
        }
    }
    true
}

struct AtomSystem {
    atoms: Vec<String>,
    atom_map: HashMap<String, usize>,
    trans: Vec<Vec<i32>>,
    visited: Vec<bool>,
}

impl AtomSystem {
    fn new() -> Self {
        AtomSystem {
            atoms: Vec::new(),
            atom_map: HashMap::new(),
            trans: Vec::new(),
            visited: Vec::new(),
        }
    }

    fn find_or_add(&mut self, s: &str) -> usize {
        if let Some(&idx) = self.atom_map.get(s) {
            return idx;
        }
        let idx = self.atoms.len();
        self.atoms.push(s.to_string());
        self.atom_map.insert(s.to_string(), idx);
        self.trans.push(vec![]);
        self.visited.push(false);
        idx
    }

    fn find_transitions(&mut self, s: &str, l: usize) {
        let idx = self.find_or_add(s);
        if self.visited[idx] {
            return;
        }
        self.visited[idx] = true;

        let next = look_and_say(s);
        let nlen = next.len();

        // Split next into atoms
        let mut parts: Vec<String> = Vec::new();
        let mut last_split = 0;
        for i in 1..=nlen {
            if i == nlen || can_split_at(&next, i, l) {
                parts.push(next[last_split..i].to_string());
                last_split = i;
            }
        }

        // Register all part atoms first
        let part_indices: Vec<usize> = parts.iter().map(|p| self.find_or_add(p)).collect();

        // Set up transitions for idx
        let size = self.atoms.len();
        if self.trans[idx].len() < size {
            self.trans[idx].resize(size, 0);
        }
        for &pidx in &part_indices {
            if pidx >= self.trans[idx].len() {
                self.trans[idx].resize(pidx + 1, 0);
            }
            self.trans[idx][pidx] += 1;
        }

        // Recurse for unvisited parts
        for part in parts {
            let pidx = *self.atom_map.get(&part).unwrap();
            if !self.visited[pidx] {
                self.find_transitions(&part, l);
            }
        }
    }
}

// Matrix operations
fn mat_mul(a: &[i64], b: &[i64], n: usize) -> Vec<i64> {
    let mut res = vec![0i64; n * n];
    for i in 0..n {
        for k in 0..n {
            let aik = a[i * n + k];
            if aik == 0 {
                continue;
            }
            for j in 0..n {
                res[i * n + j] = (res[i * n + j] + aik * b[k * n + j]) % MOD;
            }
        }
    }
    res
}

fn mat_pow(base: &[i64], n: usize, mut exp: i64) -> Vec<i64> {
    let mut result = vec![0i64; n * n];
    for i in 0..n {
        result[i * n + i] = 1;
    }
    let mut b = base.to_vec();
    while exp > 0 {
        if exp & 1 == 1 {
            result = mat_mul(&result, &b, n);
        }
        b = mat_mul(&b, &b, n);
        exp >>= 1;
    }
    result
}

fn main() {
    let big_n: i64 = 1_000_000_000_000;
    let l = 10;

    let mut sys = AtomSystem::new();
    sys.find_transitions("1", l);

    let size = sys.atoms.len();

    // Count digits in each atom
    let mut digit_count = vec![[0i64; 4]; size]; // indices 1,2,3
    for i in 0..size {
        for b in sys.atoms[i].bytes() {
            let d = (b - b'0') as usize;
            if d >= 1 && d <= 3 {
                digit_count[i][d] += 1;
            }
        }
    }

    // Build transition matrix A where A[to][from] = trans[from][to]
    let mut a_mat = vec![0i64; size * size];
    for from in 0..size {
        let tlen = sys.trans[from].len();
        for to in 0..tlen.min(size) {
            a_mat[to * size + from] = sys.trans[from][to] as i64;
        }
    }

    // Compute A^(N-1)
    let r = mat_pow(&a_mat, size, big_n - 1);

    // Initial vector: "1" is the first atom added (index 0)
    let start_idx = *sys.atom_map.get("1").unwrap();

    // Count digits
    let mut count1: i64 = 0;
    let mut count2: i64 = 0;
    let mut count3: i64 = 0;
    for i in 0..size {
        let cnt = r[i * size + start_idx];
        count1 = (count1 + cnt * digit_count[i][1]) % MOD;
        count2 = (count2 + cnt * digit_count[i][2]) % MOD;
        count3 = (count3 + cnt * digit_count[i][3]) % MOD;
    }

    println!("{},{},{}", count1, count2, count3);
}
