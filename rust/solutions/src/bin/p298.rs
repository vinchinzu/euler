// Project Euler 298: Selective Amnesia
use std::collections::HashMap;

const TURNS: usize = 50;
const K_VAL: usize = 10;
const L_VAL: usize = 5;
const DIFF_SIZE: usize = 2 * TURNS + 1;
const DIFF_OFFSET: i32 = TURNS as i32;

fn encode_mems(larry: &[u8; L_VAL], robin: &[u8; L_VAL]) -> u64 {
    let mut mapping = [255u8; K_VAL + 2];
    mapping[0] = 0;
    let mut next_id = 1u8;

    let mut key: u64 = 0;
    for i in 0..L_VAL {
        if mapping[larry[i] as usize] == 255 {
            mapping[larry[i] as usize] = next_id;
            next_id += 1;
        }
        key = key * 12 + mapping[larry[i] as usize] as u64;
    }
    for i in 0..L_VAL {
        if mapping[robin[i] as usize] == 255 {
            mapping[robin[i] as usize] = next_id;
            next_id += 1;
        }
        key = key * 12 + mapping[robin[i] as usize] as u64;
    }
    key
}

fn decode_mems(mut key: u64) -> ([u8; L_VAL], [u8; L_VAL]) {
    let mut robin = [0u8; L_VAL];
    let mut larry = [0u8; L_VAL];
    for i in (0..L_VAL).rev() {
        robin[i] = (key % 12) as u8;
        key /= 12;
    }
    for i in (0..L_VAL).rev() {
        larry[i] = (key % 12) as u8;
        key /= 12;
    }
    (larry, robin)
}

fn main() {
    let mut cur: HashMap<u64, Vec<f64>> = HashMap::new();

    let init_larry = [0u8; L_VAL];
    let init_robin = [0u8; L_VAL];
    let init_key = encode_mems(&init_larry, &init_robin);
    let mut init_prob = vec![0.0; DIFF_SIZE];
    init_prob[DIFF_OFFSET as usize] = 1.0;
    cur.insert(init_key, init_prob);

    for _turn in 0..TURNS {
        let mut nxt: HashMap<u64, Vec<f64>> = HashMap::new();

        for (&key, prob) in &cur {
            let (larry, robin) = decode_mems(key);

            for called in 1..=K_VAL as u8 {
                let mut diff_change: i32 = 0;

                // Update Larry's memory
                let mut new_larry = [0u8; L_VAL];
                let found_l = larry.iter().position(|&v| v == called);
                if found_l.is_some() {
                    diff_change += 1;
                    let fl = found_l.unwrap();
                    let mut j = 0;
                    for i in 0..L_VAL {
                        if i == fl { continue; }
                        new_larry[j] = larry[i];
                        j += 1;
                    }
                    new_larry[L_VAL - 1] = called;
                } else {
                    for i in 0..L_VAL - 1 {
                        new_larry[i] = larry[i + 1];
                    }
                    new_larry[L_VAL - 1] = called;
                }

                // Update Robin's memory
                let mut new_robin = [0u8; L_VAL];
                let found_r = robin.iter().position(|&v| v == called);
                if found_r.is_some() {
                    diff_change -= 1;
                    new_robin = robin;
                } else {
                    for i in 0..L_VAL - 1 {
                        new_robin[i] = robin[i + 1];
                    }
                    new_robin[L_VAL - 1] = called;
                }

                let nk = encode_mems(&new_larry, &new_robin);
                let entry = nxt.entry(nk).or_insert_with(|| vec![0.0; DIFF_SIZE]);

                for d in 0..DIFF_SIZE {
                    if prob[d] == 0.0 { continue; }
                    let nd = d as i32 + diff_change;
                    if nd >= 0 && nd < DIFF_SIZE as i32 {
                        entry[nd as usize] += prob[d] / K_VAL as f64;
                    }
                }
            }
        }

        cur = nxt;
    }

    let mut ans = 0.0;
    for prob in cur.values() {
        for d in 0..DIFF_SIZE {
            if prob[d] != 0.0 {
                ans += (d as i32 - DIFF_OFFSET).unsigned_abs() as f64 * prob[d];
            }
        }
    }

    println!("{:.8}", ans);
}
