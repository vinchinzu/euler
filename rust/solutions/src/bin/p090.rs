// Project Euler 90: Cube digit pairs
// Count valid arrangements of two dice that can display all 2-digit squares.

use itertools::Itertools;

fn cube_has(cube: &[u8], d: u8) -> bool {
    for &face in cube {
        if face == d {
            return true;
        }
        // 6 and 9 are interchangeable
        if (d == 6 || d == 9) && (face == 6 || face == 9) {
            return true;
        }
    }
    false
}

fn can_display_all(c1: &[u8], c2: &[u8]) -> bool {
    let squares: [(u8, u8); 9] = [
        (0, 1), (0, 4), (0, 9), (1, 6), (2, 5),
        (3, 6), (4, 9), (6, 4), (8, 1),
    ];
    for &(d1, d2) in &squares {
        let ok = (cube_has(c1, d1) && cube_has(c2, d2))
              || (cube_has(c1, d2) && cube_has(c2, d1));
        if !ok {
            return false;
        }
    }
    true
}

fn main() {
    let combos: Vec<Vec<u8>> = (0u8..=9).combinations(6).collect();
    let mut count = 0;

    for i in 0..combos.len() {
        for j in i..combos.len() {
            if can_display_all(&combos[i], &combos[j]) {
                count += 1;
            }
        }
    }

    println!("{count}");
}
