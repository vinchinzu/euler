// Project Euler 314: The Mouse on the Moon
// DAG shortest path + Dinkelbach to maximize area/perimeter ratio.
// Weight r*len + trapezoid area is positive. Moves only increase x or
// decrease y, so the grid is a DAG and no heap is required.

const NN: usize = 250;
const N: usize = NN + 1;
const MAX_STEP: usize = 11;

const fn gcd(mut a: usize, mut b: usize) -> usize {
    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }
    a
}

fn shortest(
    r: f64,
    step_len: &[[f64; MAX_STEP + 1]; MAX_STEP + 1],
    coprime: &[[bool; MAX_STEP + 1]; MAX_STEP + 1],
    dist: &mut [f64],
    parent: &mut [u32],
) {
    dist.fill(f64::MAX);
    dist[NN] = 0.0; // (0, NN)
    parent[NN] = NN as u32;

    for x in 0..=NN {
        let max_dx = (NN - x).min(MAX_STEP);
        for y in (0..=NN).rev() {
            if x + y > NN {
                continue;
            }
            let i = x * N + y;
            // SAFETY: x <= NN, y <= NN, so i < N*N
            let d = unsafe { *dist.get_unchecked(i) };
            if d == f64::MAX {
                continue;
            }

            let yf = y as f64;
            for dx in 0..=max_dx {
                let nx = x + dx;
                let max_dy = y.min(MAX_STEP);
                let nx_base = nx * N;
                let dxf = dx as f64;
                unsafe {
                    let sl = step_len.get_unchecked(dx);
                    let cp = coprime.get_unchecked(dx);
                    for dy in 0..=max_dy {
                        if !*cp.get_unchecked(dy) {
                            continue;
                        }
                        let ny = y - dy;
                        if nx + ny > NN {
                            continue;
                        }
                        let ni = nx_base + ny;
                        // SAFETY: dx,dy <= MAX_STEP; nx,ny <= NN so ni < N*N
                        let len = *sl.get_unchecked(dy);
                        let area = dxf * (yf + yf - dy as f64) * 0.5;
                        let nd = d + r * len + area;
                        let slot = dist.get_unchecked_mut(ni);
                        if nd < *slot {
                            *slot = nd;
                            *parent.get_unchecked_mut(ni) = i as u32;
                        }
                    }
                }
            }
        }
    }
}

fn path_ratio(
    k: f64,
    parent: &[u32],
    step_len: &[[f64; MAX_STEP + 1]; MAX_STEP + 1],
) -> f64 {
    let mut a = 0.0;
    let mut ell = 0.0;
    let mut cur = NN * N;
    let start = NN;
    while cur != start {
        let prev = parent[cur] as usize;
        let x = cur / N;
        let y = cur % N;
        let px = prev / N;
        let py = prev % N;
        let dx = x - px;
        let dy = py - y;
        ell += step_len[dx][dy];
        a += dx as f64 * (2.0 * py as f64 - dy as f64) * 0.5;
        cur = prev;
    }
    (k - a) / ell
}

fn main() {
    let mut step_len = [[0.0f64; MAX_STEP + 1]; MAX_STEP + 1];
    let mut coprime = [[false; MAX_STEP + 1]; MAX_STEP + 1];
    for dx in 0..=MAX_STEP {
        for dy in 0..=MAX_STEP {
            step_len[dx][dy] = ((dx * dx + dy * dy) as f64).sqrt();
            coprime[dx][dy] = gcd(dx, dy) == 1;
        }
    }

    let k = 500.0 * 500.0 / 4.0;
    let mut dist = vec![f64::MAX; N * N];
    let mut parent = vec![0u32; N * N];

    let mut r = 132.0_f64;
    for _ in 0..12 {
        shortest(r, &step_len, &coprime, &mut dist, &mut parent);
        let new_r = path_ratio(k, &parent, &step_len);
        if (new_r - r).abs() < 1e-12 {
            r = new_r;
            break;
        }
        r = new_r;
    }

    println!("{:.8}", r);
}
