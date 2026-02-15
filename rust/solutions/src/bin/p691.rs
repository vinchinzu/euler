// Project Euler 691 - Long Substring
// Suffix array + LCP array + union-find sweep for L(k).

const MAXN: usize = 5_000_002;

fn build_sa(s: &[u8], n: usize, sa: &mut [i32], rnk: &mut [i32]) {
    let mut cnt = vec![0usize; n.max(257)];
    let mut tmp = vec![0i32; n];

    for i in 0..n { sa[i] = i as i32; rnk[i] = s[i] as i32; }

    // Initial radix sort
    let max_val = 256;
    for v in cnt.iter_mut().take(max_val) { *v = 0; }
    for i in 0..n { cnt[rnk[i] as usize] += 1; }
    for i in 1..max_val { cnt[i] += cnt[i - 1]; }
    for i in (0..n).rev() { cnt[rnk[i] as usize] -= 1; sa[cnt[rnk[i] as usize]] = i as i32; }

    let mut k = 1;
    while k < n {
        let mut p = 0;
        for i in (n - k)..n { tmp[p] = i as i32; p += 1; }
        for i in 0..n { if sa[i] >= k as i32 { tmp[p] = sa[i] - k as i32; p += 1; } }

        let mut max_val = 0i32;
        for i in 0..n { if rnk[i] > max_val { max_val = rnk[i]; } }
        let mv = (max_val + 1) as usize;
        for v in cnt.iter_mut().take(mv) { *v = 0; }
        for i in 0..n { cnt[rnk[tmp[i] as usize] as usize] += 1; }
        for i in 1..mv { cnt[i] += cnt[i - 1]; }
        for i in (0..n).rev() { cnt[rnk[tmp[i] as usize] as usize] -= 1; sa[cnt[rnk[tmp[i] as usize] as usize]] = tmp[i]; }

        tmp[sa[0] as usize] = 0;
        let mut classes = 1i32;
        for i in 1..n {
            let s1 = sa[i - 1] as usize;
            let s2 = sa[i] as usize;
            let r1a = rnk[s1];
            let r1b = if s1 + k < n { rnk[s1 + k] } else { -1 };
            let r2a = rnk[s2];
            let r2b = if s2 + k < n { rnk[s2 + k] } else { -1 };
            if r1a != r2a || r1b != r2b { classes += 1; }
            tmp[sa[i] as usize] = classes - 1;
        }
        rnk[..n].copy_from_slice(&tmp[..n]);
        if classes as usize == n { break; }
        k <<= 1;
    }
}

fn build_lcp(s: &[u8], n: usize, sa: &[i32], lcp_arr: &mut [i32]) {
    let mut rank = vec![0i32; n];
    for i in 0..n { rank[sa[i] as usize] = i as i32; }
    let mut h = 0usize;
    for i in 0..n {
        if rank[i] > 0 {
            let j = sa[(rank[i] - 1) as usize] as usize;
            while i + h < n && j + h < n && s[i + h] == s[j + h] { h += 1; }
            lcp_arr[rank[i] as usize] = h as i32;
            if h > 0 { h -= 1; }
        } else {
            lcp_arr[0] = 0;
            h = 0;
        }
    }
}

fn main() {
    let big_n = 5_000_000usize;
    let phi: f64 = (1.0 + 5.0f64.sqrt()) / 2.0;

    // Build sequence
    let mut a = vec![0u8; big_n + 1];
    a[0] = 0;
    for i in 1..=big_n {
        a[i] = if i % 2 == 0 { a[i / 2] } else { 1 - a[i / 2] };
    }

    let mut s = vec![0u8; big_n + 1];
    for i in 0..big_n {
        let bn = ((i + 1) as f64 / phi).floor() as i32 - (i as f64 / phi).floor() as i32;
        s[i] = (a[i] ^ bn as u8) + b'0';
    }
    s[big_n] = 0;
    drop(a);

    let n = big_n + 1;
    let mut sa = vec![0i32; n];
    let mut rnk = vec![0i32; n];
    build_sa(&s, n, &mut sa, &mut rnk);

    let mut lcp_arr = vec![0i32; n];
    build_lcp(&s, n, &sa, &mut lcp_arr);
    drop(s);

    let mut max_lcp = 0i32;
    for i in 1..n {
        if lcp_arr[i] > max_lcp { max_lcp = lcp_arr[i]; }
    }

    // Count LCP values
    let ml = max_lcp as usize;
    let mut lcp_cnt = vec![0i32; ml + 2];
    for i in 1..n {
        if lcp_arr[i] > 0 { lcp_cnt[lcp_arr[i] as usize] += 1; }
    }

    let mut prefix_sum = vec![0i32; ml + 2];
    for v in 2..=ml {
        prefix_sum[v] = prefix_sum[v - 1] + lcp_cnt[v - 1];
    }
    let total = prefix_sum[ml] + lcp_cnt[ml];

    // Bucket sort LCP indices
    let mut idx_buf = vec![0i32; total as usize + 1];
    let mut pos = prefix_sum.clone();
    for i in 1..n {
        if lcp_arr[i] > 0 {
            let v = lcp_arr[i] as usize;
            idx_buf[pos[v] as usize] = i as i32;
            pos[v] += 1;
        }
    }

    // Union-find
    let mut parent = vec![0i32; n];
    let mut sz_arr = vec![1i32; n];
    for i in 0..n { parent[i] = i as i32; }
    let mut activated = vec![false; n];

    fn find(parent: &mut [i32], mut x: i32) -> i32 {
        while parent[x as usize] != x {
            parent[x as usize] = parent[parent[x as usize] as usize];
            x = parent[x as usize];
        }
        x
    }

    let mut global_max_sz = 0i32;

    let mut big_l = vec![0i64; big_n + 2];
    big_l[1] = big_n as i64;
    let mut max_freq = 1i32;

    for v in (1..=ml).rev() {
        let si = prefix_sum[v] as usize;
        let ei = si + lcp_cnt[v] as usize;

        for jj in si..ei {
            let i = idx_buf[jj] as usize;
            activated[i] = true;
            if global_max_sz == 0 { global_max_sz = 1; }
            if i > 1 && activated[i - 1] {
                let a = find(&mut parent, i as i32);
                let b = find(&mut parent, (i - 1) as i32);
                if a != b {
                    let (a, b) = if sz_arr[a as usize] >= sz_arr[b as usize] { (a, b) } else { (b, a) };
                    parent[b as usize] = a;
                    sz_arr[a as usize] += sz_arr[b as usize];
                    if sz_arr[a as usize] > global_max_sz { global_max_sz = sz_arr[a as usize]; }
                }
            }
            if i + 1 < n && activated[i + 1] {
                let a = find(&mut parent, i as i32);
                let b = find(&mut parent, (i + 1) as i32);
                if a != b {
                    let (a, b) = if sz_arr[a as usize] >= sz_arr[b as usize] { (a, b) } else { (b, a) };
                    parent[b as usize] = a;
                    sz_arr[a as usize] += sz_arr[b as usize];
                    if sz_arr[a as usize] > global_max_sz { global_max_sz = sz_arr[a as usize]; }
                }
            }
        }

        let freq = global_max_sz + 1;
        if freq > max_freq {
            for k in (max_freq + 1)..=freq {
                big_l[k as usize] = v as i64;
            }
            max_freq = freq;
        }
    }

    let mut ans = 0i64;
    for k in 1..=big_n {
        ans += big_l[k];
    }

    println!("{}", ans);
}
