// Project Euler 308: An amazing prime-generating automaton
//
// Conway's PRIMEGAME FRACTRAN program. Count steps to reach 10001st prime power of 2.
// Uses optimized state machine with loop shortcuts.

fn main() {
    let mut two: i64 = 1;
    let mut three: i64 = 0;
    let mut five: i64 = 0;
    let mut seven: i64 = 0;
    let mut state = 0;
    let mut steps: i64 = 0;
    let mut prime_count = 0;
    let target = 10001;

    loop {
        if state == 0 && three == 0 && five == 0 && seven == 0 && two > 1 {
            prime_count += 1;
            if prime_count >= target {
                break;
            }
        }

        match state {
            0 => {
                if two > 0 {
                    steps += two;
                    three += two;
                    five += two;
                    two = 0;
                } else if seven > 0 {
                    seven -= 1;
                    steps += 1;
                } else {
                    five += 1;
                    state = 1;
                    steps += 1;
                }
            }
            1 => {
                if three > 0 {
                    steps += 2 * three;
                    seven += three;
                    three = 0;
                } else {
                    state = 2;
                    steps += 1;
                }
            }
            2 => {
                if five > 0 && seven > 0 {
                    let min_val = five.min(seven);
                    steps += 2 * min_val;
                    two += min_val;
                    three += min_val;
                    five -= min_val;
                    seven -= min_val;
                }
                if seven > 0 {
                    seven -= 1;
                    state = 3;
                    steps += 1;
                } else {
                    state = 1;
                    steps += 1;
                }
            }
            3 => {
                if five > 0 {
                    five -= 1;
                    two += 1;
                    three += 1;
                    state = 2;
                    steps += 1;
                } else if three > 0 {
                    three -= 1;
                    state = 4;
                    steps += 1;
                } else {
                    state = 0;
                    steps += 1;
                }
            }
            4 => {
                if two > 0 {
                    steps += 2 * two;
                    five += two;
                    two = 0;
                } else {
                    seven += 1;
                    state = 1;
                    steps += 1;
                }
            }
            _ => unreachable!(),
        }
    }

    println!("{}", steps);
}
