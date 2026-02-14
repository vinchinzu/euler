// Project Euler 107: Minimal Network (MST via Kruskal)
use std::fs;

fn find(parent: &mut Vec<usize>, x: usize) -> usize {
    if parent[x] != x {
        parent[x] = find(parent, parent[x]);
    }
    parent[x]
}

fn union(parent: &mut Vec<usize>, rank: &mut Vec<usize>, x: usize, y: usize) -> bool {
    let px = find(parent, x);
    let py = find(parent, y);
    if px == py { return false; }
    if rank[px] < rank[py] { parent[px] = py; }
    else if rank[px] > rank[py] { parent[py] = px; }
    else { parent[py] = px; rank[px] += 1; }
    true
}

fn main() {
    let content = fs::read_to_string("data/network.txt")
        .or_else(|_| fs::read_to_string("../data/network.txt"))
        .expect("Cannot open network.txt");

    let mut edges: Vec<(i32, usize, usize)> = Vec::new();
    let mut n = 0usize;

    for (row, line) in content.lines().enumerate() {
        for (col, tok) in line.split(',').enumerate() {
            let tok = tok.trim();
            if tok == "-" { continue; }
            if let Ok(w) = tok.parse::<i32>() {
                if w > 0 && row < col {
                    edges.push((w, row, col));
                }
            }
        }
        n = row + 1;
    }

    let total_weight: i32 = edges.iter().map(|e| e.0).sum();
    edges.sort();

    let mut parent: Vec<usize> = (0..n).collect();
    let mut rank = vec![0usize; n];
    let mut mst_weight = 0i32;

    for &(w, u, v) in &edges {
        if union(&mut parent, &mut rank, u, v) {
            mst_weight += w;
        }
    }

    println!("{}", total_weight - mst_weight);
}
