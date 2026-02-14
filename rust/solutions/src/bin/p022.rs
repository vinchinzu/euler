// Project Euler 022: Names Scores
// Sort names, compute total of (position * alphabetical value).

fn main() {
    let data = include_str!("../../../../data/names.txt");

    let mut names: Vec<&str> = data
        .split(',')
        .map(|s| s.trim_matches('"'))
        .collect();
    names.sort();

    let total: u64 = names
        .iter()
        .enumerate()
        .map(|(i, name)| {
            let score: u64 = name.bytes().map(|b| (b - b'A' + 1) as u64).sum();
            score * (i as u64 + 1)
        })
        .sum();

    println!("{total}");
}
