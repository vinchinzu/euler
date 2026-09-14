// Project Euler 280: Ant and Seeds
// Value iteration on 5x5 grid states.

#[inline(always)]
const fn encode(pos: usize, carry: usize, bot: u32, top: u32) -> usize {
    (pos << 11) | (carry << 10) | ((bot as usize) << 5) | (top as usize)
}

fn main() {
    const MAX_STATES: usize = 51200;
    let mut e = vec![0.0f64; MAX_STATES];
    let mut valid_states = Vec::with_capacity(4000);
    let mut transitions = Vec::with_capacity(4000);

    for pos in 0..25 {
        let x = (pos % 5) as i32;
        let y = (pos / 5) as i32;
        for carry in 0..=1usize {
            for bot in 0u32..32 {
                let bot_count = bot.count_ones();
                for top in 0u32..32 {
                    if bot_count + top.count_ones() + carry as u32 != 5 { continue; }
                    let idx = encode(pos, carry, bot, top);
                    if top != 31 {
                        e[idx] = 1000.0;
                        
                        let mut neighbors = [0usize; 4];
                        let mut n_count = 0u8;
                        
                        const DX: [i32; 4] = [0, 0, -1, 1];
                        const DY: [i32; 4] = [1, -1, 0, 0];
                        
                        for d in 0..4 {
                            let nx = x + DX[d];
                            let ny = y + DY[d];
                            if nx < 0 || nx >= 5 || ny < 0 || ny >= 5 { continue; }

                            let npos = (ny * 5 + nx) as usize;
                            let mut ncarry = carry;
                            let mut nbot = bot;
                            let mut ntop = top;

                            if ny == 0 && ncarry == 0 && (nbot & (1 << nx)) != 0 {
                                ncarry = 1;
                                nbot &= !(1 << nx);
                            } else if ny == 4 && ncarry == 1 && (ntop & (1 << nx)) == 0 {
                                ncarry = 0;
                                ntop |= 1 << nx;
                            }

                            neighbors[n_count as usize] = encode(npos, ncarry, nbot, ntop);
                            n_count += 1;
                        }
                        
                        valid_states.push(idx);
                        transitions.push((neighbors, n_count));
                    }
                }
            }
        }
    }

    for _ in 0..10000 {
        let mut max_change = 0.0f64;

        for i in 0..valid_states.len() {
            let idx = valid_states[i];
            let (neighbors, count) = transitions[i];
            
            let mut sum_next = 0.0f64;
            // SAFETY: All neighbor indices < MAX_STATES by construction
            for j in 0..count as usize {
                sum_next += unsafe { *e.get_unchecked(neighbors[j]) };
            }

            let new_e = 1.0 + sum_next / count as f64;
            // SAFETY: idx < MAX_STATES by construction
            let old_e = unsafe { *e.get_unchecked(idx) };
            let change = (new_e - old_e).abs();
            if change > max_change { max_change = change; }
            unsafe { *e.get_unchecked_mut(idx) = new_e; }
        }

        if max_change < 1e-12 { break; }
    }

    let start = encode(12, 0, 31, 0);
    println!("{:.6}", e[start]);
}
