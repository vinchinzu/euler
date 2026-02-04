#!/usr/bin/env python3
"""Project Euler Problem 305 - Reflexive Position

S = "123456789101112131415..."  (Champernowne string)
f(n) = starting position of the n-th occurrence of n in S.
Find sum of f(3^k) for k=1..13.

Uses C for performance: digit DP with KMP automaton for non-spanning
occurrences (with multiplicity), and direct counting for spanning
occurrences across consecutive number boundaries.
"""
import subprocess, tempfile, os

def solve():
    c_code = r"""
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

typedef long long ll;

int pat[20];
int pat_len;
int kmp_fail[20];
int kmp_trans[20][10];

void build_kmp_table_v2(void) {
    kmp_fail[0] = 0;
    for (int i = 1; i < pat_len; i++) {
        int j = kmp_fail[i-1];
        while (j > 0 && pat[j] != pat[i]) j = kmp_fail[j-1];
        if (pat[j] == pat[i]) j++;
        kmp_fail[i] = j;
    }
    for (int state = 0; state < pat_len; state++) {
        for (int c = 0; c < 10; c++) {
            int j = state;
            while (j > 0 && pat[j] != c) j = kmp_fail[j-1];
            if (pat[j] == c) j++;
            kmp_trans[state][c] = j;
        }
    }
    int fallback = kmp_fail[pat_len - 1];
    for (int c = 0; c < 10; c++) {
        int j = fallback;
        while (j > 0 && pat[j] != c) j = kmp_fail[j-1];
        if (pat[j] == c) j++;
        kmp_trans[pat_len][c] = j;
    }
}

typedef struct { ll cnt; ll matches; } dp_pair;
dp_pair dp2_memo[20][2][20];
int dp2_valid[20][2][20];
int dp2_epoch;
int upper_digits[20];
int num_digits;

dp_pair digit_dp2(int pos, int tight, int kmp_state) {
    if (pos == num_digits) {
        dp_pair r = {1, 0};
        return r;
    }
    if (dp2_valid[pos][tight][kmp_state] == dp2_epoch)
        return dp2_memo[pos][tight][kmp_state];
    int limit = tight ? upper_digits[pos] : 9;
    int start = (pos == 0) ? 1 : 0;
    dp_pair result = {0, 0};
    for (int d = start; d <= limit; d++) {
        int new_tight = tight && (d == limit);
        int new_kmp = kmp_trans[kmp_state][d];
        int match_here = (new_kmp == pat_len) ? 1 : 0;
        dp_pair sub = digit_dp2(pos + 1, new_tight, new_kmp);
        result.cnt += sub.cnt;
        result.matches += sub.matches + match_here * sub.cnt;
    }
    dp2_valid[pos][tight][kmp_state] = dp2_epoch;
    dp2_memo[pos][tight][kmp_state] = result;
    return result;
}

ll count_nonspanning_d_digits(int d) {
    if (d < pat_len) return 0;
    num_digits = d;
    for (int i = 0; i < d; i++) upper_digits[i] = 9;
    dp2_epoch++;
    return digit_dp2(0, 0, 0).matches;
}

ll count_nonspanning_up_to_m(int d, ll m) {
    if (d < pat_len) return 0;
    num_digits = d;
    char buf[25]; sprintf(buf, "%lld", m);
    for (int i = 0; i < d; i++) upper_digits[i] = buf[i] - '0';
    dp2_epoch++;
    return digit_dp2(0, 1, 0).matches;
}

ll power10[20];

ll count_spanning_for_split(int a, int b, ll m_val) {
    if (m_val < 2) return 0;
    ll P = 0;
    for (int i = 0; i < a; i++) P = P * 10 + pat[i];
    ll R = (P + 1) % power10[a];
    int carry = (P + 1 >= power10[a]) ? 1 : 0;
    ll Q = 0;
    for (int i = a; i < pat_len; i++) Q = Q * 10 + pat[i];

    ll total = 0;
    char mbuf[25]; sprintf(mbuf, "%lld", m_val);
    int max_d2 = strlen(mbuf);

    for (int d2 = 1; d2 <= max_d2; d2++) {
        if (d2 < b) continue;
        ll x_lo = (d2 == 1) ? 2 : power10[d2-1];
        if (x_lo < 2) x_lo = 2;
        ll x_hi = (d2 < max_d2) ? power10[d2] - 1 : m_val;
        if (x_hi > m_val) x_hi = m_val;
        if (x_lo > x_hi) continue;

        if (d2 < a + b) {
            int overlap_start = d2 - a;
            char qbuf[20], rbuf[20];
            sprintf(qbuf, "%0*lld", b, Q);
            sprintf(rbuf, "%0*lld", a, R);
            if (carry) continue;
            int consistent = 1;
            for (int i = overlap_start; i < b; i++) {
                if (qbuf[i] != rbuf[i - overlap_start]) { consistent = 0; break; }
            }
            if (!consistent) continue;
            char xbuf[20];
            for (int i = 0; i < b; i++) xbuf[i] = qbuf[i];
            for (int j = 0; j < a; j++) {
                int pos = d2 - a + j;
                if (pos >= b) xbuf[pos] = rbuf[j];
            }
            xbuf[d2] = '\0';
            ll x = 0;
            for (int i = 0; i < d2; i++) x = x * 10 + (xbuf[i] - '0');
            if (x >= x_lo && x <= x_hi) {
                ll n_val = x - 1;
                if (n_val >= 1 && n_val % power10[a] == P) total++;
            }
            continue;
        }

        int mid_len = d2 - a - b;
        if (!carry) {
            if (b == 1 && Q == 0) continue;
            ll base = Q * power10[mid_len + a] + R;
            ll mid_lo = 0, mid_hi = power10[mid_len] - 1;
            if (base + mid_lo * power10[a] < x_lo)
                mid_lo = (x_lo - base + power10[a] - 1) / power10[a];
            if (base + mid_hi * power10[a] > x_hi)
                mid_hi = (x_hi - base) / power10[a];
            if (mid_lo > mid_hi) continue;
            total += mid_hi - mid_lo + 1;
        } else {
            if (b == 1 && Q == 0) continue;
            ll base = Q * power10[mid_len + a];
            ll mid_lo = 0, mid_hi = power10[mid_len] - 1;
            if (base + mid_lo * power10[a] < x_lo)
                mid_lo = (x_lo - base + power10[a] - 1) / power10[a];
            if (base + mid_hi * power10[a] > x_hi)
                mid_hi = (x_hi - base) / power10[a];
            if (mid_lo > mid_hi) continue;
            total += mid_hi - mid_lo + 1;
        }
    }
    return total;
}

ll count_occurrences(ll m) {
    if (m < 1) return 0;
    char buf[25]; sprintf(buf, "%lld", m);
    int d_m = strlen(buf);
    ll total = 0;
    for (int d = pat_len; d < d_m; d++)
        total += count_nonspanning_d_digits(d);
    if (d_m >= pat_len)
        total += count_nonspanning_up_to_m(d_m, m);
    for (int a = 1; a < pat_len; a++)
        total += count_spanning_for_split(a, pat_len - a, m);
    return total;
}

ll count_digits_up_to(ll n) {
    if (n < 1) return 0;
    ll total = 0, d = 1, first = 1;
    while (first <= n) {
        ll last = first * 10 - 1;
        if (last > n) last = n;
        total += (last - first + 1) * d;
        d++; first *= 10;
    }
    return total;
}

ll find_nth_occurrence(ll target, ll n) {
    char buf[25]; sprintf(buf, "%lld", target);
    pat_len = strlen(buf);
    for (int i = 0; i < pat_len; i++) pat[i] = buf[i] - '0';
    build_kmp_table_v2();

    ll lo = 1, hi = 10000000000000LL;
    while (lo < hi) {
        ll mid = lo + (hi - lo) / 2;
        if (count_occurrences(mid) >= n) hi = mid;
        else lo = mid + 1;
    }

    ll prev_count = count_occurrences(lo - 1);
    ll remaining = n - prev_count;

    char window[100]; int wlen = 0;
    ll nums[3] = {lo-1, lo, lo+1};
    int starts[3];
    for (int i = 0; i < 3; i++) {
        starts[i] = wlen;
        if (nums[i] >= 1) {
            char nb[25]; sprintf(nb, "%lld", nums[i]);
            int nbl = strlen(nb);
            for (int j = 0; j < nbl; j++) window[wlen++] = nb[j];
        }
    }
    window[wlen] = '\0';

    ll pos_base = count_digits_up_to(lo - 2) + 1;
    for (int i = 0; i <= wlen - pat_len; i++) {
        int match = 1;
        for (int j = 0; j < pat_len; j++)
            if (window[i+j] - '0' != pat[j]) { match = 0; break; }
        if (match) {
            if (i + pat_len <= starts[1]) continue;
            remaining--;
            if (remaining == 0) return pos_base + i;
        }
    }
    return -1;
}

int main(void) {
    power10[0] = 1;
    for (int i = 1; i < 20; i++) power10[i] = power10[i-1] * 10;

    ll total = 0, p3 = 1;
    for (int k = 1; k <= 13; k++) {
        p3 *= 3;
        total += find_nth_occurrence(p3, p3);
    }
    printf("%lld\n", total);
    return 0;
}
""";

    with tempfile.NamedTemporaryFile(suffix='.c', mode='w', delete=False) as f:
        f.write(c_code)
        c_file = f.name
    exe_file = c_file.replace('.c', '')
    try:
        subprocess.run(['gcc', '-O2', '-o', exe_file, c_file, '-lm'],
                      check=True, capture_output=True)
        result = subprocess.run([exe_file], capture_output=True, text=True, timeout=30)
        print(result.stdout.strip())
    finally:
        os.unlink(c_file)
        if os.path.exists(exe_file):
            os.unlink(exe_file)

solve()
