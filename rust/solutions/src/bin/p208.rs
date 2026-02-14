// Project Euler 208: Robot Walks - half-and-combine approach
use std::collections::HashMap;

const K: usize = 5;
const N: usize = 70;
const HALF: usize = N / 2;
const GOAL: usize = N / K; // 14

fn encode_key(left: &[u8; K], right: &[u8; K], dir: u8) -> u64 {
    let mut key: u64 = 0;
    for i in 0..K {
        key |= (left[i] as u64) << (i * 4);
    }
    for i in 0..K {
        key |= (right[i] as u64) << (20 + i * 4);
    }
    key |= (dir as u64) << 40;
    key
}

fn decode_key(key: u64) -> ([u8; K], [u8; K], u8) {
    let mut left = [0u8; K];
    let mut right = [0u8; K];
    for i in 0..K {
        left[i] = ((key >> (i * 4)) & 0xF) as u8;
    }
    for i in 0..K {
        right[i] = ((key >> (20 + i * 4)) & 0xF) as u8;
    }
    let dir = ((key >> 40) & 0x7) as u8;
    (left, right, dir)
}

fn main() {
    let mut cur: HashMap<u64, i64> = HashMap::new();
    let left0 = [0u8; K];
    let right0 = [0u8; K];
    cur.insert(encode_key(&left0, &right0, 0), 1);

    for _ in 0..HALF {
        let mut nxt: HashMap<u64, i64> = HashMap::new();
        for (&key, &val) in &cur {
            let (mut left, mut right, dir) = decode_key(key);

            // Counterclockwise: left[dir]++, dir = (dir+1)%K
            left[dir as usize] += 1;
            let nd1 = ((dir as usize + 1) % K) as u8;
            *nxt.entry(encode_key(&left, &right, nd1)).or_insert(0) += val;
            left[dir as usize] -= 1;

            // Clockwise: right[dir]++, dir = (dir+K-1)%K
            right[dir as usize] += 1;
            let nd2 = ((dir as usize + K - 1) % K) as u8;
            *nxt.entry(encode_key(&left, &right, nd2)).or_insert(0) += val;
            right[dir as usize] -= 1;
        }
        cur = nxt;
    }

    // Match halves
    let mut ans: i64 = 0;
    for (&key, &val) in &cur {
        let (left, right, dir) = decode_key(key);

        for goal in 0..=GOAL {
            let mut rem_left = [0u8; K];
            let mut rem_right = [0u8; K];
            let mut valid = true;
            for i in 0..K {
                let l = goal as i32 - left[(i + dir as usize) % K] as i32;
                let r = GOAL as i32 - goal as i32 - right[(i + dir as usize) % K] as i32;
                if l < 0 || r < 0 { valid = false; break; }
                rem_left[i] = l as u8;
                rem_right[i] = r as u8;
            }
            if !valid { continue; }

            let rem_dir = ((K - dir as usize) % K) as u8;
            let rem_key = encode_key(&rem_left, &rem_right, rem_dir);
            if let Some(&match_val) = cur.get(&rem_key) {
                ans += val * match_val;
            }
        }
    }

    println!("{}", ans);
}
