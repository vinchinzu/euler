// Project Euler 215: Crack-free Walls

const W: usize = 32;
const H: usize = 10;

fn gen_rows(x: usize, bits: u32, rows: &mut Vec<u32>) {
    if x == W {
        rows.push(bits);
        return;
    }
    if x + 2 <= W {
        if x + 2 == W {
            gen_rows(W, bits, rows);
        } else {
            gen_rows(x + 2, bits | (1 << (x + 2)), rows);
        }
    }
    if x + 3 <= W {
        if x + 3 == W {
            gen_rows(W, bits, rows);
        } else {
            gen_rows(x + 3, bits | (1 << (x + 3)), rows);
        }
    }
}

fn main() {
    let mut row_bitsets = Vec::new();
    gen_rows(0, 0, &mut row_bitsets);
    let n = row_bitsets.len();

    // Build adjacency lists
    let mut adj: Vec<Vec<usize>> = vec![Vec::new(); n];
    for i in 0..n {
        for j in 0..n {
            if row_bitsets[i] & row_bitsets[j] == 0 {
                adj[i].push(j);
            }
        }
    }

    let mut ways = vec![1i64; n];

    for _ in 2..=H {
        let mut new_ways = vec![0i64; n];
        for i in 0..n {
            if ways[i] == 0 { continue; }
            for &j in &adj[i] {
                new_ways[j] += ways[i];
            }
        }
        ways = new_ways;
    }

    let ans: i64 = ways.iter().sum();
    println!("{}", ans);
}
