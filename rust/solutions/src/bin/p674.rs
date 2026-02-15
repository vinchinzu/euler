// Project Euler 674 - Solving I-equations
//
// Parse I-expressions from data file, find least simultaneous values for all
// pairs. The I operator is defined as I(x,y) = (1+x+y)^2 + y - x.
//
// For two I-expressions e1, e2: if e1=e2 has a solution in non-negative integers,
// the least simultaneous value is the minimum value both attain at such a solution.
// If no solution exists, the value is 0.
//
// Sum all least simultaneous values for all distinct pairs from the data file,
// output last 9 digits.

use rayon::prelude::*;
use std::fs;

const MOD: u64 = 1_000_000_000;

struct INode {
    var: i32,       // variable index, -1 if I-application
    left: i32,      // left child index, -1 if leaf
    right: i32,     // right child index, -1 if leaf
    vars: Vec<i32>, // sorted list of variables in this subtree
}

struct INodePool {
    nodes: Vec<INode>,
    var_names: Vec<String>,
}

impl INodePool {
    fn new() -> Self {
        INodePool {
            nodes: Vec::new(),
            var_names: Vec::new(),
        }
    }

    fn find_or_add_var(&mut self, name: &str) -> i32 {
        for (i, vn) in self.var_names.iter().enumerate() {
            if vn == name {
                return i as i32;
            }
        }
        let idx = self.var_names.len();
        self.var_names.push(name.to_string());
        idx as i32
    }

    fn new_node_var(&mut self, var: i32) -> i32 {
        let id = self.nodes.len() as i32;
        self.nodes.push(INode {
            var,
            left: -1,
            right: -1,
            vars: vec![var],
        });
        id
    }

    fn new_node_i(&mut self, left: i32, right: i32) -> i32 {
        let id = self.nodes.len() as i32;
        let lv = &self.nodes[left as usize].vars;
        let rv = &self.nodes[right as usize].vars;
        let mut vars = Vec::with_capacity(lv.len() + rv.len());
        let (mut i, mut j) = (0, 0);
        while i < lv.len() && j < rv.len() {
            if lv[i] < rv[j] {
                vars.push(lv[i]);
                i += 1;
            } else if lv[i] > rv[j] {
                vars.push(rv[j]);
                j += 1;
            } else {
                vars.push(lv[i]);
                i += 1;
                j += 1;
            }
        }
        while i < lv.len() { vars.push(lv[i]); i += 1; }
        while j < rv.len() { vars.push(rv[j]); j += 1; }

        self.nodes.push(INode {
            var: -1,
            left,
            right,
            vars,
        });
        id
    }

    fn parse(&mut self, s: &[u8], pos: &mut usize) -> i32 {
        if *pos < s.len() && s[*pos] == b'I' {
            *pos += 2;
            let left = self.parse(s, pos);
            *pos += 1;
            let right = self.parse(s, pos);
            *pos += 1;
            self.new_node_i(left, right)
        } else {
            let start = *pos;
            while *pos < s.len() && s[*pos] >= b'a' && s[*pos] <= b'z' {
                *pos += 1;
            }
            let name = std::str::from_utf8(&s[start..*pos]).unwrap();
            let var = self.find_or_add_var(name);
            self.new_node_var(var)
        }
    }

    fn evaluate(&self, node: i32, values: &[u64]) -> u64 {
        let mut stack: Vec<(i32, u8)> = Vec::with_capacity(64);
        let mut val_stack: Vec<u64> = Vec::with_capacity(64);

        stack.push((node, 0));
        while let Some((nd, state)) = stack.last_mut() {
            let n = &self.nodes[*nd as usize];
            if n.var >= 0 {
                val_stack.push(values[n.var as usize]);
                stack.pop();
            } else if *state == 0 {
                *state = 1;
                stack.push((n.left, 0));
            } else if *state == 1 {
                *state = 2;
                stack.push((n.right, 0));
            } else {
                let y = val_stack.pop().unwrap();
                let x = val_stack.pop().unwrap();
                let sum = (1 + x + y) % MOD;
                val_stack.push((sum * sum % MOD + y + MOD - x % MOD) % MOD);
                stack.pop();
            }
        }
        val_stack[0]
    }

    fn num_vars(&self) -> usize {
        self.var_names.len()
    }
}

#[derive(Clone)]
struct Equality {
    expr: i32,
    var: i32,
}

fn build_equalities(pool: &INodePool, e1: i32, e2: i32, equalities: &mut Vec<Equality>) {
    let n1_var = pool.nodes[e1 as usize].var;
    let n2_var = pool.nodes[e2 as usize].var;

    if n1_var >= 0 && n2_var >= 0 {
        if n1_var < n2_var {
            equalities.push(Equality { expr: e1, var: n2_var });
        } else if n1_var > n2_var {
            equalities.push(Equality { expr: e2, var: n1_var });
        }
    } else if n1_var >= 0 {
        equalities.push(Equality { expr: e2, var: n1_var });
    } else if n2_var >= 0 {
        equalities.push(Equality { expr: e1, var: n2_var });
    } else {
        let n1_left = pool.nodes[e1 as usize].left;
        let n1_right = pool.nodes[e1 as usize].right;
        let n2_left = pool.nodes[e2 as usize].left;
        let n2_right = pool.nodes[e2 as usize].right;
        build_equalities(pool, n1_left, n2_left, equalities);
        build_equalities(pool, n1_right, n2_right, equalities);
    }
}

fn least_simultaneous_value(pool: &INodePool, e1: i32, e2: i32, num_vars: usize) -> u64 {
    let mut equalities: Vec<Equality> = Vec::new();
    build_equalities(pool, e1, e2, &mut equalities);

    if equalities.is_empty() {
        // e1 == e2 structurally (all variables match), evaluate with all zeros
        let values = vec![0u64; num_vars];
        return pool.evaluate(e1, &values);
    }

    // Optimized topological sort using reference counts
    // var_ref_count[v] = number of active equalities whose expr contains var v
    let mut var_ref_count = vec![0i32; num_vars];
    let mut active: Vec<bool> = vec![true; equalities.len()];

    // Initialize ref counts: for each active equality's expr, increment for each var in it
    for eq in &equalities {
        for &v in &pool.nodes[eq.expr as usize].vars {
            var_ref_count[v as usize] += 1;
        }
    }

    let mut evaluations: Vec<Equality> = Vec::new();
    let mut total = equalities.len();

    while total > 0 {
        // Find an equality where var_ref_count[equality.var] == 0
        let mut found: Option<usize> = None;
        for i in 0..equalities.len() {
            if !active[i] { continue; }
            if var_ref_count[equalities[i].var as usize] == 0 {
                found = Some(i);
                break;
            }
        }

        let found_idx = match found {
            Some(idx) => idx,
            None => return 0,
        };

        let target_var = equalities[found_idx].var;

        // Collect all equalities with this variable
        let mut good: Vec<Equality> = Vec::new();
        for i in 0..equalities.len() {
            if active[i] && equalities[i].var == target_var {
                good.push(equalities[i].clone());
                active[i] = false;
                total -= 1;
                // Decrement ref counts for vars in this expr
                for &v in &pool.nodes[equalities[i].expr as usize].vars {
                    var_ref_count[v as usize] -= 1;
                }
            }
        }

        evaluations.push(good[0].clone());

        // Add new equalities for remaining pairs
        for k in 1..good.len() {
            let old_len = equalities.len();
            build_equalities(pool, good[0].expr, good[k].expr, &mut equalities);
            for idx in old_len..equalities.len() {
                active.push(true);
                total += 1;
                // Increment ref counts for vars in new expr
                for &v in &pool.nodes[equalities[idx].expr as usize].vars {
                    var_ref_count[v as usize] += 1;
                }
            }
        }
    }

    let mut values = vec![0u64; num_vars];

    for i in (0..evaluations.len()).rev() {
        values[evaluations[i].var as usize] = pool.evaluate(evaluations[i].expr, &values);
    }

    pool.evaluate(e1, &values)
}

fn main() {
    let paths = [
        "python/0674_i_expressions.txt",
        "../python/0674_i_expressions.txt",
        "../../python/0674_i_expressions.txt",
        "data/0674_i_expressions.txt",
        "../data/0674_i_expressions.txt",
    ];

    let content = paths.iter()
        .find_map(|p| fs::read_to_string(p).ok())
        .expect("Cannot open 0674_i_expressions.txt");

    let mut pool = INodePool::new();
    let mut exprs: Vec<i32> = Vec::new();

    for line in content.lines() {
        let line = line.trim();
        if line.is_empty() { continue; }
        let bytes = line.as_bytes();
        let mut pos = 0;
        let expr = pool.parse(bytes, &mut pos);
        exprs.push(expr);
    }

    let num_vars = pool.num_vars();
    let n = exprs.len();
    let pairs: Vec<(usize, usize)> = (0..n)
        .flat_map(|i| ((i + 1)..n).map(move |j| (i, j)))
        .collect();

    let ans: u64 = pairs.par_iter()
        .map(|&(i, j)| least_simultaneous_value(&pool, exprs[i], exprs[j], num_vars))
        .sum();

    println!("{}", ans % MOD);
}
