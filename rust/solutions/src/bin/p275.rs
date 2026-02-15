use std::collections::HashMap;
use rayon::prelude::*;

const N: usize = 18;
const DXS: [i32; 4] = [1, -1, 0, 0];
const DYS: [i32; 4] = [0, 0, 1, -1];

struct Sculpture {
    cand_x: Vec<i32>,
    cand_y: Vec<i32>,
    invalid: Vec<Vec<bool>>,
    column_counts: Vec<i64>,
}

impl Sculpture {
    fn new() -> Self {
        Sculpture {
            cand_x: vec![0; 4 * N],
            cand_y: vec![0; 4 * N],
            invalid: vec![vec![false; N + 2]; 2 * N + 1],
            column_counts: vec![0; 2 * N + 1],
        }
    }

    fn num_sculptures(&mut self, start: usize, end: usize, num_tiles: i64) -> i64 {
        if num_tiles == 0 { return 1; }
        let mut res = 0i64;
        for i in start..end {
            let cx = self.cand_x[i];
            let cy = self.cand_y[i];
            let ci = (cx + N as i32) as usize;
            if self.column_counts[ci] == 0 { continue; }
            self.column_counts[ci] -= 1;
            let mut new_end = end;
            for d in 0..4 {
                let nx = cx + DXS[d];
                let ny = cy + DYS[d];
                let ni = (nx + N as i32) as usize;
                if ny > 0 && !self.invalid[ni][ny as usize] {
                    self.cand_x[new_end] = nx;
                    self.cand_y[new_end] = ny;
                    new_end += 1;
                    self.invalid[ni][ny as usize] = true;
                }
            }
            res += self.num_sculptures(i + 1, new_end, num_tiles - 1);
            self.column_counts[ci] += 1;
            for j in end..new_end {
                let ni = (self.cand_x[j] + N as i32) as usize;
                self.invalid[ni][self.cand_y[j] as usize] = false;
            }
        }
        res
    }
}

fn find_column_counts(num_tiles: usize, current: &mut Vec<usize>, results: &mut Vec<Vec<usize>>) {
    if num_tiles > 0 {
        results.push(current.clone());
    }
    for i in 1..=(N - num_tiles) {
        current.push(i);
        find_column_counts(num_tiles + i, current, results);
        current.pop();
    }
}

fn cc_sum(cc: &[usize]) -> usize {
    cc.iter().sum()
}

fn cc_moment(cc: &[usize]) -> i64 {
    cc.iter().enumerate().map(|(i, &v)| i as i64 * v as i64).sum()
}

fn main() {
    let mut current = Vec::new();
    let mut all_cc = Vec::new();
    find_column_counts(0, &mut current, &mut all_cc);

    // Group by (c0, moment)
    let mut groups: HashMap<(usize, i64), Vec<usize>> = HashMap::new();
    for (i, cc) in all_cc.iter().enumerate() {
        let key = (cc[0], cc_moment(cc));
        groups.entry(key).or_default().push(i);
    }

    let group_list: Vec<_> = groups.values().collect();

    let ans: i64 = group_list.par_iter()
        .map(|group| {
            let num_middle = all_cc[group[0]][0];
            let mut local_ans: i64 = 0;
            let mut sculpt = Sculpture::new();

            for (ii, &i1) in group.iter().enumerate() {
                let size1 = cc_sum(&all_cc[i1]);
                let size2 = N - size1 + num_middle;

                for &i2 in &group[ii..] {
                    if cc_sum(&all_cc[i2]) != size2 { continue; }

                    sculpt.cand_x[0] = 0;
                    sculpt.cand_y[0] = 1;
                    for row in sculpt.invalid.iter_mut() {
                        for v in row.iter_mut() { *v = false; }
                    }
                    sculpt.invalid[N][1] = true;
                    for v in sculpt.column_counts.iter_mut() { *v = 0; }

                    for (i, &v) in all_cc[i1].iter().enumerate() {
                        sculpt.column_counts[i + N] = v as i64;
                    }

                    let reversible = i1 == i2;

                    if reversible {
                        local_ans += sculpt.num_sculptures(0, 1, size1 as i64);
                    }

                    for (i, &v) in all_cc[i2].iter().enumerate().skip(1) {
                        sculpt.column_counts[N - i] = v as i64;
                    }

                    let count = sculpt.num_sculptures(0, 1, N as i64);

                    if reversible {
                        local_ans += count;
                    } else {
                        local_ans += 2 * count;
                    }
                }
            }
            local_ans
        })
        .sum();

    let ans = ans / 2;
    println!("{}", ans);
}
