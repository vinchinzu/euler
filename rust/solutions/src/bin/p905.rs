// Project Euler 905 - Three-player game with Euclidean-like reduction
// Compute F(a^b, b^a, a^b + b^a) for a=1..7, b=1..19 and sum results

fn next_turn(prev_turn: i64, player: i64) -> i64 {
    let base = player + 1;
    if base > prev_turn {
        return base;
    }
    let k = (prev_turn - base) / 3 + 1;
    base + 3 * k
}

fn f(a_val: i64, b_val: i64, c_val: i64) -> i64 {
    let mut cur = [a_val, b_val, c_val];
    let mut batch_max_idx = Vec::new();
    let mut batch_mid_idx = Vec::new();
    let mut batch_q = Vec::new();

    loop {
        // Find max index
        let mut max_i = 0;
        if cur[1] > cur[max_i] { max_i = 1; }
        if cur[2] > cur[max_i] { max_i = 2; }

        // Find min and mid among non-max
        let mut others = Vec::new();
        for i in 0..3 {
            if i != max_i { others.push(i); }
        }
        if cur[others[0]] > cur[others[1]] {
            others.swap(0, 1);
        }
        let min_i = others[0];
        let min_val = cur[min_i];
        let mid_i = others[1];
        let mid_val = cur[mid_i];

        if min_val == 0 { break; }

        let q = mid_val / min_val;
        let remainder = mid_val % min_val;

        batch_max_idx.push(max_i);
        batch_mid_idx.push(mid_i);
        batch_q.push(q);

        if q % 2 == 1 {
            cur[max_i] = remainder;
            cur[mid_i] = remainder + min_val;
        } else {
            cur[mid_i] = remainder;
            cur[max_i] = remainder + min_val;
        }
    }

    // Reverse batches
    batch_max_idx.reverse();
    batch_mid_idx.reverse();
    batch_q.reverse();

    let mut prev_turn = 0i64;

    for b in 0..batch_max_idx.len() {
        let p_a = batch_max_idx[b] as i64;
        let p_b = batch_mid_idx[b] as i64;
        let q = batch_q[b];

        let (first_player, second_player) = if q % 2 == 1 {
            (p_a, p_b)
        } else {
            (p_b, p_a)
        };

        if q == 0 { continue; }

        prev_turn = next_turn(prev_turn, first_player);
        let mut remaining = q - 1;
        if remaining == 0 { continue; }

        prev_turn = next_turn(prev_turn, second_player);
        remaining -= 1;
        if remaining == 0 { continue; }

        let pairs = remaining / 2;
        let leftover = remaining % 2;

        prev_turn += pairs * 3;
        if leftover != 0 {
            prev_turn = next_turn(prev_turn, first_player);
        }
    }

    prev_turn
}

fn main() {
    let mut total = 0i64;
    for a in 1..=7i64 {
        for b in 1..=19i64 {
            let mut a_val = 1i128;
            for _ in 0..b { a_val *= a as i128; }
            let mut b_val = 1i128;
            for _ in 0..a { b_val *= b as i128; }
            let c_val = a_val + b_val;

            let result = f(a_val as i64, b_val as i64, c_val as i64);
            total += result;
        }
    }
    println!("{}", total);
}
