// Project Euler Problem 121: Disc game prize fund
// Dynamic programming to count winning weights, then compute prize fund.
use std::mem;

fn main() {
    const TURNS: usize = 15;

    let mut weights = [0i64; TURNS + 1];
    let mut next_weights = [0i64; TURNS + 1];
    weights[0] = 1;

    for turn in 1..=TURNS {
        next_weights.fill(0);
        for blue in 0..turn {
            let w = weights[blue];
            if w == 0 {
                continue;
            }
            next_weights[blue] += w * turn as i64; // draw red
            next_weights[blue + 1] += w; // draw blue
        }
        mem::swap(&mut weights, &mut next_weights);
    }

    let winning_weight: i64 = weights[TURNS / 2 + 1..=TURNS].iter().sum();

    let denominator: i64 = (2..=TURNS as i64 + 1).product();

    println!("{}", denominator / winning_weight);
}
