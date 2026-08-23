// Project Euler 238: Infinite string tour
const S0: i64 = 14025256;
const M: i64 = 20300713;

#[inline]
fn low_bits(n: usize) -> u64 {
    if n >= 64 { u64::MAX } else { (1u64 << n) - 1 }
}

fn push_digits(mut n: i32, out: &mut Vec<u8>) {
    if n == 0 {
        out.push(0);
        return;
    }
    let mut buf = [0u8; 8];
    let mut k = 0;
    while n > 0 {
        buf[k] = (n % 10) as u8;
        n /= 10;
        k += 1;
    }
    while k > 0 {
        k -= 1;
        out.push(buf[k]);
    }
}

/// OR `src[0..nbits]` into `dst` starting at bit `dst_off`.
fn or_bits(dst: &mut [u64], mut dst_bit: usize, src: &[u64], mut nbits: usize) {
    let mut src_bit = 0usize;
    while nbits > 0 {
        let d_off = dst_bit & 63;
        let s_off = src_bit & 63;
        let take = nbits.min(64 - d_off).min(64 - s_off);
        let chunk = (src[src_bit >> 6] >> s_off) & low_bits(take);
        dst[dst_bit >> 6] |= chunk << d_off;
        dst_bit += take;
        src_bit += take;
        nbits -= take;
    }
}

/// Count bits of `word` (word index `w`) whose global index lies in `1..=r_lim`.
#[inline]
fn popcount_partial(word: u64, w: usize, r_lim: usize) -> u64 {
    if r_lim == 0 {
        return 0;
    }
    let lo = w << 6;
    let hi = lo + 63;
    if hi < 1 || lo > r_lim {
        return 0;
    }
    let start = lo.max(1);
    let end = hi.min(r_lim);
    if start > end {
        return 0;
    }
    let sl = start - lo;
    (word & (low_bits(end - start + 1) << sl)).count_ones() as u64
}

/// Cover `unknown ∩ rotate_right(S, shift)`; clear those bits from `unknown`.
/// Returns (newly covered, newly covered in 1..=r_lim).
fn cover_shift(
    s_dbl: &[u64],
    unknown: &mut [u64],
    shift: usize,
    r_lim: usize,
) -> (u64, u64) {
    let nwords = unknown.len();
    let word_off = shift >> 6;
    let bit_off = shift & 63;
    let mut total = 0u64;
    let mut partial = 0u64;
    // SAFETY: s_dbl is padded past nwords+1 at any shift < nbits; unknown has nwords.
    unsafe {
        if bit_off == 0 {
            for i in 0..nwords {
                let rot = *s_dbl.get_unchecked(word_off + i);
                let unk = unknown.get_unchecked_mut(i);
                let neww = rot & *unk;
                if neww != 0 {
                    *unk ^= neww;
                    total += neww.count_ones() as u64;
                    partial += popcount_partial(neww, i, r_lim);
                }
            }
        } else {
            let r = 64 - bit_off;
            for i in 0..nwords {
                let rot = (*s_dbl.get_unchecked(word_off + i) >> bit_off)
                    | (*s_dbl.get_unchecked(word_off + i + 1) << r);
                let unk = unknown.get_unchecked_mut(i);
                let neww = rot & *unk;
                if neww != 0 {
                    *unk ^= neww;
                    total += neww.count_ones() as u64;
                    partial += popcount_partial(neww, i, r_lim);
                }
            }
        }
    }
    (total, partial)
}

fn collect_set_bits(bits: &[u64], nbits: usize, out: &mut Vec<u32>) {
    out.clear();
    for (w, &word) in bits.iter().enumerate() {
        let mut x = word;
        while x != 0 {
            let b = x.trailing_zeros();
            let idx = (w << 6) + b as usize;
            if idx < nbits {
                out.push(idx as u32);
            }
            x &= x - 1;
        }
    }
}

struct Coverer {
    s_bits: Vec<u64>,
    s_dbl: Vec<u64>,
    unknown: Vec<u64>,
    compact: Option<Vec<u32>>,
    d: usize,
    r_lim: usize,
    unknown_count: u64,
    steps: u32,
    total_f: i64,
    partial_g: i64,
}

impl Coverer {
    fn done(&self) -> bool {
        self.unknown_count == 0
    }

    fn apply(&mut self, v: usize, f_val: i64) {
        if self.unknown_count == 0 {
            return;
        }
        const BITSET_STEPS: u32 = 16;
        const COMPACT_AT: u64 = 2_000_000;

        if let Some(list) = self.compact.as_mut() {
            let mut nkeep = 0usize;
            let mut hits = 0i64;
            let mut hits_p = 0i64;
            let d = self.d;
            let r_lim = self.r_lim;
            let s_bits = self.s_bits.as_slice();
            for i in 0..list.len() {
                let r = unsafe { *list.get_unchecked(i) };
                let mut t = r as usize + v;
                if t >= d {
                    t -= d;
                }
                let in_s = unsafe { (*s_bits.get_unchecked(t >> 6) >> (t & 63)) & 1 != 0 };
                if in_s {
                    hits += 1;
                    if r >= 1 && (r as usize) <= r_lim {
                        hits_p += 1;
                    }
                } else {
                    unsafe {
                        *list.get_unchecked_mut(nkeep) = r;
                    }
                    nkeep += 1;
                }
            }
            list.truncate(nkeep);
            self.unknown_count = nkeep as u64;
            self.total_f += f_val * hits;
            self.partial_g += f_val * hits_p;
            return;
        }

        let (new_total, new_partial) =
            cover_shift(&self.s_dbl, &mut self.unknown, v, self.r_lim);
        if new_total != 0 {
            self.total_f += f_val * new_total as i64;
            self.partial_g += f_val * new_partial as i64;
            self.unknown_count -= new_total;
        }
        self.steps += 1;
        if self.unknown_count > 0 && (self.unknown_count <= COMPACT_AT || self.steps >= BITSET_STEPS)
        {
            let mut list = Vec::with_capacity(self.unknown_count as usize);
            collect_set_bits(&self.unknown, self.d, &mut list);
            self.unknown_count = list.len() as u64;
            self.compact = Some(list);
        }
    }
}

fn main() {
    let big_n: i64 = 2_000_000_000_000_000;
    let max_period: usize = 3_000_000;

    let mut digits = Vec::with_capacity(22_000_000);
    let mut s = S0;
    loop {
        push_digits(s as i32, &mut digits);
        s = s * s % M;
        if s == S0 || digits.len() > max_period * 8 {
            break;
        }
    }

    let d = digits.iter().map(|&x| x as usize).sum::<usize>();
    let nwords = (d + 63) >> 6;
    let r_lim = (big_n % d as i64) as usize;
    let q = big_n / d as i64;

    let mut s_bits = vec![0u64; nwords];
    {
        let mut cum = 0usize;
        // SAFETY: cum stays in 0..d, so the word index is in 0..nwords.
        unsafe {
            *s_bits.get_unchecked_mut(0) |= 1;
            for &dig in &digits {
                cum += dig as usize;
                if cum < d {
                    *s_bits.get_unchecked_mut(cum >> 6) |= 1u64 << (cum & 63);
                }
            }
        }
    }

    let mut s_dbl = vec![0u64; ((2 * d + 63) >> 6) + 2];
    s_dbl[..nwords].copy_from_slice(&s_bits);
    or_bits(&mut s_dbl, d, &s_bits, d);

    let mut unknown = vec![u64::MAX; nwords];
    let rem_bits = d & 63;
    if rem_bits != 0 {
        unknown[nwords - 1] = low_bits(rem_bits);
    }

    let mut cov = Coverer {
        s_bits,
        s_dbl,
        unknown,
        compact: None,
        d,
        r_lim,
        unknown_count: d as u64,
        steps: 0,
        total_f: 0,
        partial_g: 0,
    };

    cov.apply(0, 1);
    let mut cum = 0usize;
    for (idx, &dig) in digits.iter().enumerate() {
        if cov.done() {
            break;
        }
        cum += dig as usize;
        if dig != 0 && cum < d {
            cov.apply(cum, (idx + 2) as i64);
        }
    }

    let answer = q * cov.total_f + cov.partial_g;
    println!("{answer}");
}
