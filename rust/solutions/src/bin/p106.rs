// Project Euler 106 - Special Sum Sets: Meta-testing
// For n=12, count pairs of equal-size disjoint subsets that need testing.
fn gen_combos(n: usize, k: usize) -> Vec<Vec<usize>> {
    let mut result = Vec::new();
    let mut current = vec![0usize; k];
    fn recurse(n: usize, k: usize, start: usize, depth: usize, current: &mut Vec<usize>, result: &mut Vec<Vec<usize>>) {
        if depth == k {
            result.push(current.clone());
            return;
        }
        for i in start..n {
            current[depth] = i;
            recurse(n, k, i + 1, depth + 1, current, result);
        }
    }
    recurse(n, k, 0, 0, &mut current, &mut result);
    result
}

fn needs_testing(a: &[usize], b: &[usize]) -> bool {
    let mut a_less = true;
    let mut a_greater = true;
    for i in 0..a.len() {
        if a[i] >= b[i] { a_less = false; }
        if a[i] <= b[i] { a_greater = false; }
    }
    !a_less && !a_greater
}

fn main() {
    let n = 12;
    let mut total = 0u64;

    for subset_size in 2..=n / 2 {
        let combos = gen_combos(n, subset_size);
        let masks: Vec<u32> = combos.iter().map(|c| {
            c.iter().fold(0u32, |acc, &idx| acc | (1 << idx))
        }).collect();

        for i in 0..combos.len() {
            for j in (i + 1)..combos.len() {
                if masks[i] & masks[j] != 0 { continue; }
                if needs_testing(&combos[i], &combos[j]) {
                    total += 1;
                }
            }
        }
    }

    println!("{}", total);
}
