// Smarter brute force for Problem 908: Clock Sequence II
// Enumerate valid clock sequences by building them element by element,
// constrained by the segmentation requirements.

fn main() {
    for max_period in &[3, 4, 6, 10, 15, 20] {
        let count = count_clock_sequences(*max_period);
        println!("C({}) = {}", max_period, count);
    }
}

fn count_clock_sequences(max_period: usize) -> u64 {
    let mut total = 0u64;
    for period in 1..=max_period {
        total += count_with_exact_period(period);
    }
    total
}

fn count_with_exact_period(period: usize) -> u64 {
    // Build the sequence element by element.
    // State: position in period we're filling (0..period-1),
    //        current segment number (starting from 1),
    //        current partial sum within segment,
    //        whether we've completed enough to close the period.

    let mut seq = vec![0usize; period];
    let mut count = 0u64;

    // Position 0 must be 1 (segment 1 has sum 1, first element >= 1, so a_1 = 1)
    seq[0] = 1;

    // After consuming a_1 = 1: segment 1 is complete (sum = 1).
    // Start segment 2 at position 1.
    // Current state: filling position 1, segment 2, partial sum 0.

    if period == 1 {
        // Sequence is just [1] repeating. Segment 1 = [1].
        // Segment 2 must be sum 2 from position 0: [1, 1] = sum 2. OK.
        // Segment 3 must be sum 3 from position 0: [1, 1, 1] = sum 3. OK.
        // In general, segment n = n copies of [1]. Always works.
        // Minimal period is 1. Count it.
        count += 1;
    } else {
        build_sequence(&mut seq, period, 1, 2, 0, &mut count);
    }

    count
}

fn build_sequence(
    seq: &mut Vec<usize>,
    period: usize,
    fill_pos: usize,      // which position in the period we're filling next
    segment: usize,        // current segment number (sum must equal this)
    partial_sum: usize,    // sum accumulated so far in current segment
    count: &mut u64,
) {
    if fill_pos == period {
        // All positions filled. Now verify the sequence works as a clock sequence.
        // Also check minimal period (must not have a smaller period).
        if !has_minimal_period(seq) {
            return;
        }
        if validate_clock_sequence(seq) {
            *count += 1;
        }
        return;
    }

    let remaining = segment - partial_sum;
    // The next element a[fill_pos] must be >= 1 and <= remaining (to not overshoot)
    // Also, the element should be reasonable (not too large for future segments)

    for val in 1..=remaining {
        seq[fill_pos] = val;
        let new_partial = partial_sum + val;
        let new_fill = fill_pos + 1;

        if new_partial == segment {
            // Segment complete. Next segment starts.
            build_sequence(seq, period, new_fill, segment + 1, 0, count);
        } else {
            // Still in same segment
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
    let s: usize = seq.iter().sum();

    // Simulate segmentation for segments 1, 2, 3, ... up to a limit
    // The segmentation must work. For large enough segments, it always works
    // (since elements are bounded). We need to check until we're sure.

    let max_seg = 5 * period * period + 100; // generous limit
    let max_elem = *seq.iter().max().unwrap();

    let mut pos = 0usize; // position in period

    for n in 1..=max_seg {
        let mut sum = 0;
        loop {
            let elem = seq[pos % period];
            if sum + elem > n {
                return false; // overshoot
            }
            sum += elem;
            pos += 1;
            if sum == n {
                break;
            }
        }
    }

    true
}
