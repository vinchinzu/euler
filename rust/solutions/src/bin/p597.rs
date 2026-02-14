// Project Euler 597 - Torpedoes
//
// N boats starting at points 0, D, 2D, ... on the number line.
// Find the probability that the final order is an even permutation.
// Recursive approach with memoized combinatorial formula.

const MAXN: usize = 15;

struct Bump {
    prev: usize,
    bumper: usize,
    bumped: usize,
    after: usize,
}

fn helper(
    boats: &[usize],
    bumps: &mut Vec<Bump>,
    last_bumped: usize,
    n_val: usize,
    spots: &[f64],
    answer: &mut f64,
) {
    if bumps.len() == n_val {
        let mut parity = 0usize;
        let mut constant = 1.0f64;
        let mut exponents = [0.0f64; MAXN + 2];

        for i in 0..bumps.len() {
            let bumper = bumps[i].bumper;
            let bumped = bumps[i].bumped;

            let (next_bumper, next_bumped) = if i + 1 < bumps.len()
                && bumps[i + 1].bumper == bumped
            {
                (bumps[i].bumped, bumps[i].after)
            } else {
                (bumps[i].prev, bumps[i].bumped)
            };

            parity += bumper - bumps[i].prev;
            let exponent = exponents[bumper] + 1.0;
            constant /= exponent;
            let pow_val = (spots[bumped] - spots[bumper])
                / (spots[next_bumped] - spots[next_bumper]);
            exponents[next_bumper] += exponent * pow_val;
            exponents[next_bumped] -= exponent * pow_val;
            exponents[bumped] += exponent;
        }

        if parity % 2 == n_val % 2 {
            *answer += constant;
        }
        return;
    }

    for i in 1..boats.len() {
        if boats[i] > n_val || boats[i] > last_bumped {
            break;
        }

        let new_last_bumper = boats[i];

        // Remove boats[i]
        let mut new_boats: Vec<usize> = Vec::with_capacity(boats.len() - 1);
        for j in 0..boats.len() {
            if j != i {
                new_boats.push(boats[j]);
            }
        }

        let new_last_bumped = new_boats[i]; // was boats[i+1]
        let after = if i + 1 < new_boats.len() {
            new_boats[i + 1]
        } else {
            n_val + 1
        };

        bumps.push(Bump {
            prev: new_boats[i - 1],
            bumper: new_last_bumper,
            bumped: new_last_bumped,
            after,
        });

        helper(&new_boats, bumps, new_last_bumped, n_val, spots, answer);
        bumps.pop();
    }
}

fn main() {
    let n_val = 13;
    let l_val = 1800.0f64;
    let d_val = 40.0f64;

    let mut spots = [0.0f64; MAXN + 2];
    for i in 0..n_val {
        spots[i + 1] = d_val * i as f64;
    }
    spots[n_val + 1] = l_val;

    let mut answer = 0.0f64;

    let boats: Vec<usize> = (0..=n_val + 1).collect();
    let mut bumps: Vec<Bump> = Vec::new();

    helper(&boats, &mut bumps, n_val, n_val, &spots, &mut answer);

    println!("{:.10}", answer);
}
