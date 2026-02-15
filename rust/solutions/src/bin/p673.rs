// Project Euler 673 - Beds and Desks
// Read bed/desk pairings from files, find connected components.
// NOTE: Requires data files python/0673_beds.txt and python/0673_desks.txt

const N_STUDENTS: usize = 500;
const MOD: i64 = 999_999_937;

fn mod_pow(mut base: i64, mut exp: i64, m: i64) -> i64 {
    let mut r = 1i64; base %= m;
    while exp > 0 {
        if exp & 1 == 1 { r = (r as i128 * base as i128 % m as i128) as i64; }
        base = (base as i128 * base as i128 % m as i128) as i64;
        exp >>= 1;
    }
    r
}

fn factorial_mod(n: usize, m: i64) -> i64 {
    let mut r = 1i64;
    for i in 1..=n { r = r * i as i64 % m; }
    r
}

fn read_graph(path: &str) -> Vec<i32> {
    let mut graph = vec![-1i32; N_STUDENTS];
    if let Ok(content) = std::fs::read_to_string(path) {
        for line in content.lines() {
            let parts: Vec<&str> = line.trim().split(',').collect();
            if parts.len() == 2 {
                if let (Ok(v1), Ok(v2)) = (parts[0].parse::<usize>(), parts[1].parse::<usize>()) {
                    graph[v1 - 1] = (v2 - 1) as i32;
                    graph[v2 - 1] = (v1 - 1) as i32;
                }
            }
        }
    }
    graph
}

fn main() {
    let paths = ["python/0673_beds.txt", "../python/0673_beds.txt"];
    let beds_path = paths.iter().find(|p| std::path::Path::new(p).exists()).unwrap_or(&paths[0]);
    let desks_path = beds_path.replace("beds", "desks");
    let beds = read_graph(beds_path);
    let desks = read_graph(&desks_path);

    let mut visited = vec![false; N_STUDENTS];
    let mut components: Vec<(usize, usize, usize)> = Vec::new();

    for i in 0..N_STUDENTS {
        if visited[i] { continue; }
        let (mut ns, mut nb, mut nd) = (0, 0, 0);
        let mut stack = vec![i];
        while let Some(v) = stack.pop() {
            if visited[v] { continue; }
            visited[v] = true; ns += 1;
            if beds[v] >= 0 { nb += 1; if !visited[beds[v] as usize] { stack.push(beds[v] as usize); } }
            if desks[v] >= 0 { nd += 1; if !visited[desks[v] as usize] { stack.push(desks[v] as usize); } }
        }
        components.push((ns, nb / 2, nd / 2));
    }

    let mut used = vec![false; components.len()];
    let mut ans = 1i64;
    for i in 0..components.len() {
        if used[i] { continue; }
        let mut count = 0;
        for j in i..components.len() {
            if !used[j] && components[j] == components[i] { count += 1; used[j] = true; }
        }
        let (ns, nb, nd) = components[i];
        if nb + nd == ns { ans = (ans as i128 * mod_pow(ns as i64, count as i64, MOD) as i128 % MOD as i128) as i64; }
        else if ns % 2 == 0 { ans = (ans as i128 * mod_pow(2, count as i64, MOD) as i128 % MOD as i128) as i64; }
        ans = (ans as i128 * factorial_mod(count, MOD) as i128 % MOD as i128) as i64;
    }
    println!("{}", ans);
}
