"""Project Euler Problem 292: Pythagorean Polygons.

Find P(120), the number of distinct convex polygons (up to translation) with
integer coordinates, integer-length edges, at least 3 vertices, no three
collinear, and perimeter <= 120.

Approach:
- Each edge of such a polygon is a vector (a, b) with integer length sqrt(a^2+b^2).
- For convexity, edges must be sorted by strictly increasing angle.
- A primitive direction (a, b) with gcd(|a|,|b|)=1 and a^2+b^2 = c^2 can be
  used with any multiplier k, giving edge length k*c.
- We enumerate all 156 primitive directions, split them into two halves by angle
  (lower and upper semicircle), compute all reachable DP states for one half,
  then combine using the symmetry that the second half is the negation of the first.
- The DP is accelerated using a compiled C extension for the inner loop.
"""

import ctypes
import os
import subprocess
from math import atan2, gcd, isqrt

N = 120

C_SOURCE = r"""
#include <stdlib.h>
#include <string.h>

#define HASH_SIZE (1 << 22)
#define HASH_MASK (HASH_SIZE - 1)

typedef struct Entry {
    long long key;
    long long val;
    struct Entry *next;
} Entry;

static Entry *buckets[HASH_SIZE];
static Entry pool[2000000];
static int pool_idx = 0;

static unsigned int hash_key(long long k) {
    unsigned long long u = (unsigned long long)k;
    u = (u ^ (u >> 30)) * 0xbf58476d1ce4e5b9ULL;
    u = (u ^ (u >> 27)) * 0x94d049bb133111ebULL;
    return (unsigned int)(u ^ (u >> 31)) & HASH_MASK;
}

static void hm_clear(void) {
    memset(buckets, 0, sizeof(buckets));
    pool_idx = 0;
}

static void hm_add(long long key, long long val) {
    unsigned int h = hash_key(key);
    Entry *e = buckets[h];
    while (e) {
        if (e->key == key) { e->val += val; return; }
        e = e->next;
    }
    Entry *ne = &pool[pool_idx++];
    ne->key = key;
    ne->val = val;
    ne->next = buckets[h];
    buckets[h] = ne;
}

static long long encode(int sx, int sy, int peri, int ne) {
    return (long long)((sx + 120) * 241 + (sy + 120)) * (121 * 4) + peri * 4 + ne;
}

static void decode(long long key, int *sx, int *sy, int *peri, int *ne) {
    *ne = (int)(key % 4); key /= 4;
    *peri = (int)(key % 121); key /= 121;
    *sy = (int)(key % 241) - 120;
    *sx = (int)(key / 241) - 120;
}

#define MAX_DIRS 200
#define MAX_OPTS 130
static int dir_dx[MAX_DIRS][MAX_OPTS];
static int dir_dy[MAX_DIRS][MAX_OPTS];
static int dir_dl[MAX_DIRS][MAX_OPTS];
static int dir_nopts[MAX_DIRS];

void set_direction(int idx, int opt_idx, int dx, int dy, int dl) {
    dir_dx[idx][opt_idx] = dx;
    dir_dy[idx][opt_idx] = dy;
    dir_dl[idx][opt_idx] = dl;
}

void set_direction_count(int idx, int count) {
    dir_nopts[idx] = count;
}

int compute_half(int start_dir, int end_dir, int N,
                 long long *out_keys, long long *out_vals, int max_out) {
    hm_clear();
    hm_add(encode(0, 0, 0, 0), 1);

    static long long iter_keys[500000];
    static long long iter_vals[500000];

    for (int didx = start_dir; didx < end_dir; didx++) {
        int nopts = dir_nopts[didx];
        if (nopts == 0) continue;

        int nstates = 0;
        for (int b = 0; b < HASH_SIZE; b++) {
            Entry *e = buckets[b];
            while (e) {
                iter_keys[nstates] = e->key;
                iter_vals[nstates] = e->val;
                nstates++;
                e = e->next;
            }
        }

        hm_clear();

        for (int s = 0; s < nstates; s++) {
            long long key = iter_keys[s];
            long long cnt = iter_vals[s];
            int sx, sy, peri, ne;
            decode(key, &sx, &sy, &peri, &ne);

            hm_add(key, cnt);

            for (int o = 0; o < nopts; o++) {
                int new_peri = peri + dir_dl[didx][o];
                if (new_peri <= N) {
                    int new_ne = ne < 3 ? ne + 1 : 3;
                    long long nk = encode(sx + dir_dx[didx][o],
                                          sy + dir_dy[didx][o],
                                          new_peri, new_ne);
                    hm_add(nk, cnt);
                }
            }
        }
    }

    int nstates = 0;
    for (int b = 0; b < HASH_SIZE; b++) {
        Entry *e = buckets[b];
        while (e) {
            if (nstates < max_out) {
                out_keys[nstates] = e->key;
                out_vals[nstates] = e->val;
            }
            nstates++;
            e = e->next;
        }
    }
    return nstates;
}
"""


def compile_helper():
    """Compile the C helper library, returning the path to the .so file."""
    script_dir = os.path.dirname(os.path.abspath(__file__))
    so_path = os.path.join(script_dir, "292_helper.so")
    c_path = os.path.join(script_dir, "292_helper.c")

    # Check if .so already exists and is newer than this script
    if os.path.exists(so_path):
        return so_path

    # Write C source
    with open(c_path, "w") as f:
        f.write(C_SOURCE)

    # Compile
    subprocess.check_call(
        ["gcc", "-O2", "-shared", "-fPIC", "-o", so_path, c_path, "-lm"],
        stdout=subprocess.DEVNULL,
        stderr=subprocess.DEVNULL,
    )
    return so_path


def generate_directions():
    """Generate all primitive Pythagorean directions sorted by angle."""
    directions = []
    for a in range(-N, N + 1):
        for b in range(-N, N + 1):
            if a == 0 and b == 0:
                continue
            c2 = a * a + b * b
            c = isqrt(c2)
            if c * c != c2:
                continue
            if c > N:
                continue
            g = gcd(abs(a), abs(b))
            if g != 1:
                continue
            directions.append((a, b, c))
    directions.sort(key=lambda d: atan2(d[1], d[0]))
    return directions


def solve():
    """Solve Problem 292 using meet-in-the-middle DP with C acceleration."""
    so_path = compile_helper()
    lib = ctypes.CDLL(so_path)

    lib.set_direction.argtypes = [
        ctypes.c_int, ctypes.c_int, ctypes.c_int, ctypes.c_int, ctypes.c_int,
    ]
    lib.set_direction.restype = None
    lib.set_direction_count.argtypes = [ctypes.c_int, ctypes.c_int]
    lib.set_direction_count.restype = None
    lib.compute_half.argtypes = [
        ctypes.c_int, ctypes.c_int, ctypes.c_int,
        ctypes.POINTER(ctypes.c_longlong),
        ctypes.POINTER(ctypes.c_longlong),
        ctypes.c_int,
    ]
    lib.compute_half.restype = ctypes.c_int

    directions = generate_directions()
    nd = len(directions)
    mid = nd // 2

    # Set up direction options in C library
    for i, (a, b, c) in enumerate(directions):
        nopts = 0
        for k in range(1, N // c + 1):
            lib.set_direction(i, nopts, k * a, k * b, k * c)
            nopts += 1
        lib.set_direction_count(i, nopts)

    # Compute first half of directions using C
    MAX_OUT = 500000
    out_keys = (ctypes.c_longlong * MAX_OUT)()
    out_vals = (ctypes.c_longlong * MAX_OUT)()

    nstates = lib.compute_half(0, mid, N, out_keys, out_vals, MAX_OUT)

    # By symmetry, direction[i] and direction[mid+i] are negatives of each other,
    # so states2[(-sx, -sy, peri, ne)] = states1[(sx, sy, peri, ne)].
    # For combining: we need sx1 + sx2 = 0, sy1 + sy2 = 0, p1 + p2 <= N,
    # ne1 + ne2 >= 3. With symmetry this becomes matching states1 against itself.

    # Decode states and group by (sx, sy, ne) with prefix sums over perimeter
    group = {}
    decoded = []
    for i in range(nstates):
        key = int(out_keys[i])
        cnt = int(out_vals[i])
        ne = key % 4
        key2 = key // 4
        peri = key2 % 121
        key3 = key2 // 121
        sy = key3 % 241 - 120
        sx = key3 // 241 - 120
        decoded.append((sx, sy, peri, ne, cnt))

        gkey = (sx, sy, ne)
        if gkey not in group:
            group[gkey] = [0] * (N + 1)
        group[gkey][peri] += cnt

    for gkey in group:
        arr = group[gkey]
        for p in range(1, N + 1):
            arr[p] += arr[p - 1]

    # Combine: for each state from first half, look up matching states
    total = 0
    for sx1, sy1, p1, ne1, cnt1 in decoded:
        remaining = N - p1
        if remaining < 0:
            continue
        min_ne2 = max(0, 3 - ne1)
        for ne2 in range(min_ne2, 4):
            gkey = (sx1, sy1, ne2)
            if gkey in group:
                total += cnt1 * group[gkey][remaining]

    return total


def solve_pure_python():
    """Pure Python fallback if C compilation fails."""
    directions = generate_directions()
    nd = len(directions)
    mid = nd // 2

    dir_options = []
    for a, b, c in directions:
        opts = []
        for k in range(1, N // c + 1):
            opts.append((k * a, k * b, k * c))
        dir_options.append(opts)

    # DP over first half of directions
    states = {(0, 0, 0, 0): 1}
    for idx in range(mid):
        opts = dir_options[idx]
        if not opts:
            continue
        new_states = {}
        for key, cnt in states.items():
            sx, sy, peri, ne = key
            new_states[key] = new_states.get(key, 0) + cnt
            for dx, dy, dl in opts:
                new_peri = peri + dl
                if new_peri <= N:
                    new_ne = ne + 1 if ne < 3 else 3
                    nk = (sx + dx, sy + dy, new_peri, new_ne)
                    new_states[nk] = new_states.get(nk, 0) + cnt
        states = new_states

    # Group by (sx, sy, ne) with prefix sums
    group = {}
    for (sx, sy, peri, ne), cnt in states.items():
        gkey = (sx, sy, ne)
        if gkey not in group:
            group[gkey] = [0] * (N + 1)
        group[gkey][peri] += cnt

    for gkey in group:
        arr = group[gkey]
        for p in range(1, N + 1):
            arr[p] += arr[p - 1]

    # Combine using symmetry
    total = 0
    for (sx1, sy1, p1, ne1), cnt1 in states.items():
        remaining = N - p1
        if remaining < 0:
            continue
        min_ne2 = max(0, 3 - ne1)
        for ne2 in range(min_ne2, 4):
            gkey = (sx1, sy1, ne2)
            if gkey in group:
                total += cnt1 * group[gkey][remaining]

    return total


if __name__ == "__main__":
    try:
        result = solve()
    except (OSError, subprocess.CalledProcessError):
        result = solve_pure_python()
    print(result)
