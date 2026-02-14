const MAX_LEVEL: usize = 50;

#[derive(Clone, Copy)]
struct Summary {
    steps: i64,
    dx: i64,
    dy: i64,
    ddir: i32,
}

static DX: [i64; 4] = [0, -1, 0, 1];
static DY: [i64; 4] = [1, 0, -1, 0];

fn rotate(sx: i64, sy: i64, d: i32) -> (i64, i64) {
    match d.rem_euclid(4) {
        0 => (sx, sy),
        1 => (-sy, sx),
        2 => (-sx, -sy),
        3 => (sy, -sx),
        _ => unreachable!(),
    }
}

struct State {
    x: i64,
    y: i64,
    d: i32,
    sum_a: [Summary; MAX_LEVEL + 1],
    sum_b: [Summary; MAX_LEVEL + 1],
}

impl State {
    fn process_char(&mut self, c: char, level: usize, remaining: i64) -> i64 {
        if remaining <= 0 { return 0; }
        match c {
            'F' => {
                self.x += DX[self.d as usize];
                self.y += DY[self.d as usize];
                1
            }
            'L' => { self.d = (self.d + 1) & 3; 0 }
            'R' => { self.d = (self.d + 3) & 3; 0 }
            'a' | 'b' => {
                if level == 0 { return 0; }
                let s = if c == 'a' { &self.sum_a[level - 1] } else { &self.sum_b[level - 1] };
                let s = *s;
                if s.steps <= remaining {
                    let (rx, ry) = rotate(s.dx, s.dy, self.d);
                    self.x += rx;
                    self.y += ry;
                    self.d = (self.d + s.ddir) & 3;
                    s.steps
                } else {
                    let expansion = if c == 'a' { 1 } else { 2 };
                    self.walk(expansion, level - 1, remaining)
                }
            }
            _ => 0,
        }
    }

    fn walk(&mut self, string_type: usize, level: usize, remaining: i64) -> i64 {
        let strings: [&[u8]; 3] = [b"Fa", b"aRbFR", b"LFaLb"];
        let s = strings[string_type];
        let mut taken = 0i64;
        for &ch in s {
            if taken >= remaining { break; }
            taken += self.process_char(ch as char, level, remaining - taken);
        }
        taken
    }
}

fn main() {
    let n: i64 = 1_000_000_000_000;

    let zero_summary = Summary { steps: 0, dx: 0, dy: 0, ddir: 0 };
    let mut sum_a = [zero_summary; MAX_LEVEL + 1];
    let mut sum_b = [zero_summary; MAX_LEVEL + 1];

    for lev in 1..=MAX_LEVEL {
        // a -> aRbFR
        {
            let (mut dx, mut dy, mut d, mut steps) = (0i64, 0i64, 0i32, 0i64);
            let sa = sum_a[lev - 1];
            let (rx, ry) = rotate(sa.dx, sa.dy, d);
            dx += rx; dy += ry; d = (d + sa.ddir) & 3; steps += sa.steps;
            d = (d + 3) & 3; // R
            let sb = sum_b[lev - 1];
            let (rx, ry) = rotate(sb.dx, sb.dy, d);
            dx += rx; dy += ry; d = (d + sb.ddir) & 3; steps += sb.steps;
            dx += DX[d as usize]; dy += DY[d as usize]; steps += 1; // F
            d = (d + 3) & 3; // R
            sum_a[lev] = Summary { steps, dx, dy, ddir: d };
        }
        // b -> LFaLb
        {
            let (mut dx, mut dy, mut d, mut steps) = (0i64, 0i64, 0i32, 0i64);
            d = (d + 1) & 3; // L
            dx += DX[d as usize]; dy += DY[d as usize]; steps += 1; // F
            let sa = sum_a[lev - 1];
            let (rx, ry) = rotate(sa.dx, sa.dy, d);
            dx += rx; dy += ry; d = (d + sa.ddir) & 3; steps += sa.steps;
            d = (d + 1) & 3; // L
            let sb = sum_b[lev - 1];
            let (rx, ry) = rotate(sb.dx, sb.dy, d);
            dx += rx; dy += ry; d = (d + sb.ddir) & 3; steps += sb.steps;
            sum_b[lev] = Summary { steps, dx, dy, ddir: d };
        }
    }

    let mut state = State { x: 0, y: 0, d: 0, sum_a, sum_b };
    state.walk(0, MAX_LEVEL, n);

    println!("{},{}", state.x, state.y);
}
