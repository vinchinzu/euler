// Project Euler 691 - Long Substring
// SA-IS suffix array + Phi LCP + union-find sweep for L(k).

trait Alpha: Copy + Ord {
    fn key(self) -> usize;
}
impl Alpha for u8 {
    #[inline(always)]
    fn key(self) -> usize {
        self as usize
    }
}
impl Alpha for i32 {
    #[inline(always)]
    fn key(self) -> usize {
        self as usize
    }
}

fn sa_naive<T: Alpha>(s: &[T], sa: &mut [i32]) {
    let n = s.len();
    for i in 0..n {
        sa[i] = i as i32;
    }
    sa.sort_unstable_by(|&l, &r| {
        let mut l = l as usize;
        let mut r = r as usize;
        if l == r {
            return std::cmp::Ordering::Equal;
        }
        while l < n && r < n {
            if s[l] != s[r] {
                return s[l].cmp(&s[r]);
            }
            l += 1;
            r += 1;
        }
        if l == n {
            std::cmp::Ordering::Less
        } else {
            std::cmp::Ordering::Greater
        }
    });
}

fn sa_doubling<T: Alpha>(s: &[T], sa: &mut [i32]) {
    let n = s.len();
    for i in 0..n {
        sa[i] = i as i32;
    }
    let mut rnk: Vec<i32> = s.iter().map(|&c| c.key() as i32).collect();
    let mut tmp = vec![0i32; n];
    let mut k = 1;
    while k < n {
        sa.sort_unstable_by(|&x, &y| {
            let x = x as usize;
            let y = y as usize;
            rnk[x].cmp(&rnk[y]).then_with(|| {
                let rx = if x + k < n { rnk[x + k] } else { -1 };
                let ry = if y + k < n { rnk[y + k] } else { -1 };
                rx.cmp(&ry)
            })
        });
        tmp[sa[0] as usize] = 0;
        let mut classes = 0i32;
        for i in 1..n {
            let x = sa[i - 1] as usize;
            let y = sa[i] as usize;
            let rx = if x + k < n { rnk[x + k] } else { -1 };
            let ry = if y + k < n { rnk[y + k] } else { -1 };
            if rnk[x] != rnk[y] || rx != ry {
                classes += 1;
            }
            tmp[y] = classes;
        }
        std::mem::swap(&mut rnk, &mut tmp);
        if classes as usize + 1 == n {
            break;
        }
        k *= 2;
    }
}

fn induce<T: Alpha>(
    s: &[T],
    sa: &mut [i32],
    ls: &[u8],
    sum_l: &[i32],
    sum_s: &[i32],
    lms: &[i32],
) {
    let n = s.len();
    sa.fill(-1);
    let mut buf = vec![0i32; sum_l.len()];
    buf.copy_from_slice(sum_s);
    unsafe {
        let s_p = s.as_ptr();
        let sa_p = sa.as_mut_ptr();
        let buf_p = buf.as_mut_ptr();
        for &d in lms {
            if d == n as i32 {
                continue;
            }
            let c = (*s_p.add(d as usize)).key();
            let p = *buf_p.add(c);
            *buf_p.add(c) = p + 1;
            *sa_p.add(p as usize) = d;
        }
        buf.copy_from_slice(sum_l);
        let last = (*s_p.add(n - 1)).key();
        let p = *buf_p.add(last);
        *buf_p.add(last) = p + 1;
        *sa_p.add(p as usize) = (n - 1) as i32;

        let ls_p = ls.as_ptr();
        for i in 0..n {
            let v = *sa_p.add(i);
            if v >= 1 && *ls_p.add((v - 1) as usize) == 0 {
                let c = (*s_p.add((v - 1) as usize)).key();
                let p = *buf_p.add(c);
                *buf_p.add(c) = p + 1;
                *sa_p.add(p as usize) = v - 1;
            }
        }
        buf.copy_from_slice(sum_l);
        for i in (0..n).rev() {
            let v = *sa_p.add(i);
            if v >= 1 && *ls_p.add((v - 1) as usize) != 0 {
                let c = (*s_p.add((v - 1) as usize)).key() + 1;
                let p = *buf_p.add(c) - 1;
                *buf_p.add(c) = p;
                *sa_p.add(p as usize) = v - 1;
            }
        }
    }
}

fn sa_is_inner<T: Alpha>(s: &[T], upper: usize, sa: &mut [i32]) {
    let n = s.len();
    if n == 0 {
        return;
    }
    if n == 1 {
        sa[0] = 0;
        return;
    }
    if n == 2 {
        if s[0] < s[1] {
            sa[0] = 0;
            sa[1] = 1;
        } else {
            sa[0] = 1;
            sa[1] = 0;
        }
        return;
    }
    if n < 10 {
        sa_naive(s, sa);
        return;
    }
    if n < 40 {
        sa_doubling(s, sa);
        return;
    }

    let mut ls = vec![0u8; n];
    for i in (0..n - 1).rev() {
        ls[i] = if s[i] == s[i + 1] {
            ls[i + 1]
        } else {
            (s[i] < s[i + 1]) as u8
        };
    }

    let mut sum_l = vec![0i32; upper + 1];
    let mut sum_s = vec![0i32; upper + 1];
    for i in 0..n {
        let c = s[i].key();
        if ls[i] == 0 {
            sum_s[c] += 1;
        } else {
            sum_l[c + 1] += 1;
        }
    }
    for i in 0..=upper {
        sum_s[i] += sum_l[i];
        if i < upper {
            sum_l[i + 1] += sum_s[i];
        }
    }

    let mut lms_map = vec![-1i32; n + 1];
    let mut lms = Vec::with_capacity(n / 2 + 1);
    for i in 1..n {
        if ls[i - 1] == 0 && ls[i] != 0 {
            lms_map[i] = lms.len() as i32;
            lms.push(i as i32);
        }
    }
    let m = lms.len();

    induce(s, sa, &ls, &sum_l, &sum_s, &lms);

    if m > 0 {
        let mut sorted_lms = Vec::with_capacity(m);
        for &v in sa.iter() {
            if lms_map[v as usize] != -1 {
                sorted_lms.push(v);
            }
        }
        let mut rec_s = vec![0i32; m];
        let mut rec_upper = 0i32;
        rec_s[lms_map[sorted_lms[0] as usize] as usize] = 0;
        for i in 1..m {
            let mut l = sorted_lms[i - 1] as usize;
            let mut r = sorted_lms[i] as usize;
            let ml = lms_map[l];
            let mr = lms_map[r];
            let end_l = if (ml as usize) + 1 < m {
                lms[(ml as usize) + 1] as usize
            } else {
                n
            };
            let end_r = if (mr as usize) + 1 < m {
                lms[(mr as usize) + 1] as usize
            } else {
                n
            };
            let mut same = true;
            if end_l - l != end_r - r {
                same = false;
            } else {
                while l < end_l {
                    if s[l] != s[r] {
                        break;
                    }
                    l += 1;
                    r += 1;
                }
                if l == n || s[l] != s[r] {
                    same = false;
                }
            }
            if !same {
                rec_upper += 1;
            }
            rec_s[lms_map[sorted_lms[i] as usize] as usize] = rec_upper;
        }

        if rec_upper as usize + 1 < m {
            let mut rec_sa = vec![0i32; m];
            sa_is_inner(&rec_s, rec_upper as usize, &mut rec_sa);
            for i in 0..m {
                sorted_lms[i] = lms[rec_sa[i] as usize];
            }
        }
        induce(s, sa, &ls, &sum_l, &sum_s, &sorted_lms);
    }
}

#[inline(always)]
unsafe fn match_len(s: &[u8], i: usize, j: usize, mut h: usize, n: usize) -> usize {
    // SAFETY: callers keep i, j < n; the 8-byte loop only loads inside [0, n).
    unsafe {
        while h + 8 <= n.wrapping_sub(i) && h + 8 <= n.wrapping_sub(j) {
            let a = std::ptr::read_unaligned(s.as_ptr().add(i + h) as *const u64);
            let b = std::ptr::read_unaligned(s.as_ptr().add(j + h) as *const u64);
            if a != b {
                let xor = a ^ b;
                let z = if cfg!(target_endian = "little") {
                    xor.trailing_zeros()
                } else {
                    xor.leading_zeros()
                };
                return h + (z as usize >> 3);
            }
            h += 8;
        }
        while i + h < n && j + h < n && *s.get_unchecked(i + h) == *s.get_unchecked(j + h) {
            h += 1;
        }
    }
    h
}

fn build_lcp(s: &[u8], sa: &[i32], lcp: &mut [i32]) {
    let n = s.len();
    let mut phi = vec![0i32; n];
    // SAFETY: sa is a permutation of 0..n.
    unsafe {
        *phi.get_unchecked_mut(*sa.get_unchecked(0) as usize) = -1;
        for i in 1..n {
            *phi.get_unchecked_mut(*sa.get_unchecked(i) as usize) = *sa.get_unchecked(i - 1);
        }
        let mut h = 0usize;
        for i in 0..n {
            let j = *phi.get_unchecked(i);
            if j < 0 {
                *phi.get_unchecked_mut(i) = 0;
                h = 0;
                continue;
            }
            h = match_len(s, i, j as usize, h, n);
            *phi.get_unchecked_mut(i) = h as i32;
            if h > 0 {
                h -= 1;
            }
        }
        *lcp.get_unchecked_mut(0) = 0;
        for i in 1..n {
            *lcp.get_unchecked_mut(i) = *phi.get_unchecked(*sa.get_unchecked(i) as usize);
        }
    }
}

#[inline(always)]
fn find(parent: &mut [i32], mut x: i32) -> i32 {
    // SAFETY: x is a UF index in 0..n; parent has length n.
    unsafe {
        while *parent.get_unchecked(x as usize) != x {
            let p = *parent.get_unchecked(x as usize);
            let gp = *parent.get_unchecked(p as usize);
            *parent.get_unchecked_mut(x as usize) = gp;
            x = gp;
        }
    }
    x
}

fn unite(parent: &mut [i32], sz: &mut [i32], a: i32, b: i32, global_max_sz: &mut i32) {
    let mut a = find(parent, a);
    let mut b = find(parent, b);
    if a == b {
        return;
    }
    unsafe {
        if *sz.get_unchecked(a as usize) < *sz.get_unchecked(b as usize) {
            std::mem::swap(&mut a, &mut b);
        }
        *parent.get_unchecked_mut(b as usize) = a;
        *sz.get_unchecked_mut(a as usize) += *sz.get_unchecked(b as usize);
        if *sz.get_unchecked(a as usize) > *global_max_sz {
            *global_max_sz = *sz.get_unchecked(a as usize);
        }
    }
}

fn main() {
    let big_n = 5_000_000usize;
    let phi: f64 = (1.0 + 5.0f64.sqrt()) / 2.0;

    // Thue-Morse XOR Beatty (Fibonacci) word; remap {0,'0','1'} -> {0,1,2}.
    let n = big_n + 1;
    let mut s = vec![0u8; n];
    let mut prev = 0i32;
    for i in 0..big_n {
        let cur = ((i + 1) as f64 / phi).floor() as i32;
        let bn = (cur - prev) as u8;
        s[i] = ((i.count_ones() as u8) & 1) ^ bn;
        s[i] += 1;
        prev = cur;
    }

    let mut sa = vec![0i32; n];
    sa_is_inner(&s, 2, &mut sa);

    let mut lcp_arr = vec![0i32; n];
    build_lcp(&s, &sa, &mut lcp_arr);
    drop(s);
    drop(sa);

    let mut max_lcp = 0i32;
    for i in 1..n {
        if lcp_arr[i] > max_lcp {
            max_lcp = lcp_arr[i];
        }
    }

    let ml = max_lcp as usize;
    let mut lcp_cnt = vec![0i32; ml + 2];
    for i in 1..n {
        if lcp_arr[i] > 0 {
            lcp_cnt[lcp_arr[i] as usize] += 1;
        }
    }

    let mut prefix_sum = vec![0i32; ml + 2];
    for v in 2..=ml {
        prefix_sum[v] = prefix_sum[v - 1] + lcp_cnt[v - 1];
    }
    let total = prefix_sum[ml] + lcp_cnt[ml];

    let mut idx_buf = vec![0i32; total as usize + 1];
    let mut pos = prefix_sum.clone();
    for i in 1..n {
        if lcp_arr[i] > 0 {
            let v = lcp_arr[i] as usize;
            idx_buf[pos[v] as usize] = i as i32;
            pos[v] += 1;
        }
    }
    drop(lcp_arr);

    let mut parent = vec![0i32; n];
    let mut sz_arr = vec![1i32; n];
    for i in 0..n {
        parent[i] = i as i32;
    }
    let mut activated = vec![0u8; n];

    let mut global_max_sz = 0i32;
    let mut big_l = vec![0i32; big_n + 2];
    big_l[1] = big_n as i32;
    let mut max_freq = 1i32;

    for v in (1..=ml).rev() {
        let si = prefix_sum[v] as usize;
        let ei = si + lcp_cnt[v] as usize;

        for jj in si..ei {
            let i = idx_buf[jj] as usize;
            activated[i] = 1;
            if global_max_sz == 0 {
                global_max_sz = 1;
            }
            if i > 1 && activated[i - 1] != 0 {
                unite(&mut parent, &mut sz_arr, i as i32, (i - 1) as i32, &mut global_max_sz);
            }
            if i + 1 < n && activated[i + 1] != 0 {
                unite(&mut parent, &mut sz_arr, i as i32, (i + 1) as i32, &mut global_max_sz);
            }
        }

        let freq = global_max_sz + 1;
        if freq > max_freq {
            big_l[(max_freq as usize + 1)..=freq as usize].fill(v as i32);
            max_freq = freq;
        }
    }

    let mut ans = 0i64;
    for k in 1..=big_n {
        ans += big_l[k] as i64;
    }
    println!("{}", ans);
}
