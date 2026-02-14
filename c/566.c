/*
 * Project Euler Problem 566: Cake Cutting
 *
 * Given a circular cake with icing on top, if we repeatedly cut slices of
 * lengths 360/a, 360/b, 360/sqrt(c), each time flipping, find F(a,b,c) =
 * minimum flips until all icing returns to top.
 * Compute sum F(a,b,c) for 9 <= a < b < c <= 53.
 *
 * This is a direct C translation of the Python solution using Rad (a+b*sqrt_c)/d
 * representation, generalized CRT, and interval-based region tracking.
 */
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <math.h>

#define MAX_POSITIONS 200000
#define MAX_INTERVALS 50000
#define MAX_ORDERS 100000

static double glob_sqrt_c;
static long long glob_d;

/* Rad represents (a + b*sqrt_c)/d */
typedef struct { long long a; long long b; } Rad;

static Rad rad_add(Rad x, Rad y) { return (Rad){x.a + y.a, x.b + y.b}; }
static Rad rad_sub(Rad x, Rad y) { return (Rad){x.a - y.a, x.b - y.b}; }

static double rad_val(Rad x) { return x.a + x.b * glob_sqrt_c; }

static int rad_cmp(Rad x, Rad y) {
    double vx = rad_val(x), vy = rad_val(y);
    if (vx < vy) return -1;
    if (vx > vy) return 1;
    return 0;
}

static Rad rad_min(Rad x, Rad y) { return rad_cmp(x, y) <= 0 ? x : y; }
static Rad rad_max(Rad x, Rad y) { return rad_cmp(x, y) >= 0 ? x : y; }
static Rad rad_one(void) { return (Rad){glob_d, 0}; }
static Rad rad_zero(void) { return (Rad){0, 0}; }

typedef struct { Rad angle; int flipped; } Pos;

static int is_sq(int n) {
    int r = (int)sqrt((double)n);
    if (r * r == n) return 1;
    if ((r+1)*(r+1) == n) return 1;
    return 0;
}

static int isqrt_i(int n) {
    int r = (int)sqrt((double)n);
    while (r * r > n) r--;
    while ((r+1)*(r+1) <= n) r++;
    return r;
}

static long long gcd_ll(long long a, long long b) {
    if (a < 0) a = -a;
    if (b < 0) b = -b;
    while (b) { long long t = b; b = a % b; a = t; }
    return a;
}

static long long lcm_ll(long long a, long long b) {
    return a / gcd_ll(a, b) * b;
}

static long long mod_ll(long long a, long long m) {
    return ((a % m) + m) % m;
}

/* Extended GCD returning (x, y) such that a*x + b*y = gcd(a,b) */
static void ext_gcd(long long a, long long b, long long *x, long long *y) {
    if (b == 0) { *x = 1; *y = 0; return; }
    long long x1, y1;
    ext_gcd(b, a % b, &x1, &y1);
    *x = y1;
    *y = x1 - (a / b) * y1;
}

/* General CRT: find x such that x = a1 (mod m1) and x = a2 (mod m2) */
static long long general_crt(long long a1, long long m1, long long a2, long long m2) {
    long long g = gcd_ll(m1, m2);
    if ((a1 - a2) % g != 0) return -1;
    long long lx, ly;
    ext_gcd(m1, m2, &lx, &ly);
    long long lcm = m1 / g * m2;
    return mod_ll(a1 - m1 * ((__int128)lx * ((a1 - a2) / g) % lcm), lcm);
}

/* Intervals for processed regions */
typedef struct { Rad start; Rad end; } Interval;

static Interval intervals[MAX_INTERVALS];
static int n_intervals;
static Rad current_point;

static void intervals_init(void) {
    n_intervals = 0;
    current_point = rad_zero();
}

static void advance_to_next_unprocessed(void) {
    for (int i = 0; i < n_intervals; i++) {
        if (rad_cmp(intervals[i].start, current_point) <= 0 &&
            rad_cmp(current_point, intervals[i].end) < 0) {
            current_point = intervals[i].end;
            return;
        }
    }
}

static void process_interval(Rad start, Rad end) {
    Rad ms = start, me = end;
    Interval new_ints[MAX_INTERVALS];
    int nn = 0;
    for (int i = 0; i < n_intervals; i++) {
        /* Check overlap */
        if (rad_cmp(intervals[i].end, start) < 0 || rad_cmp(end, intervals[i].start) < 0) {
            new_ints[nn++] = intervals[i];
        } else {
            ms = rad_min(ms, intervals[i].start);
            me = rad_max(me, intervals[i].end);
        }
    }
    new_ints[nn++] = (Interval){ms, me};
    /* Sort by start */
    for (int i = 0; i < nn - 1; i++)
        for (int j = i + 1; j < nn; j++)
            if (rad_cmp(new_ints[i].start, new_ints[j].start) > 0) {
                Interval tmp = new_ints[i]; new_ints[i] = new_ints[j]; new_ints[j] = tmp;
            }
    memcpy(intervals, new_ints, nn * sizeof(Interval));
    n_intervals = nn;
}

static int intervals_done(void) {
    return rad_cmp(current_point, rad_one()) >= 0;
}

/* identical_shifts: find all i such that vals shifted by i (step k) matches vals at 0,k,2k... */
static int shift_results[MAX_POSITIONS];
static int n_shift_results;

static void identical_shifts(int *vals, int len, int k) {
    n_shift_results = 0;
    int H = 3;
    int sublen = len / k;
    long long pow_h = 1;
    for (int i = 0; i < sublen; i++) pow_h *= H;

    long long target_hash = 0;
    for (int i = 0; i < len; i += k)
        target_hash = target_hash * H + vals[i];

    for (int initial_shift = 0; initial_shift < k; initial_shift++) {
        long long hash_val = 0;
        for (int i = initial_shift; i < len; i += k)
            hash_val = hash_val * H + vals[i];
        for (int i = initial_shift; i < len; i += k) {
            if (hash_val == target_hash)
                shift_results[n_shift_results++] = i;
            hash_val = hash_val * H + (1 - pow_h) * vals[i];
        }
    }
}

static Pos positions[MAX_POSITIONS];
static int n_positions;

static int order_compute(Rad *flip_sizes, int n_flips) {
    /* Orders stored as set of longs */
    static long long orders[MAX_ORDERS];
    static long long new_orders[MAX_ORDERS];
    int n_orders = 1;
    orders[0] = 0;
    long long period = 1;

    intervals_init();

    while (!intervals_done()) {
        advance_to_next_unprocessed();
        if (intervals_done()) break;

        Pos start_pos = {current_point, 0};
        n_positions = 0;
        Rad uncut_region = rad_one();
        Pos pos = start_pos;
        int first = 1;

        while (first || !(rad_cmp(pos.angle, start_pos.angle) == 0 && pos.flipped == start_pos.flipped)) {
            first = 0;
            for (int fi = 0; fi < n_flips; fi++) {
                if (n_positions >= MAX_POSITIONS) break;
                positions[n_positions++] = pos;
                Rad remaining = rad_sub(rad_one(), pos.angle);
                Rad size = flip_sizes[fi];
                int cmp = rad_cmp(pos.angle, size);
                if (cmp < (pos.flipped ? 1 : 0)) {
                    uncut_region = rad_min(uncut_region,
                        pos.flipped ? pos.angle : rad_sub(size, pos.angle));
                    pos = (Pos){remaining, !pos.flipped};
                } else {
                    uncut_region = rad_min(uncut_region,
                        pos.flipped ? rad_sub(pos.angle, size) : remaining);
                    pos = (Pos){rad_sub(pos.angle, size), pos.flipped};
                }
            }
        }

        /* Build flipped array */
        int *flip_arr = (int *)malloc(n_positions * sizeof(int));
        for (int i = 0; i < n_positions; i++)
            flip_arr[i] = positions[i].flipped ? 1 : 0;

        identical_shifts(flip_arr, n_positions, n_flips);

        int curr_period = n_positions;
        int n_new = 0;
        for (int oi = 0; oi < n_orders; oi++) {
            for (int si = 0; si < n_shift_results; si++) {
                long long nv = general_crt(orders[oi], period, shift_results[si], curr_period);
                if (nv != -1) {
                    /* Check if already in new_orders */
                    int found = 0;
                    for (int k = 0; k < n_new; k++)
                        if (new_orders[k] == nv) { found = 1; break; }
                    if (!found && n_new < MAX_ORDERS)
                        new_orders[n_new++] = nv;
                }
            }
        }
        period = lcm_ll(period, curr_period);
        memcpy(orders, new_orders, n_new * sizeof(long long));
        n_orders = n_new;

        /* Process regions */
        for (int i = 0; i < n_positions; i += n_flips) {
            if (!positions[i].flipped) {
                process_interval(positions[i].angle, rad_add(positions[i].angle, uncut_region));
            }
        }

        free(flip_arr);
    }

    /* Find minimum non-zero order */
    long long min_order = period;
    for (int i = 0; i < n_orders; i++) {
        if (orders[i] > 0 && orders[i] < min_order)
            min_order = orders[i];
    }
    return (int)min_order;
}

static int F(int a, int b, int c) {
    Rad flip_sizes[3];
    if (is_sq(c)) {
        int sc = isqrt_i(c);
        glob_sqrt_c = 1.0;
        glob_d = (long long)a * b * sc;
        flip_sizes[0] = (Rad){glob_d / a, 0};
        flip_sizes[1] = (Rad){glob_d / b, 0};
        flip_sizes[2] = (Rad){glob_d / sc, 0};
    } else {
        glob_sqrt_c = sqrt((double)c);
        glob_d = (long long)a * b * c;
        flip_sizes[0] = (Rad){glob_d / a, 0};
        flip_sizes[1] = (Rad){glob_d / b, 0};
        flip_sizes[2] = (Rad){0, glob_d / c};
    }
    return order_compute(flip_sizes, 3);
}

int main(void) {
    int NN = 53;
    long long ans = 0;
    for (int a = 9; a < NN; a++) {
        for (int b = a + 1; b < NN; b++) {
            for (int c = b + 1; c <= NN; c++) {
                ans += F(a, b, c);
            }
        }
    }
    printf("%lld\n", ans);
    return 0;
}
