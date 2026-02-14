// Project Euler 280: Ant and Seeds
// Value iteration on 5x5 grid states.

fn popcount(x: u32) -> u32 { x.count_ones() }

fn encode(pos: usize, carry: usize, bot: u32, top: u32) -> usize {
    pos * 2048 + carry * 1024 + (bot as usize) * 32 + top as usize
}

fn main() {
    const MAX_STATES: usize = 51200;
    let mut e = vec![0.0f64; MAX_STATES];
    let mut is_valid = vec![false; MAX_STATES];
    let mut is_terminal = vec![false; MAX_STATES];

    for pos in 0..25usize {
        for carry in 0..=1usize {
            for bot in 0u32..32 {
                for top in 0u32..32 {
                    if popcount(bot) + popcount(top) + carry as u32 != 5 { continue; }
                    let idx = encode(pos, carry, bot, top);
                    is_valid[idx] = true;
                    if top == 31 { is_terminal[idx] = true; }
                }
            }
        }
    }

    for i in 0..MAX_STATES {
        if is_valid[i] && !is_terminal[i] { e[i] = 1000.0; }
    }

    let dxx: [i32; 4] = [0, 0, -1, 1];
    let dyy: [i32; 4] = [1, -1, 0, 0];

    for _ in 0..10000 {
        let mut max_change = 0.0f64;

        for pos in 0..25usize {
            let x = (pos % 5) as i32;
            let y = (pos / 5) as i32;

            for carry in 0..=1usize {
                for bot in 0u32..32 {
                    for top in 0u32..32 {
                        let idx = encode(pos, carry, bot, top);
                        if !is_valid[idx] || is_terminal[idx] { continue; }

                        let mut sum_next = 0.0f64;
                        let mut count = 0;

                        for d in 0..4 {
                            let nx = x + dxx[d];
                            let ny = y + dyy[d];
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

                            let nidx = encode(npos, ncarry, nbot, ntop);
                            sum_next += e[nidx];
                            count += 1;
                        }

                        let new_e = 1.0 + sum_next / count as f64;
                        let change = (new_e - e[idx]).abs();
                        if change > max_change { max_change = change; }
                        e[idx] = new_e;
                    }
                }
            }
        }

        if max_change < 1e-12 { break; }
    }

    let start = encode(2 * 5 + 2, 0, 31, 0);
    println!("{:.6}", e[start]);
}
