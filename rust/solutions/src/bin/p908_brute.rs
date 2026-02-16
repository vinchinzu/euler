// Brute force for Problem 908: Clock Sequence II
// A clock sequence has period P: (a_1, ..., a_P) with a_i >= 1.
// Segments: segment n has sum n, using consecutive elements from the periodic sequence.
// Count distinct clock sequences with period <= N.

fn main() {
    for max_period in &[3, 4, 10] {
        let count = count_clock_sequences(*max_period);
        println!("C({}) = {}", max_period, count);
    }
}

fn count_clock_sequences(max_period: usize) -> u64 {
    let mut count = 0u64;
    for period in 1..=max_period {
        count += count_with_period(period);
    }
    count
}

fn count_with_period(period: usize) -> u64 {
    // Try all sequences of positive integers with the given period.
    // Sum of one period must divide into segments summing to 1, 2, 3, ...
    // We need to enumerate sequences (a_1, ..., a_period) with a_i >= 1.

    // But we also need to ensure the period is EXACTLY period, not a divisor.
    // Actually, re-reading the problem: "period at most N" and C counts sequences.
    // A sequence with period 2 also has period 4, 6, etc. But we should count
    // each distinct sequence once. Let me count sequences whose MINIMAL period
    // divides some value <= N... actually, let me just count distinct infinite
    // sequences that have SOME period <= N.

    // Actually, for brute force: enumerate all tuples (a_1,...,a_p) for p=1..N,
    // check if the minimal period divides p, and if the segmentation works.
    // To avoid double counting, only count when p is the minimal period.

    let mut count = 0u64;

    // For small periods, enumerate all possible sequences.
    // The sum of the period must be such that the greedy segmentation works.
    // Maximum element value: limited by the fact that segment 1 must have sum 1,
    // so the first element must be 1 (if we start segment 1 at position 0).

    // Wait -- the first segment has sum 1, so the consecutive elements starting
    // from position 0 must sum to 1. Since all elements >= 1, segment 1 is just
    // the first element, which must be 1.

    // Hmm, but position 0 of the sequence might not be where segment 1 starts.
    // Actually, the segmentation starts from position 0 of the sequence.
    // So a_1 = 1.

    // Actually wait - I need to re-read. "a periodic sequence of positive integers
    // that can be broken into contiguous segments such that the sum of the n-th
    // segment is equal to n." So the sequence starts from position 0, and the
    // segments are contiguous from the beginning. Segment 1 starts at position 0.

    // So a_1 = 1 (since segment 1 has sum 1 and a_1 >= 1).

    // Segment 2 has sum 2, starting from position 1.
    // If a_2 = 2, segment 2 = [2], one element.
    // If a_2 = 1, then we need a_3 to make sum 2, so a_3 = 1. Segment 2 = [1, 1].
    // Etc.

    // Max value of any element: bounded by the fact that eventually we need
    // a segment with small sum. The element values are bounded by the period's
    // sum and structure.

    // For period p, max element value: the maximum segment sum that can be
    // formed in one period is at most sum of all elements. For small p, let's
    // just try values 1..max_val.

    let max_val = 10 * period; // generous upper bound
    let mut seq = vec![0usize; period];
    enumerate_sequences(period, max_val, &mut seq, 0, &mut count);

    count
}

fn enumerate_sequences(period: usize, max_val: usize, seq: &mut Vec<usize>, pos: usize, count: &mut u64) {
    if pos == period {
        // Check if this sequence works as a clock sequence
        // and has minimal period = period
        if has_minimal_period(seq, period) && is_valid_clock(seq) {
            *count += 1;
        }
        return;
    }

    for v in 1..=max_val {
        seq[pos] = v;
        // Early termination: first element must be 1
        if pos == 0 && v != 1 {
            break;
        }
        enumerate_sequences(period, max_val, seq, pos + 1, count);
    }
}

fn has_minimal_period(seq: &[usize], period: usize) -> bool {
    // Check that no smaller period divides this one
    for d in 1..period {
        if period % d == 0 {
            let mut is_subperiod = true;
            for i in 0..period {
                if seq[i] != seq[i % d] {
                    is_subperiod = false;
                    break;
                }
            }
            if is_subperiod {
                return false;
            }
        }
    }
    true
}

fn is_valid_clock(seq: &[usize]) -> bool {
    let period = seq.len();
    // Check that the segmentation works for many segments
    let mut pos = 0usize; // position in period
    let max_segments = 200; // check enough segments

    for n in 1..=max_segments {
        let mut sum = 0;
        let mut steps = 0;
        while sum < n {
            sum += seq[pos % period];
            pos += 1;
            steps += 1;
            if steps > period * 2 + n {
                return false; // safety
            }
        }
        if sum != n {
            return false; // overshot
        }
    }
    true
}

