// Project Euler 324: Building a tower
// Find f(10^10000) mod 100000007, where f(n) = ways to fill 3x3xn tower with 2x1x1 blocks.
// Uses transition matrix with D4 symmetry reduction, then decimal-exponent matrix exponentiation.

const Q: i64 = 100_000_007;
const K: usize = 3;
const KK: usize = 9;
const FULL: usize = 511; // (1<<9)-1

fn rotate(face: usize) -> usize {
    let mut new = 0;
    for i in 0..K {
        for j in 0..K {
            if face & (1 << (K * i + j)) != 0 {
                new |= 1 << (K * j + (K - 1 - i));
            }
        }
    }
    new
}

fn flip(face: usize) -> usize {
    let mut new = 0;
    for i in 0..K {
        for j in 0..K {
            if face & (1 << (K * i + j)) != 0 {
                new |= 1 << (K * i + (K - 1 - j));
            }
        }
    }
    new
}

fn face_min(face: usize) -> usize {
    let mut mn = face;
    let mut f = face;
    for _ in 0..4 {
        if f < mn { mn = f; }
        let fl = flip(f);
        if fl < mn { mn = fl; }
        f = rotate(f);
    }
    mn
}

fn fill_layer(mask_cur: usize, mask_out: usize, results: &mut Vec<usize>) {
    let inv = (!mask_cur) & FULL;
    if inv == 0 {
        results.push(mask_out);
        return;
    }
    let pos = inv.trailing_zeros() as usize;
    // Place block vertically (into next layer)
    fill_layer(mask_cur | (1 << pos), mask_out | (1 << pos), results);

    let row = pos / K;
    let col = pos % K;
    // Place block horizontally within row
    let nei = pos + 1;
    if col + 1 < K && (mask_cur & (1 << nei)) == 0 {
        fill_layer(mask_cur | (1 << pos) | (1 << nei), mask_out, results);
    }
    // Place block vertically within layer
    let nei = pos + K;
    if row + 1 < K && (mask_cur & (1 << nei)) == 0 {
        fill_layer(mask_cur | (1 << pos) | (1 << nei), mask_out, results);
    }
}

#[derive(Clone)]
struct Matrix {
    n: usize,
    data: Vec<i64>,
}

impl Matrix {
    fn new(n: usize) -> Self {
        Matrix { n, data: vec![0; n * n] }
    }

    fn eye(n: usize) -> Self {
        let mut m = Self::new(n);
        for i in 0..n { m.data[i * n + i] = 1; }
        m
    }

    fn get(&self, i: usize, j: usize) -> i64 { self.data[i * self.n + j] }
    fn set(&mut self, i: usize, j: usize, v: i64) { self.data[i * self.n + j] = v; }

    fn mul(&self, other: &Matrix) -> Matrix {
        let n = self.n;
        let mut c = Matrix::new(n);
        for i in 0..n {
            for k in 0..n {
                let a = self.get(i, k);
                if a == 0 { continue; }
                for j in 0..n {
                    let v = ((c.get(i, j) as i128 + a as i128 * other.get(k, j) as i128) % Q as i128) as i64;
                    c.set(i, j, v);
                }
            }
        }
        c
    }

    fn pow_decimal(&self, digits: &[u8]) -> Matrix {
        let n = self.n;
        let mut result = Matrix::eye(n);

        for &d in digits {
            // result = result^10
            let r2 = result.mul(&result);
            let r4 = r2.mul(&r2);
            let r5 = r4.mul(&result);
            let r10 = r5.mul(&r5);
            result = r10;

            // Multiply by M^digit
            let digit = d;
            if digit > 0 {
                let mut base = self.clone();
                let mut md = Matrix::eye(n);
                let mut e = digit;
                while e > 0 {
                    if e & 1 == 1 {
                        md = md.mul(&base);
                    }
                    e >>= 1;
                    if e > 0 {
                        base = base.mul(&base);
                    }
                }
                result = result.mul(&md);
            }
        }
        result
    }
}

fn main() {
    // Precompute raw transitions
    let mut raw_results: Vec<Vec<usize>> = vec![Vec::new(); 1 << KK];
    for mask in 0..(1 << KK) {
        let mut results = Vec::new();
        fill_layer(mask, 0, &mut results);
        raw_results[mask] = results;
    }

    // Build symmetry classes via BFS from face 0
    let mut face_index = vec![-1i32; 512];
    let mut face_rep = Vec::new();
    let mut nfaces = 0;
    let mut queue = std::collections::VecDeque::new();

    let rep0 = face_min(0);
    face_rep.push(rep0);
    face_index[rep0] = 0;
    nfaces = 1;
    queue.push_back(rep0);

    let mut trans = vec![vec![0i32; 100]; 100];

    while let Some(rep) = queue.pop_front() {
        let fi = face_index[rep] as usize;
        for &ns in &raw_results[rep] {
            let nrep = face_min(ns);
            if face_index[nrep] == -1 {
                face_rep.push(nrep);
                face_index[nrep] = nfaces as i32;
                nfaces += 1;
                queue.push_back(nrep);
            }
            trans[face_index[nrep] as usize][fi] += 1;
        }
    }

    // Build transition matrix
    let mut a = Matrix::new(nfaces);
    for i in 0..nfaces {
        for j in 0..nfaces {
            a.set(i, j, trans[i][j] as i64 % Q);
        }
    }

    // Squaring trick: keep only even-popcount faces
    let mut even_faces = Vec::new();
    for i in 0..nfaces {
        if face_rep[i].count_ones() % 2 == 0 {
            even_faces.push(i);
        }
    }
    let ne = even_faces.len();

    // A^2 restricted to even-popcount faces
    let mut a2 = Matrix::new(ne);
    for i in 0..ne {
        for j in 0..ne {
            let mut sum = 0i64;
            for k in 0..nfaces {
                sum = ((sum as i128 + a.get(even_faces[i], k) as i128 * a.get(k, even_faces[j]) as i128) % Q as i128) as i64;
            }
            a2.set(i, j, sum);
        }
    }

    // Find index of face 0 in even_faces
    let fi0 = face_index[face_min(0)] as usize;
    let ei = even_faces.iter().position(|&x| x == fi0).unwrap();

    // Exponent = 10^10000 / 2 = 5 * 10^9999 = "5" followed by 9999 zeros
    let mut exp_digits = Vec::with_capacity(10000);
    exp_digits.push(5u8);
    for _ in 0..9999 { exp_digits.push(0u8); }

    let powered = a2.pow_decimal(&exp_digits);
    println!("{}", powered.get(ei, ei));
}
