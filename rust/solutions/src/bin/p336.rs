const N: usize = 11;
const TARGET: usize = 2011;

fn reverse_suffix(arr: &mut [i32], start: usize) {
    let n = arr.len();
    let mut i = start;
    let mut j = n - 1;
    while i < j {
        arr.swap(i, j);
        i += 1;
        j -= 1;
    }
}

fn count_rotations(perm: &[i32]) -> usize {
    let n = perm.len();
    let mut arr = perm.to_vec();
    let mut rotations = 0;
    for target in 1..n as i32 {
        let mut pos = None;
        for j in (target as usize - 1)..n {
            if arr[j] == target { pos = Some(j); break; }
        }
        let pos = match pos { Some(p) => p, None => continue };
        if pos == target as usize - 1 { continue; }
        if pos != n - 1 { reverse_suffix(&mut arr, pos); rotations += 1; }
        reverse_suffix(&mut arr, target as usize - 1); rotations += 1;
    }
    rotations
}

fn generate(perm: &mut [i32; N], pos: usize, result_count: &mut usize, answer: &mut Option<String>) {
    if answer.is_some() { return; }
    if pos == N {
        let max_rot = 2 * (N - 1) - 1;
        if count_rotations(perm) == max_rot {
            *result_count += 1;
            if *result_count == TARGET {
                let s: String = perm.iter().map(|&v| (b'A' + v as u8 - 1) as char).collect();
                *answer = Some(s);
            }
        }
        return;
    }

    // Sort perm[pos..N]
    let mut tail: Vec<i32> = perm[pos..N].to_vec();
    tail.sort();
    perm[pos..N].copy_from_slice(&tail);

    let saved: Vec<i32> = perm[pos..N].to_vec();
    let nrem = N - pos;

    for ri in 0..nrem {
        let v = saved[ri];
        perm[pos] = v;
        let mut idx = pos + 1;
        for (j, &sv) in saved.iter().enumerate() {
            if j == ri { continue; }
            perm[idx] = sv;
            idx += 1;
        }
        generate(perm, pos + 1, result_count, answer);
        if answer.is_some() { return; }
    }
    // Restore
    for (i, &sv) in saved.iter().enumerate() {
        perm[pos + i] = sv;
    }
}

fn main() {
    let mut perm = [0i32; N];
    for i in 0..N { perm[i] = (i + 1) as i32; }
    let mut result_count = 0;
    let mut answer = None;
    generate(&mut perm, 0, &mut result_count, &mut answer);
    if let Some(s) = answer {
        println!("{}", s);
    } else {
        println!("NOT FOUND");
    }
}
