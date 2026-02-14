const N: usize = 5;
const SIZE: usize = 1 << N; // 32
const MASK: i64 = (1 << (N - 1)) - 1; // 0xF

static mut ANS: i64 = 0;
static mut USED: [bool; SIZE] = [false; SIZE];

fn helper(index: usize, seq: i64) {
    unsafe {
        if index == SIZE - 1 {
            ANS += seq >> (N - 1);
            return;
        }

        for bit in 0..2i64 {
            let subseq = ((seq & MASK) * 2 + bit) as usize;
            if !USED[subseq] {
                USED[subseq] = true;
                helper(index + 1, seq * 2 + bit);
                USED[subseq] = false;
            }
        }
    }
}

fn main() {
    unsafe {
        USED[0] = true;
        helper(0, 0);
        println!("{}", ANS);
    }
}
