// Project Euler 375: Minimum of subsequences
// BBS cycle once, one monotone-stack pass, Lagrange interpolation.

const N: i64 = 2_000_000_000;
const BBS_MOD: u64 = 50_515_093;
const STACK: usize = 64;

fn main() {
    let mut st_val = [0i64; STACK];
    let mut st_cnt = [0i64; STACK];
    st_val[0] = -1;
    let mut sp = 1usize;
    let mut cur = 0i64;
    let mut total = 0i64;

    macro_rules! push {
        ($x:expr) => {{
            let x = $x;
            let mut cnt = 1i64;
            // SAFETY: sentinel at 0; depth < 40 for this PRNG
            while unsafe { *st_val.get_unchecked(sp - 1) } >= x {
                sp -= 1;
                let v = unsafe { *st_val.get_unchecked(sp) };
                let c = unsafe { *st_cnt.get_unchecked(sp) };
                cur -= v * c;
                cnt += c;
            }
            unsafe {
                *st_val.get_unchecked_mut(sp) = x;
                *st_cnt.get_unchecked_mut(sp) = cnt;
            }
            sp += 1;
            cur += x * cnt;
            total += cur;
        }};
    }

    let mut cycle = Vec::with_capacity(8_000_000);
    let mut s: u64 = 290797;
    s = s * s % BBS_MOD;
    let first = s;
    cycle.push(s as i32);
    push!(s as i64);
    loop {
        s = s * s % BBS_MOD;
        if s == first {
            break;
        }
        cycle.push(s as i32);
        push!(s as i64);
    }

    let period = cycle.len();
    let r = (N as usize) % period;
    let cptr = cycle.as_ptr();

    macro_rules! process {
        ($lo:expr, $hi:expr) => {{
            let mut i = $lo;
            let end = $hi;
            while i < end {
                push!(unsafe { *cptr.add(i) } as i64);
                i += 1;
            }
        }};
    }

    // Snapshots of M at P+r, 2P+r, 3P+r (continue from offset r, not 0).
    process!(0, r);
    let y0 = total as i128;
    process!(r, period);
    process!(0, r);
    let y1 = total as i128;
    process!(r, period);
    process!(0, r);
    let y2 = total as i128;

    let k = (N / period as i64) as i128;
    let d1 = y1 - y0;
    let d2 = y2 - 2 * y1 + y0;
    let result = y0 + d1 * (k - 1) + d2 * (k - 1) * (k - 2) / 2;
    println!("{}", result as i64);
}
