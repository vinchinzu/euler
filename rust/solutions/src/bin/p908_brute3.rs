// Per-period count brute force for Problem 908
// Output c(P) = number of clock sequences with minimal period P

fn main() {
    let max_p = 20;
    let mut cumulative = 0u64;
    for period in 1..=max_p {
        let c = count_with_exact_period(period);
        cumulative += c;
        println!("c({}) = {}, C({}) = {}", period, c, period, cumulative);
    }
}

fn count_with_exact_period(period: usize) -> u64 {
    let mut seq = vec![0usize; period];
    let mut count = 0u64;
    seq[0] = 1;

    if period == 1 {
        count += 1;
    } else {
        build_sequence(&mut seq, period, 1, 2, 0, &mut count);
    }
    count
}

fn build_sequence(
    seq: &mut Vec<usize>,
    period: usize,
    fill_pos: usize,
    segment: usize,
    partial_sum: usize,
    count: &mut u64,
) {
    if fill_pos == period {
        if !has_minimal_period(seq) {
            return;
        }
        if validate_clock_sequence(seq) {
            *count += 1;
        }
        return;
    }

    let remaining = segment - partial_sum;
    for val in 1..=remaining {
        seq[fill_pos] = val;
        let new_partial = partial_sum + val;
        let new_fill = fill_pos + 1;

        if new_partial == segment {
            build_sequence(seq, period, new_fill, segment + 1, 0, count);
        } else {
            build_sequence(seq, period, new_fill, segment, new_partial, count);
        }
    }
}

fn has_minimal_period(seq: &[usize]) -> bool {
    let period = seq.len();
    for d in 1..period {
        if period % d == 0 {
            let mut is_sub = true;
            for i in 0..period {
                if seq[i] != seq[i % d] {
                    is_sub = false;
                    break;
                }
            }
            if is_sub {
                return false;
            }
        }
    }
    true
}

fn validate_clock_sequence(seq: &[usize]) -> bool {
    let period = seq.len();
    let max_seg = 3 * period * period + 100;
    let mut pos = 0usize;

    for n in 1..=max_seg {
        let mut sum = 0;
        let mut steps = 0;
        loop {
            let elem = seq[pos % period];
            if sum + elem > n {
                return false;
            }
            sum += elem;
            pos += 1;
            steps += 1;
            if sum == n {
                break;
            }
            if steps > 2 * period + n {
                return false;
            }
        }
    }
    true
}
