// Project Euler 480: The Last Question
// From "thereisasyetinsufficientdataforameaningfulanswer", consider all words
// formed by selecting at most 15 letters (respecting counts).
// Given P(w) = position, find
// W(P(legionary) + P(calorimeters) - P(annihilate) + P(orchestrated) - P(fluttering)).

const K: usize = 15;

// Sorted unique letters from the phrase
const LETTERS: &[u8] = b"acdefghilmnorstuwy";

#[derive(Clone, Copy)]
struct Rat { num: i128, den: i128 }

impl Rat {
    fn new(n: i128, d: i128) -> Rat {
        if n == 0 { return Rat { num: 0, den: 1 }; }
        let (mut n, mut d) = if d < 0 { (-n, -d) } else { (n, d) };
        let g = {
            let mut a = n.abs();
            let mut b = d;
            while b != 0 { let t = b; b = a % b; a = t; }
            a
        };
        n /= g; d /= g;
        Rat { num: n, den: d }
    }
    fn zero() -> Rat { Rat { num: 0, den: 1 } }
    fn one() -> Rat { Rat { num: 1, den: 1 } }
    fn add(self, b: Rat) -> Rat { Rat::new(self.num * b.den + b.num * self.den, self.den * b.den) }
    fn mul(self, b: Rat) -> Rat { Rat::new(self.num * b.num, self.den * b.den) }
}

fn count_words(avail: &mut [usize], max_len: usize, n_letters: usize, fact: &[i64]) -> i64 {
    if max_len == 0 { return 0; }

    let mut dp = vec![Rat::zero(); max_len + 1];
    dp[0] = Rat::one();

    for i in 0..n_letters {
        let fi = avail[i];
        if fi == 0 { continue; }
        let mut new_dp = vec![Rat::zero(); max_len + 1];
        for j in 0..=max_len {
            if dp[j].num == 0 { continue; }
            let lim = fi.min(max_len - j);
            for c in 0..=lim {
                let inv_c_fact = Rat::new(1, fact[c] as i128);
                new_dp[j + c] = new_dp[j + c].add(dp[j].mul(inv_c_fact));
            }
        }
        dp = new_dp;
    }

    let mut total: i64 = 0;
    for l in 1..=max_len {
        let val = dp[l].mul(Rat::new(fact[l] as i128, 1));
        total += val.num as i64;
    }
    total
}

fn letter_to_idx(c: u8) -> usize {
    LETTERS.iter().position(|&x| x == c).unwrap()
}

fn position(word: &[u8], freq_list: &[usize], n_letters: usize, fact: &[i64]) -> i64 {
    let len = word.len();
    let mut pos: i64 = 0;
    let mut avail: Vec<usize> = freq_list.to_vec();

    for i in 0..len {
        let ci = letter_to_idx(word[i]);
        for li in 0..ci {
            if avail[li] > 0 {
                avail[li] -= 1;
                pos += 1 + count_words(&mut avail, K - i - 1, n_letters, fact);
                avail[li] += 1;
            }
        }
        avail[ci] -= 1;
        if i < len - 1 {
            pos += 1; // count this prefix as a shorter word
        }
    }
    pos += 1; // the word itself
    pos
}

fn find_word(mut p: i64, freq_list: &[usize], n_letters: usize, fact: &[i64]) -> String {
    let mut avail: Vec<usize> = freq_list.to_vec();
    let mut result = Vec::new();

    for depth in 0..K {
        let mut found = false;
        for li in 0..n_letters {
            if avail[li] > 0 {
                avail[li] -= 1;
                let cnt = 1 + count_words(&mut avail, K - depth - 1, n_letters, fact);
                if p <= cnt {
                    result.push(LETTERS[li]);
                    if p == 1 {
                        return String::from_utf8(result).unwrap();
                    }
                    p -= 1;
                    found = true;
                    break;
                }
                p -= cnt;
                avail[li] += 1;
            }
        }
        if !found { break; }
    }
    String::from_utf8(result).unwrap()
}

fn main() {
    let mut fact = [0i64; K + 1];
    fact[0] = 1;
    for i in 1..=K { fact[i] = fact[i - 1] * i as i64; }

    let phrase = b"thereisasyetinsufficientdataforameaningfulanswer";
    let mut counts = [0usize; 256];
    for &c in phrase.iter() { counts[c as usize] += 1; }

    let n_letters = LETTERS.len();
    let mut freq_list = vec![0usize; n_letters];
    for i in 0..n_letters {
        freq_list[i] = counts[LETTERS[i] as usize];
    }

    let mut total_pos: i64 = 0;
    total_pos += position(b"legionary", &freq_list, n_letters, &fact);
    total_pos += position(b"calorimeters", &freq_list, n_letters, &fact);
    total_pos -= position(b"annihilate", &freq_list, n_letters, &fact);
    total_pos += position(b"orchestrated", &freq_list, n_letters, &fact);
    total_pos -= position(b"fluttering", &freq_list, n_letters, &fact);

    let answer = find_word(total_pos, &freq_list, n_letters, &fact);
    println!("{answer}");
}
