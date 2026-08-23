// Problem 961 - Removing Digits
//
// Two-player game: remove one digit per turn (leading zeros then vanish);
// last nonzero digit wins. W(N) = count of n < N that are first-player wins.
//
// Only the zero/nonzero mask matters. All odd-length masks are N-positions.
// An even-length mask wins iff deleting the leading digit leaves a P-position.
// Weighted counts A_n of winning n-digit numbers then satisfy
//   A_2 = 9,  A_{2m+2} = 81 * 10^{2m-1} - 8 A_{2m},
// which sums to the closed form W(10^{2m}) = (100^m - (-8)^m) / 6.

fn w(k: usize) -> u128 {
    if k % 2 == 0 {
        let m = (k / 2) as u32;
        let p100 = 100u128.pow(m);
        let p8 = 8u128.pow(m);
        if m % 2 == 0 {
            (p100 - p8) / 6
        } else {
            (p100 + p8) / 6
        }
    } else {
        // W(10^{2m+1}) = W(10^{2m}) + 9 * 10^{2m}  (every odd-length mask wins)
        let m = (k / 2) as u32;
        let p100 = 100u128.pow(m);
        let p8 = 8u128.pow(m);
        let even = if m % 2 == 0 { p100 - p8 } else { p100 + p8 };
        even / 6 + 9 * p100
    }
}

fn main() {
    debug_assert_eq!(w(2), 18);
    debug_assert_eq!(w(4), 1656);
    println!("{}", w(18));
}
