// Project Euler 240: Top Dice

const T: i64 = 70;
const NN: usize = 20;
const K: usize = 10;
const S: i64 = 12;

fn n_cr(n: i64, k: i64) -> i64 {
    if k < 0 || k > n { return 0; }
    let k = k.min(n - k);
    let mut r = 1i64;
    for i in 0..k {
        r = r * (n - i) / (i + 1);
    }
    r
}

fn pow_int(base: i64, exp: i64) -> i64 {
    let mut r = 1i64;
    for _ in 0..exp { r *= base; }
    r
}

fn helper(
    min_die: i64,
    num_dice: usize,
    sum_val: i64,
    dice_vals: &mut Vec<i64>,
    dice_cnts: &mut Vec<i64>,
    ans: &mut i64,
) {
    if num_dice == K {
        if sum_val != T { return; }
        let lowest_die = dice_vals[0];

        for i in 0..=(NN - K) as i64 {
            let mut num_dice_remaining = NN as i64;
            let mut num_ways = 1i64;

            for (d, &cnt) in dice_cnts.iter().enumerate() {
                let count = if d == 0 { cnt + i } else { cnt };
                num_ways *= n_cr(num_dice_remaining, count);
                num_dice_remaining -= count;
            }

            num_ways *= pow_int(lowest_die - 1, num_dice_remaining);
            *ans += num_ways;
        }
        return;
    }

    for die in min_die..=S {
        for count in 1..=(K - num_dice) as i64 {
            if sum_val + count * die > T { break; }
            dice_vals.push(die);
            dice_cnts.push(count);
            helper(die + 1, num_dice + count as usize, sum_val + count * die,
                   dice_vals, dice_cnts, ans);
            dice_vals.pop();
            dice_cnts.pop();
        }
    }
}

fn main() {
    let mut ans = 0i64;
    let mut dice_vals = Vec::new();
    let mut dice_cnts = Vec::new();
    helper(1, 0, 0, &mut dice_vals, &mut dice_cnts, &mut ans);
    println!("{}", ans);
}
