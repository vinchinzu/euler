// Project Euler 900 — DistribuNim II — WRONG ANSWER
// Expected: 646900900, currently outputs: 296202364
//
// Problem: Game with n+1 piles (n piles of n stones, one pile of n+k).
// Each turn: remove exactly min(piles) stones total, can't empty any pile.
// t(n) = smallest k >= 0 such that position is losing for first player.
// S(N) = sum_{n=1}^{2^N} t(n). Given S(10) = 361522. Find S(10^4) mod 900497239.
//
// Current formula ((4^N+2)/3 - 2^N) is WRONG — all refs (C, Python, Rust) use it.
//
// Investigation notes:
// - Computed t(n) by game-tree search for n=1..9:
//   t(1)=0, t(2)=0, t(3)=2, t(4)=0, t(5)=6, t(6)=4, t(7)=6, t(8)=0, t(9)=14
//   (t(1),t(2),t(3) confirmed by problem statement)
// - Hypothesis t(n) = 2^{v+1}*(2^a-1) where n=2^v*m (m odd), a=floor(log2(m))
//   matches n=1..9 but gives S(10)=688810 != 361522. Formula breaks for n>=10.
// - Game solver too slow for n>=10 (state space = partitions of ~n^2 into n+1 parts).
// - S(N) is NOT of form a*4^N + b*N*2^N + c*2^N + d (overdetermined by S(0..4) vs S(10)).
// - Need either: (a) faster game solver to get t(n) for n=10..32 and find true pattern,
//   or (b) mathematical analysis of the DistribuNim II game (PE #899 is related 2-pile version).
// - DistribuNim I (PE #899) is the 2-pile variant; understanding it may help.

fn mod_pow(mut base: i64, mut exp: i64, m: i64) -> i64 {
    let mut result = 1i64;
    base = ((base % m) + m) % m;
    while exp > 0 {
        if exp & 1 == 1 {
            result = result * base % m;
        }
        base = base * base % m;
        exp >>= 1;
    }
    result
}

fn main() {
    let p: i64 = 900497239;
    let n: i64 = 10000;

    let four_n = mod_pow(4, n, p);
    let two_n = mod_pow(2, n, p);
    let three_inv = mod_pow(3, p - 2, p); // Fermat's little theorem

    let s = ((four_n + 2) % p * three_inv % p - two_n) % p;
    let s = ((s % p) + p) % p; // Ensure positive

    println!("{}", s);
}
