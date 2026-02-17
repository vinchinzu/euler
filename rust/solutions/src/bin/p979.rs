// Problem 979: Heptagon Hopping
// Ported from python/979.py.

type TypeCode = u8; // 0 = I, 1 = II

fn build_layers(max_layer: usize) -> (Vec<Vec<TypeCode>>, Vec<Vec<usize>>, Vec<Vec<isize>>) {
    let mut types = vec![Vec::<TypeCode>::new(); max_layer + 1];
    let mut parent1 = vec![Vec::<usize>::new(); max_layer + 1];
    let mut parent2 = vec![Vec::<isize>::new(); max_layer + 1];

    types[0] = vec![0];
    if max_layer == 0 {
        return (types, parent1, parent2);
    }

    types[1] = vec![0; 7];
    parent1[1] = vec![0; 7];
    parent2[1] = vec![-1; 7];

    for k in 2..=max_layer {
        let prev = &types[k - 1];
        let m = prev.len();
        let mut cur: Vec<TypeCode> = Vec::new();
        let mut p1: Vec<usize> = Vec::new();
        let mut p2: Vec<isize> = Vec::new();

        for (j, &t) in prev.iter().enumerate() {
            let block: &[TypeCode] = if t == 0 { &[0, 0, 1] } else { &[0, 1] };
            let blen = block.len();
            for (pos, &ct) in block.iter().enumerate() {
                cur.push(ct);
                p1.push(j);
                if pos + 1 == blen {
                    p2.push(((j + 1) % m) as isize);
                } else {
                    p2.push(-1);
                }
            }
        }

        types[k] = cur;
        parent1[k] = p1;
        parent2[k] = p2;
    }

    (types, parent1, parent2)
}

fn add_edge(adj: &mut [Vec<usize>], u: usize, v: usize) {
    adj[u].push(v);
    adj[v].push(u);
}

fn build_ball_adjacency(max_layer: usize) -> (Vec<Vec<usize>>, usize, Vec<usize>) {
    let (types, parent1, parent2) = build_layers(max_layer);

    let mut sizes = vec![0usize; max_layer + 1];
    for k in 0..=max_layer {
        sizes[k] = types[k].len();
    }

    let mut offsets = vec![0usize; max_layer + 1];
    let mut total = 0usize;
    for k in 0..=max_layer {
        offsets[k] = total;
        total += sizes[k];
    }

    let mut adj = vec![Vec::<usize>::new(); total];
    let origin = offsets[0];

    for k in 1..=max_layer {
        let off = offsets[k];
        let m = sizes[k];
        for i in 0..m {
            add_edge(&mut adj, off + i, off + (i + 1) % m);
        }
    }

    if max_layer >= 1 {
        let off1 = offsets[1];
        for i in 0..sizes[1] {
            add_edge(&mut adj, origin, off1 + i);
        }
    }

    for k in 2..=max_layer {
        let off = offsets[k];
        let poff = offsets[k - 1];
        for i in 0..sizes[k] {
            add_edge(&mut adj, off + i, poff + parent1[k][i]);
            let p = parent2[k][i];
            if p != -1 {
                add_edge(&mut adj, off + i, poff + p as usize);
            }
        }
    }

    for k in 0..max_layer {
        let off = offsets[k];
        for i in 0..sizes[k] {
            let u = off + i;
            assert_eq!(adj[u].len(), 7);
        }
    }

    (adj, origin, offsets)
}

fn f(n: usize) -> u128 {
    if n == 0 {
        return 1;
    }

    let max_layer = n / 2;
    let (adj, origin, _) = build_ball_adjacency(max_layer);

    let node_count = adj.len();
    let mut dp = vec![0u128; node_count];
    dp[origin] = 1;

    for _ in 0..n {
        let mut ndp = vec![0u128; node_count];
        for (u, &val) in dp.iter().enumerate() {
            if val == 0 {
                continue;
            }
            for &v in &adj[u] {
                ndp[v] += val;
            }
        }
        dp = ndp;
    }

    dp[origin]
}

fn main() {
    println!("{}", f(20));
}
