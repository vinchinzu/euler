// Problem 982: The Third Dice
// Port of the LP + simplex reference implementation.

use std::collections::{BTreeSet, HashMap, HashSet};

const EPS: f64 = 1e-9;
const MAX_ITER: usize = 200_000;

#[derive(Clone, Copy)]
enum Sense {
    Le,
    Ge,
    Eq,
}

fn pivot(tableau: &mut [Vec<f64>], basis: &mut [usize], row: usize, col: usize) {
    let pivot_val = tableau[row][col];
    let inv = 1.0 / pivot_val;
    let n_cols = tableau[0].len();

    for j in 0..n_cols {
        tableau[row][j] *= inv;
    }

    for i in 0..tableau.len() {
        if i == row {
            continue;
        }
        let factor = tableau[i][col];
        if factor.abs() > EPS {
            for j in 0..n_cols {
                tableau[i][j] -= factor * tableau[row][j];
            }
        }
    }

    basis[row] = col;
}

fn set_objective(tableau: &mut Vec<Vec<f64>>, basis: &[usize], c: &[f64]) {
    let m = basis.len();
    let n = tableau[0].len() - 1;

    let mut obj = vec![0.0; n + 1];
    for j in 0..n {
        obj[j] = -c[j];
    }

    for i in 0..m {
        let cb = c[basis[i]];
        if cb.abs() > EPS {
            for (j, obj_j) in obj.iter_mut().enumerate().take(n + 1) {
                *obj_j += cb * tableau[i][j];
            }
        }
    }

    if tableau.len() == m {
        tableau.push(obj);
    } else {
        tableau[m] = obj;
    }
}

fn simplex_max(tableau: &mut Vec<Vec<f64>>, basis: &mut [usize]) -> bool {
    let m = basis.len();
    let n = tableau[0].len() - 1;

    for _ in 0..MAX_ITER {
        let mut entering = None;
        for j in 0..n {
            if tableau[m][j] < -EPS {
                entering = Some(j);
                break;
            }
        }

        let Some(entering_col) = entering else {
            return true;
        };

        let mut min_ratio = f64::INFINITY;
        let mut leaving = None;
        for (i, row) in tableau.iter().take(m).enumerate() {
            let a = row[entering_col];
            if a > EPS {
                let ratio = row[n] / a;
                if ratio < min_ratio - EPS {
                    min_ratio = ratio;
                    leaving = Some(i);
                }
            }
        }

        let Some(leaving_row) = leaving else {
            return false;
        };

        pivot(tableau, basis, leaving_row, entering_col);
    }

    panic!("simplex did not converge");
}

fn build_tableau(
    n_vars: usize,
    constraints: &[(Vec<f64>, Sense, f64)],
) -> (Vec<Vec<f64>>, Vec<usize>, Vec<usize>, usize) {
    let mut rows: Vec<Vec<f64>> = Vec::with_capacity(constraints.len());
    let mut rhs: Vec<f64> = Vec::with_capacity(constraints.len());
    let mut basis: Vec<usize> = Vec::with_capacity(constraints.len());
    let mut art_indices: Vec<usize> = Vec::new();
    let mut n_total = n_vars;

    for (coeffs_in, sense_in, b_in) in constraints {
        let mut coeffs = coeffs_in.clone();
        let mut b = *b_in;
        let mut sense = *sense_in;

        if b < 0.0 {
            for v in &mut coeffs {
                *v = -*v;
            }
            b = -b;
            sense = match sense {
                Sense::Le => Sense::Ge,
                Sense::Ge => Sense::Le,
                Sense::Eq => Sense::Eq,
            };
        }

        let mut row = coeffs;
        row.resize(n_total, 0.0);

        let add_var = |rows: &mut Vec<Vec<f64>>, n_total: &mut usize| {
            for r in rows {
                r.push(0.0);
            }
            *n_total += 1;
        };

        match sense {
            Sense::Le => {
                add_var(&mut rows, &mut n_total);
                row.push(1.0);
                basis.push(n_total - 1);
            }
            Sense::Ge => {
                add_var(&mut rows, &mut n_total);
                row.push(-1.0);
                add_var(&mut rows, &mut n_total);
                row.push(1.0);
                basis.push(n_total - 1);
                art_indices.push(n_total - 1);
            }
            Sense::Eq => {
                add_var(&mut rows, &mut n_total);
                row.push(1.0);
                basis.push(n_total - 1);
                art_indices.push(n_total - 1);
            }
        }

        rows.push(row);
        rhs.push(b);
    }

    let tableau: Vec<Vec<f64>> = rows
        .into_iter()
        .zip(rhs)
        .map(|(mut row, b)| {
            row.push(b);
            row
        })
        .collect();

    (tableau, basis, art_indices, n_total)
}

fn remove_artificial(
    mut tableau: Vec<Vec<f64>>,
    mut basis: Vec<usize>,
    art_indices: &[usize],
) -> (Vec<Vec<f64>>, Vec<usize>, HashMap<usize, usize>) {
    let art_set: HashSet<usize> = art_indices.iter().copied().collect();
    let mut m = basis.len();
    let mut n = tableau[0].len() - 1;

    let mut i = 0usize;
    while i < m {
        if art_set.contains(&basis[i]) {
            let mut pivot_col = None;
            for j in 0..n {
                if art_set.contains(&j) {
                    continue;
                }
                if tableau[i][j].abs() > EPS {
                    pivot_col = Some(j);
                    break;
                }
            }

            if let Some(col) = pivot_col {
                pivot(&mut tableau, &mut basis, i, col);
                i += 1;
            } else {
                if tableau[i][n].abs() > EPS {
                    panic!("infeasible during artificial removal");
                }
                tableau.remove(i);
                basis.remove(i);
                m -= 1;
            }
        } else {
            i += 1;
        }
    }

    n = tableau[0].len() - 1;
    let keep_cols: Vec<usize> = (0..n).filter(|j| !art_set.contains(j)).collect();
    let mapping: HashMap<usize, usize> =
        keep_cols.iter().enumerate().map(|(new, &old)| (old, new)).collect();

    let new_tableau: Vec<Vec<f64>> = tableau
        .into_iter()
        .map(|row| {
            let mut new_row = Vec::with_capacity(keep_cols.len() + 1);
            for &j in &keep_cols {
                new_row.push(row[j]);
            }
            new_row.push(row[n]);
            new_row
        })
        .collect();

    let new_basis: Vec<usize> = basis
        .into_iter()
        .map(|b| *mapping.get(&b).expect("basis var must be preserved"))
        .collect();

    (new_tableau, new_basis, mapping)
}

fn solve_lp(n_vars: usize, constraints: &[(Vec<f64>, Sense, f64)], objective: &[f64]) -> f64 {
    let (mut tableau, mut basis, art_indices, n_total) = build_tableau(n_vars, constraints);

    let mapping = if !art_indices.is_empty() {
        let mut c_phase1 = vec![0.0; n_total];
        for &j in &art_indices {
            c_phase1[j] = -1.0;
        }

        set_objective(&mut tableau, &basis, &c_phase1);
        assert!(simplex_max(&mut tableau, &mut basis), "unbounded in phase I");
        let rhs = tableau[0].len() - 1;
        assert!(tableau[basis.len()][rhs] >= -1e-7, "infeasible LP");

        tableau.pop();
        let (t2, b2, map2) = remove_artificial(tableau, basis, &art_indices);
        tableau = t2;
        basis = b2;
        map2
    } else {
        (0..n_total).map(|i| (i, i)).collect()
    };

    let n_total2 = tableau[0].len() - 1;
    let mut c_phase2 = vec![0.0; n_total2];
    for (j, &coef) in objective.iter().enumerate() {
        if let Some(&mapped) = mapping.get(&j) {
            c_phase2[mapped] = -coef;
        }
    }

    set_objective(&mut tableau, &basis, &c_phase2);
    assert!(simplex_max(&mut tableau, &mut basis), "unbounded in phase II");

    let rhs = tableau[0].len() - 1;
    -tableau[basis.len()][rhs]
}

fn build_and_solve(num_dice: usize) -> f64 {
    let mut states: Vec<Vec<u8>> = vec![Vec::new()];
    for _ in 0..num_dice {
        let mut next = Vec::with_capacity(states.len() * 6);
        for st in &states {
            for v in 1u8..=6 {
                let mut ns = st.clone();
                ns.push(v);
                next.push(ns);
            }
        }
        states = next;
    }

    let num_states = states.len();
    let hide_options: Vec<usize> = (0..num_dice).collect();

    let mut signal_set: BTreeSet<Vec<u8>> = BTreeSet::new();
    for t in &states {
        for &h in &hide_options {
            let mut revealed = Vec::with_capacity(num_dice - 1);
            for (i, &v) in t.iter().enumerate() {
                if i != h {
                    revealed.push(v);
                }
            }
            revealed.sort_unstable();
            signal_set.insert(revealed);
        }
    }

    let signals: Vec<Vec<u8>> = signal_set.into_iter().collect();
    let signal_index: HashMap<Vec<u8>, usize> = signals
        .iter()
        .cloned()
        .enumerate()
        .map(|(i, s)| (s, i))
        .collect();

    let num_x = num_states * num_dice;
    let num_z = signals.len();
    let n_vars = num_x + num_z;

    let x_index = |state_idx: usize, hide_idx: usize| -> usize { state_idx * num_dice + hide_idx };
    let z_index = |signal_idx: usize| -> usize { num_x + signal_idx };

    let mut reveal_sig_idx = vec![vec![0usize; num_dice]; num_states];
    for (s_idx, t) in states.iter().enumerate() {
        for &h in &hide_options {
            let mut revealed = Vec::with_capacity(num_dice - 1);
            for (i, &v) in t.iter().enumerate() {
                if i != h {
                    revealed.push(v);
                }
            }
            revealed.sort_unstable();
            reveal_sig_idx[s_idx][h] = *signal_index
                .get(&revealed)
                .expect("revealed signal must be indexed");
        }
    }

    let mut constraints: Vec<(Vec<f64>, Sense, f64)> = Vec::new();
    let p_state = 1.0 / num_states as f64;

    for s_idx in 0..num_states {
        let mut coeffs = vec![0.0; n_vars];
        for &h in &hide_options {
            coeffs[x_index(s_idx, h)] = 1.0;
        }
        constraints.push((coeffs, Sense::Eq, p_state));
    }

    for (sig_idx, sig) in signals.iter().enumerate() {
        let b_val = *sig.iter().max().expect("non-empty signal") as f64;

        let mut coeffs_vis = vec![0.0; n_vars];
        let mut coeffs_hid = vec![0.0; n_vars];
        coeffs_vis[z_index(sig_idx)] = -1.0;
        coeffs_hid[z_index(sig_idx)] = -1.0;

        for s_idx in 0..num_states {
            for &h in &hide_options {
                if reveal_sig_idx[s_idx][h] == sig_idx {
                    coeffs_vis[x_index(s_idx, h)] += b_val;
                    coeffs_hid[x_index(s_idx, h)] += states[s_idx][h] as f64;
                }
            }
        }

        constraints.push((coeffs_vis, Sense::Le, 0.0));
        constraints.push((coeffs_hid, Sense::Le, 0.0));
    }

    let mut objective = vec![0.0; n_vars];
    for sig_idx in 0..num_z {
        objective[z_index(sig_idx)] = 1.0;
    }

    solve_lp(n_vars, &constraints, &objective)
}

fn main() {
    let val_two = build_and_solve(2);
    let target_exact = 145.0 / 36.0;
    assert!((val_two - target_exact).abs() < 1e-8);
    assert!((val_two - 4.027_778).abs() < 1e-6);

    let val_three = build_and_solve(3);
    println!("{val_three:.6}");
}
