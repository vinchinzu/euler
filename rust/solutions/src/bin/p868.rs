// Project Euler 868 - Bell Ringing (SJT Permutation Rank)
// Compute SJT rank of "NOWPICKBELFRYMATHS"

fn sjt_rank(perm: &[i32]) -> i64 {
    let n = perm.len();
    if n <= 1 { return 0; }

    let largest = n as i32 - 1;
    let pos = perm.iter().position(|&x| x == largest).unwrap();

    let sub_perm: Vec<i32> = perm.iter().filter(|&&x| x != largest).copied().collect();
    let r_sub = sjt_rank(&sub_perm);

    let local_index = if r_sub % 2 == 0 {
        (n - 1) - pos
    } else {
        pos
    };

    r_sub * n as i64 + local_index as i64
}

fn main() {
    let target = b"NOWPICKBELFRYMATHS";
    let n = target.len();

    let mut sorted: Vec<u8> = target.to_vec();
    sorted.sort_unstable();

    let mut char_map = [0i32; 256];
    for (i, &c) in sorted.iter().enumerate() {
        char_map[c as usize] = i as i32;
    }

    let perm: Vec<i32> = target.iter().map(|&c| char_map[c as usize]).collect();
    let result = sjt_rank(&perm);
    println!("{}", result);
}
