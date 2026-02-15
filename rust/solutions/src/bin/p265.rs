const N: usize = 5;
const SIZE: usize = 1 << N; // 32
const MASK: i64 = (1 << (N - 1)) - 1; // 0xF

fn helper(index: usize, seq: i64, used: &mut [bool; SIZE], ans: &mut i64) {
    if index == SIZE - 1 {
        *ans += seq >> (N - 1);
        return;
    }

    for bit in 0..2i64 {
        let subseq = ((seq & MASK) * 2 + bit) as usize;
        if !used[subseq] {
            used[subseq] = true;
            helper(index + 1, seq * 2 + bit, used, ans);
            used[subseq] = false;
        }
    }
}

fn main() {
    let mut used = [false; SIZE];
    let mut ans: i64 = 0;
    used[0] = true;
    helper(0, 0, &mut used, &mut ans);
    println!("{}", ans);
}
