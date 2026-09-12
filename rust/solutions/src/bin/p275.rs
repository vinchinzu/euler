use rayon::prelude::*;

const N: usize = 18;
const DXS: [i32; 4] = [1, -1, 0, 0];
const DYS: [i32; 4] = [0, 0, 1, -1];
const MAX_MOM: usize = N * (N - 1) / 2; // 153
const MOM_STRIDE: usize = MAX_MOM + 1;

struct Sculpture {
    cand_x: [i32; 4 * N],
    cand_y: [i32; 4 * N],
    invalid: [u32; 2 * N + 1],
    column_counts: [i64; 2 * N + 1],
}

impl Sculpture {
    fn new() -> Self {
        Sculpture {
            cand_x: [0; 4 * N],
            cand_y: [0; 4 * N],
            invalid: [0; 2 * N + 1],
            column_counts: [0; 2 * N + 1],
        }
    }

    fn reset(&mut self) {
        self.invalid = [0; 2 * N + 1];
        self.column_counts = [0; 2 * N + 1];
        self.cand_x[0] = 0;
        self.cand_y[0] = 1;
        self.invalid[N] = 1u32 << 1;
    }

    fn num_sculptures(&mut self, start: usize, end: usize, num_tiles: i64) -> i64 {
        if num_tiles == 0 {
            return 1;
        }
        let mut res = 0i64;
        for i in start..end {
            let cx = self.cand_x[i];
            let cy = self.cand_y[i];
            let ci = (cx + N as i32) as usize;
            if self.column_counts[ci] == 0 {
                continue;
            }
            self.column_counts[ci] -= 1;
            let mut new_end = end;
            for d in 0..4 {
                let nx = cx + DXS[d];
                let ny = cy + DYS[d];
                let ni = (nx + N as i32) as usize;
                if ny > 0 && (self.invalid[ni] & (1u32 << (ny as u32))) == 0 {
                    self.cand_x[new_end] = nx;
                    self.cand_y[new_end] = ny;
                    new_end += 1;
                    self.invalid[ni] |= 1u32 << (ny as u32);
                }
            }
            res += self.num_sculptures(i + 1, new_end, num_tiles - 1);
            self.column_counts[ci] += 1;
            for j in end..new_end {
                let ni = (self.cand_x[j] + N as i32) as usize;
                self.invalid[ni] &= !(1u32 << (self.cand_y[j] as u32));
            }
        }
        res
    }
}

#[derive(Clone, Copy)]
struct CC {
    len: u8,
    sum: u8,
    c0: u8,
    moment: u16,
    data: [u8; N],
}

fn find_column_counts(num_tiles: usize, current: &mut [u8; N], cur_len: usize, results: &mut Vec<CC>) {
    if num_tiles > 0 {
        let mut data = [0u8; N];
        data[..cur_len].copy_from_slice(&current[..cur_len]);
        let mut sum = 0u8;
        let mut moment = 0u16;
        for i in 0..cur_len {
            let v = current[i];
            sum += v;
            moment += i as u16 * v as u16;
        }
        results.push(CC {
            len: cur_len as u8,
            sum,
            c0: current[0],
            moment,
            data,
        });
    }
    for i in 1..=(N - num_tiles) {
        current[cur_len] = i as u8;
        find_column_counts(num_tiles + i, current, cur_len + 1, results);
    }
}

fn count_group(group: &[usize], all_cc: &[CC]) -> i64 {
    let num_middle = all_cc[group[0]].c0 as usize;
    let mut local_ans: i64 = 0;
    let mut sculpt = Sculpture::new();

    for (ii, &i1) in group.iter().enumerate() {
        let size1 = all_cc[i1].sum as usize;
        let size2 = N - size1 + num_middle;

        for &i2 in &group[ii..] {
            if all_cc[i2].sum as usize != size2 {
                continue;
            }

            sculpt.reset();

            let cc1 = &all_cc[i1];
            for i in 0..cc1.len as usize {
                sculpt.column_counts[i + N] = cc1.data[i] as i64;
            }

            let reversible = i1 == i2;

            if reversible {
                local_ans += sculpt.num_sculptures(0, 1, size1 as i64);
            }

            let cc2 = &all_cc[i2];
            for i in 1..cc2.len as usize {
                sculpt.column_counts[N - i] = cc2.data[i] as i64;
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
}

fn main() {
    let mut current = [0u8; N];
    let mut all_cc = Vec::new();
    find_column_counts(0, &mut current, 0, &mut all_cc);

    let mut groups: Vec<Vec<usize>> = vec![Vec::new(); (N + 1) * MOM_STRIDE];
    for (i, cc) in all_cc.iter().enumerate() {
        let key = cc.c0 as usize * MOM_STRIDE + cc.moment as usize;
        groups[key].push(i);
    }
    let group_list: Vec<Vec<usize>> = groups.into_iter().filter(|g| !g.is_empty()).collect();

    let ans: i64 = group_list
        .par_iter()
        .map(|group| {
            // Split large groups across threads so one bucket cannot dominate.
            if group.len() > 64 {
                (0..group.len())
                    .into_par_iter()
                    .map(|ii| {
                        let i1 = group[ii];
                        let num_middle = all_cc[group[0]].c0 as usize;
                        let size1 = all_cc[i1].sum as usize;
                        let size2 = N - size1 + num_middle;
                        let mut local_ans: i64 = 0;
                        let mut sculpt = Sculpture::new();
                        for &i2 in &group[ii..] {
                            if all_cc[i2].sum as usize != size2 {
                                continue;
                            }
                            sculpt.reset();
                            let cc1 = &all_cc[i1];
                            for i in 0..cc1.len as usize {
                                sculpt.column_counts[i + N] = cc1.data[i] as i64;
                            }
                            let reversible = i1 == i2;
                            if reversible {
                                local_ans += sculpt.num_sculptures(0, 1, size1 as i64);
                            }
                            let cc2 = &all_cc[i2];
                            for i in 1..cc2.len as usize {
                                sculpt.column_counts[N - i] = cc2.data[i] as i64;
                            }
                            let count = sculpt.num_sculptures(0, 1, N as i64);
                            if reversible {
                                local_ans += count;
                            } else {
                                local_ans += 2 * count;
                            }
                        }
                        local_ans
                    })
                    .sum()
            } else {
                count_group(group, &all_cc)
            }
        })
        .sum();

    let ans = ans / 2;
    println!("{}", ans);
}
