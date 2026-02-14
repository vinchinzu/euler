// Project Euler 68: Magic 5-gon ring
// Find the maximum 16-digit string for a magic 5-gon ring using digits 1-10.

fn main() {
    let mut best = String::new();

    // outer[0..5] are the external nodes, inner[0..5] are the internal ring nodes.
    // Lines: (outer[i], inner[i], inner[(i+1)%5]) for i = 0..5
    // For a 16-digit string, 10 must be an outer node.
    let mut perm = [0u8; 10];
    let mut used = [false; 11];

    fn generate(
        depth: usize,
        perm: &mut [u8; 10],
        used: &mut [bool; 11],
        best: &mut String,
    ) {
        if depth == 10 {
            check(perm, best);
            return;
        }
        for v in 1..=10u8 {
            if !used[v as usize] {
                used[v as usize] = true;
                perm[depth] = v;
                generate(depth + 1, perm, used, best);
                used[v as usize] = false;
            }
        }
    }

    fn check(perm: &[u8; 10], best: &mut String) {
        let outer = &perm[0..5];
        let inner = &perm[5..10];

        // 10 must be in outer for 16-digit string
        if !outer.contains(&10) { return; }

        let target = outer[0] as u32 + inner[0] as u32 + inner[1] as u32;
        for i in 1..5 {
            if outer[i] as u32 + inner[i] as u32 + inner[(i + 1) % 5] as u32 != target {
                return;
            }
        }

        // Find start: minimum outer node
        let start = (0..5).min_by_key(|&i| outer[i]).unwrap();

        let mut candidate = String::new();
        for i in 0..5 {
            let idx = (start + i) % 5;
            candidate.push_str(&outer[idx].to_string());
            candidate.push_str(&inner[idx].to_string());
            candidate.push_str(&inner[(idx + 1) % 5].to_string());
        }

        if candidate.len() != 16 { return; }

        if candidate > *best {
            *best = candidate;
        }
    }

    generate(0, &mut perm, &mut used, &mut best);
    println!("{best}");
}
