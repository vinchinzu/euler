// Project Euler 54: Poker hands
// Count how many of the 1000 hands Player 1 wins.

fn card_value(c: u8) -> u8 {
    match c {
        b'2'..=b'9' => c - b'0',
        b'T' => 10,
        b'J' => 11,
        b'Q' => 12,
        b'K' => 13,
        b'A' => 14,
        _ => 0,
    }
}

fn hand_rank(cards: &[(u8, u8); 5]) -> Vec<u8> {
    let mut values: Vec<u8> = cards.iter().map(|c| c.0).collect();
    values.sort_unstable_by(|a, b| b.cmp(a));
    let suits: Vec<u8> = cards.iter().map(|c| c.1).collect();

    // Count frequencies
    let mut counts = [0u8; 15];
    for &v in &values {
        counts[v as usize] += 1;
    }

    // Build (count, value) pairs sorted by count desc, then value desc
    let mut pairs: Vec<(u8, u8)> = (2..=14u8)
        .filter(|&v| counts[v as usize] > 0)
        .map(|v| (counts[v as usize], v))
        .collect();
    pairs.sort_by(|a, b| b.cmp(a));

    let is_flush = suits.iter().all(|&s| s == suits[0]);
    let is_straight = values.windows(2).all(|w| w[0] == w[1] + 1);
    let is_wheel = values == [14, 5, 4, 3, 2];
    let straight_high = if is_wheel {
        5
    } else if is_straight {
        values[0]
    } else {
        0
    };

    let mut rank = Vec::new();

    if (is_straight || is_wheel) && is_flush {
        rank.push(8);
        rank.push(straight_high);
    } else if pairs[0].0 == 4 {
        rank.push(7);
        rank.push(pairs[0].1);
        rank.push(pairs[1].1);
    } else if pairs[0].0 == 3 && pairs.len() >= 2 && pairs[1].0 == 2 {
        rank.push(6);
        rank.push(pairs[0].1);
        rank.push(pairs[1].1);
    } else if is_flush {
        rank.push(5);
        rank.extend_from_slice(&values);
    } else if is_straight || is_wheel {
        rank.push(4);
        rank.push(straight_high);
    } else if pairs[0].0 == 3 {
        rank.push(3);
        for p in &pairs {
            rank.push(p.1);
        }
    } else if pairs[0].0 == 2 && pairs.len() >= 2 && pairs[1].0 == 2 {
        let high_pair = pairs[0].1.max(pairs[1].1);
        let low_pair = pairs[0].1.min(pairs[1].1);
        rank.push(2);
        rank.push(high_pair);
        rank.push(low_pair);
        rank.push(pairs[2].1);
    } else if pairs[0].0 == 2 {
        rank.push(1);
        for p in &pairs {
            rank.push(p.1);
        }
    } else {
        rank.push(0);
        rank.extend_from_slice(&values);
    }

    rank
}

fn main() {
    let data = include_str!("../../../../data/poker.txt");
    let mut count = 0;

    for line in data.lines() {
        let cards: Vec<&str> = line.split_whitespace().collect();
        if cards.len() != 10 {
            continue;
        }

        let parse_card = |s: &str| -> (u8, u8) {
            let b = s.as_bytes();
            (card_value(b[0]), b[1])
        };

        let mut hand1 = [(0u8, 0u8); 5];
        let mut hand2 = [(0u8, 0u8); 5];
        for i in 0..5 {
            hand1[i] = parse_card(cards[i]);
            hand2[i] = parse_card(cards[i + 5]);
        }

        let r1 = hand_rank(&hand1);
        let r2 = hand_rank(&hand2);

        if r1 > r2 {
            count += 1;
        }
    }

    println!("{count}");
}
