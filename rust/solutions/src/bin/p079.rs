// Project Euler 79: Passcode derivation
// Topological sort of digit ordering constraints from keylog data.

fn main() {
    let data = include_str!("../../../../data/0079_keylog.txt");

    let mut exists = [false; 10];
    let mut adj = [[false; 10]; 10];
    let mut in_degree = [0u32; 10];

    for line in data.lines() {
        let line = line.trim();
        if line.len() < 3 { continue; }
        let digits: Vec<usize> = line.chars().map(|c| c.to_digit(10).unwrap() as usize).collect();
        let (d1, d2, d3) = (digits[0], digits[1], digits[2]);

        exists[d1] = true;
        exists[d2] = true;
        exists[d3] = true;

        for &(a, b) in &[(d1, d2), (d1, d3), (d2, d3)] {
            if !adj[a][b] {
                adj[a][b] = true;
                in_degree[b] += 1;
            }
        }
    }

    // Kahn's topological sort
    let mut queue: Vec<usize> = (0..10).filter(|&i| exists[i] && in_degree[i] == 0).collect();
    let mut passcode = String::new();

    while !queue.is_empty() {
        // Pick smallest for deterministic order
        queue.sort();
        let u = queue.remove(0);
        passcode.push(char::from_digit(u as u32, 10).unwrap());

        for v in 0..10 {
            if adj[u][v] {
                in_degree[v] -= 1;
                if in_degree[v] == 0 {
                    queue.push(v);
                }
            }
        }
    }

    println!("{passcode}");
}
