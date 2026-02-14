/* Project Euler 482 - The incenter of a triangle
 * Extracted from embedded C in python/482.py
 */
#include <stdio.h>
#include <stdlib.h>
#include <math.h>
#include <stdint.h>

typedef int64_t i64;

#define N 10000000LL
#define L_VAL ((i64)(N / sqrt(108.0)))

i64 gcd(i64 a, i64 b) {
    while (b) { i64 t = b; b = a % b; a = t; }
    return a;
}

i64 lcm(i64 a, i64 b) {
    return a / gcd(a, b) * b;
}

i64 isqrt_func(i64 n) {
    if (n <= 0) return 0;
    i64 x = (i64)sqrt((double)n);
    while (x * x > n) x--;
    while ((x+1) * (x+1) <= n) x++;
    return x;
}

i64 tr(i64 n) {
    return n * (n + 1) / 2;
}

/* Multimap: a -> list of b values */
typedef struct {
    i64 *vals;
    int count;
    int cap;
} ValList;

ValList *tri_map;
i64 MAP_SIZE;

void add_to_map(i64 a, i64 b) {
    if (a >= MAP_SIZE) return;
    ValList *vl = &tri_map[a];
    if (vl->count == vl->cap) {
        vl->cap = vl->cap ? vl->cap * 2 : 4;
        vl->vals = realloc(vl->vals, vl->cap * sizeof(i64));
    }
    vl->vals[vl->count++] = b;
}

/* Hash set for solutions */
#define SOL_HASH_SIZE 2000003
typedef struct { i64 x, y, z; int used; } Triple;
Triple *sol_table;
int num_solutions = 0;

#define MAX_SOLUTIONS 500000
Triple *solutions;

int add_solution(i64 x, i64 y, i64 z) {
    /* Sort */
    if (x > y) { i64 t = x; x = y; y = t; }
    if (y > z) { i64 t = y; y = z; z = t; }
    if (x > y) { i64 t = x; x = y; y = t; }

    uint64_t h = ((uint64_t)x * 1000000007ULL + y) * 1000000007ULL + z;
    uint64_t idx = h % SOL_HASH_SIZE;
    while (sol_table[idx].used) {
        if (sol_table[idx].x == x && sol_table[idx].y == y && sol_table[idx].z == z)
            return 0;
        idx = (idx + 1) % SOL_HASH_SIZE;
    }
    sol_table[idx].x = x;
    sol_table[idx].y = y;
    sol_table[idx].z = z;
    sol_table[idx].used = 1;
    if (num_solutions < MAX_SOLUTIONS) {
        solutions[num_solutions].x = x;
        solutions[num_solutions].y = y;
        solutions[num_solutions].z = z;
        num_solutions++;
    }
    return 1;
}

int all_divisors[10000];
int num_divisors;

void get_divisors(i64 n) {
    num_divisors = 0;
    for (i64 i = 1; i * i <= n; i++) {
        if (n % i == 0) {
            all_divisors[num_divisors++] = (int)i;
            if (i != n / i)
                all_divisors[num_divisors++] = (int)(n / i);
        }
    }
}

/* Collect keys with non-empty lists */
i64 *keys;
int num_keys;

int main() {
    MAP_SIZE = L_VAL + 10;
    tri_map = (ValList*)calloc(MAP_SIZE, sizeof(ValList));
    sol_table = (Triple*)calloc(SOL_HASH_SIZE, sizeof(Triple));
    solutions = (Triple*)malloc(MAX_SOLUTIONS * sizeof(Triple));
    keys = (i64*)malloc(MAP_SIZE * sizeof(i64));

    /* Generate primitive Pythagorean triples */
    for (i64 m = 2; 2 * m * m <= N; m++) {
        for (i64 n = 1; n < m && 2 * m * (m + n) <= N; n++) {
            if ((m + n) % 2 == 1 && gcd(m, n) == 1) {
                i64 a = m * m - n * n;
                i64 b = 2 * m * n;
                if (a < MAP_SIZE) add_to_map(a, b);
                if (b < MAP_SIZE) add_to_map(b, a);
            }
        }
    }

    /* Collect keys */
    num_keys = 0;
    for (i64 i = 1; i < MAP_SIZE; i++) {
        if (tri_map[i].count > 0) {
            keys[num_keys++] = i;
        }
    }

    /* Process each key a1 */
    for (int ki = 0; ki < num_keys; ki++) {
        i64 a1 = keys[ki];
        get_divisors(a1);

        /* Generate valid a2 values */
        for (int di = 0; di < num_divisors; di++) {
            i64 d = all_divisors[di];
            for (i64 mult = 1; mult * a1 < MAP_SIZE && mult * d <= a1; mult++) {
                i64 a2 = mult * d;
                if (a2 >= MAP_SIZE || tri_map[a2].count == 0) continue;

                i64 r = lcm(a1, a2);
                if (r > N) continue;

                /* Iterate over b values */
                for (int bi = 0; bi < tri_map[a1].count; bi++) {
                    i64 b1 = tri_map[a1].vals[bi];
                    for (int bj = 0; bj < tri_map[a2].count; bj++) {
                        i64 b2 = tri_map[a2].vals[bj];

                        i64 x = b1 * r / a1;
                        i64 y = b2 * r / a2;
                        i64 r2 = r * r;
                        i64 num = r2 * (x + y);
                        i64 den = x * y - r2;

                        if (den > 0 && 2 * (x + y + num / den) <= N) {
                            i64 g = gcd(num, den);
                            num /= g;
                            den /= g;
                            if (2 * (x * den + y * den + num) <= N) {
                                add_solution(x * den, y * den, num);
                            }
                        }
                    }
                }
            }
        }
    }

    /* Calculate answer */
    i64 ans = 0;
    for (int i = 0; i < num_solutions; i++) {
        i64 x = solutions[i].x;
        i64 y = solutions[i].y;
        i64 z = solutions[i].z;
        i64 r2 = (x * y * z) / (x + y + z);
        i64 perim = 2 * (x + y + z);
        i64 ia = isqrt_func(r2 + x * x);
        i64 ib = isqrt_func(r2 + y * y);
        i64 ic = isqrt_func(r2 + z * z);
        ans += tr(N / perim) * (perim + ia + ib + ic);
    }

    printf("%lld\n", (long long)ans);
    return 0;
}
