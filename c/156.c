/*
 * Project Euler 156 - Counting Digits
 *
 * f(n, d) = number of times digit d appears in 1..n
 * Find sum of all n where f(n, d) = n, for d = 1..9.
 * Uses recursive divide-and-conquer approach.
 */
#include <stdio.h>

typedef long long ll;

static int count_single(int d, ll v) {
    if (v == 0 && d == 0) return 1;
    int res = 0;
    while (v > 0) {
        if (v % 10 == d) res++;
        v /= 10;
    }
    return res;
}

static ll count_digit(int d, ll v) {
    if (v < 0) return 0;
    if (v < 10) return (v >= d) ? 1 : 0;

    ll base = 10;
    ll shift = 1;
    ll multi = 0;
    while (shift * base <= v) {
        shift *= base;
        multi += 1;
    }
    multi *= shift / base;

    ll first = v / shift;
    ll rem = v % shift;
    ll res = first * multi + count_digit(d, rem);
    if (d == first) res += rem + 1;
    if (d < first && d > 0) res += shift;
    return res;
}

static ll find_all(int d, ll fr, ll to_n) {
    ll center = (fr + to_n) / 2;
    if (fr == center) {
        return (count_digit(d, fr) == fr) ? fr : 0;
    }

    ll result = 0;
    ll count_fr_val = count_digit(d, fr);
    ll cur_fr = fr;
    ll cur_count = count_fr_val;

    while (cur_count == cur_fr && cur_fr < to_n) {
        result += cur_fr;
        cur_fr++;
        cur_count += count_single(d, cur_fr);
    }
    if (cur_fr >= to_n + 1) return result;

    fr = cur_fr;
    count_fr_val = cur_count;
    center = (fr + to_n) / 2;
    ll count_center = count_digit(d, center);
    ll count_to = count_digit(d, to_n);

    if (count_center >= fr && center >= count_fr_val && center > fr) {
        result += find_all(d, fr, center);
    }
    if (count_to >= center && to_n >= count_center && center < to_n) {
        result += find_all(d, center, to_n);
    }
    return result;
}

int main(void) {
    ll max_n = 1000000000000LL; /* 10^12 */
    ll total = 0;
    for (int d = 1; d <= 9; d++) {
        total += find_all(d, 0, max_n);
    }
    printf("%lld\n", total);
    return 0;
}
