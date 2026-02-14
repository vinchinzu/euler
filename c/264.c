/*
 * Project Euler 264: Triangle Centres
 *
 * Find sum of perimeters of all triangles with lattice vertices,
 * circumcenter at origin, orthocenter at (5,0), perimeter <= 100000.
 *
 * For each candidate (Ax, Ay), find Bx, By via the circumradius constraint.
 * Uses divisor enumeration for efficiency.
 */
#include <stdio.h>
#include <stdlib.h>
#include <stdint.h>
#include <math.h>
#include <string.h>

typedef int64_t i64;

static int is_square(i64 n) {
    if (n < 0) return 0;
    i64 r = (i64)sqrt((double)n);
    while (r > 0 && r * r > n) r--;
    while ((r+1)*(r+1) <= n) r++;
    return r * r == n;
}

static i64 isqrt64(i64 n) {
    if (n <= 0) return 0;
    i64 r = (i64)sqrt((double)n);
    while (r > 0 && r * r > n) r--;
    while ((r+1)*(r+1) <= n) r++;
    return r;
}

/* SPF sieve */
#define SPF_LIMIT 50010
static int spf[SPF_LIMIT + 1];

static void init_spf(void) {
    for (int i = 0; i <= SPF_LIMIT; i++) spf[i] = i;
    for (int i = 2; (i64)i * i <= SPF_LIMIT; i++) {
        if (spf[i] == i) {
            for (int j = i * i; j <= SPF_LIMIT; j += i) {
                if (spf[j] == j) spf[j] = i;
            }
        }
    }
}

/* Factorize using SPF, store primes and exponents */
#define MAX_F 20
typedef struct {
    i64 p[MAX_F];
    int e[MAX_F];
    int cnt;
} Factors;

static void factorize_spf(i64 n, Factors *f) {
    f->cnt = 0;
    while (n > 1 && n <= SPF_LIMIT) {
        int p = spf[n];
        f->p[f->cnt] = p;
        f->e[f->cnt] = 0;
        while (n % p == 0) { f->e[f->cnt]++; n /= p; }
        f->cnt++;
    }
    if (n > 1) {
        f->p[f->cnt] = n;
        f->e[f->cnt] = 1;
        f->cnt++;
    }
}

/* Generate all divisors of a number given its factorization */
#define MAX_DIVS 100000
static i64 divs[MAX_DIVS];
static int num_divs;

static void gen_divisors(Factors *f) {
    divs[0] = 1;
    num_divs = 1;
    for (int i = 0; i < f->cnt; i++) {
        int old = num_divs;
        i64 pe = 1;
        for (int j = 0; j < f->e[i]; j++) {
            pe *= f->p[i];
            for (int k = 0; k < old; k++) {
                divs[num_divs++] = divs[k] * pe;
            }
        }
    }
}

/* Triangle dedup using hash set on sorted triple of points */
typedef struct {
    int x1, y1, x2, y2, x3, y3;
} Triangle;

#define HASH_SIZE (1 << 20)
#define HASH_MASK (HASH_SIZE - 1)

typedef struct HNode {
    Triangle t;
    double perim;
    struct HNode *next;
} HNode;

static HNode *htable[HASH_SIZE];
static HNode hpool[2000000];
static int hpool_idx = 0;

static unsigned tri_hash(int x1, int y1, int x2, int y2, int x3, int y3) {
    unsigned h = (unsigned)(x1 * 1000003 + y1 * 999983 + x2 * 999979 + y2 * 999961 + x3 * 999953 + y3 * 999931);
    return h & HASH_MASK;
}

static void sort3(int *ax, int *ay, int *bx, int *by, int *cx, int *cy) {
    /* Sort 3 points by (x, y) */
    int pts[3][2] = {{*ax,*ay},{*bx,*by},{*cx,*cy}};
    for (int i = 0; i < 2; i++)
        for (int j = i + 1; j < 3; j++)
            if (pts[j][0] < pts[i][0] || (pts[j][0] == pts[i][0] && pts[j][1] < pts[i][1])) {
                int t0 = pts[i][0], t1 = pts[i][1];
                pts[i][0] = pts[j][0]; pts[i][1] = pts[j][1];
                pts[j][0] = t0; pts[j][1] = t1;
            }
    *ax = pts[0][0]; *ay = pts[0][1];
    *bx = pts[1][0]; *by = pts[1][1];
    *cx = pts[2][0]; *cy = pts[2][1];
}

static int tri_eq(Triangle *a, Triangle *b) {
    return a->x1 == b->x1 && a->y1 == b->y1 && a->x2 == b->x2 && a->y2 == b->y2 && a->x3 == b->x3 && a->y3 == b->y3;
}

static void add_triangle(int x1, int y1, int x2, int y2, int x3, int y3, double perim) {
    sort3(&x1, &y1, &x2, &y2, &x3, &y3);
    unsigned h = tri_hash(x1, y1, x2, y2, x3, y3);
    Triangle t = {x1, y1, x2, y2, x3, y3};
    for (HNode *n = htable[h]; n; n = n->next) {
        if (tri_eq(&n->t, &t)) return; /* already exists */
    }
    HNode *n = &hpool[hpool_idx++];
    n->t = t;
    n->perim = perim;
    n->next = htable[h];
    htable[h] = n;
}

static void process_point(int Ax, int Ay, int N) {
    i64 R2 = (i64)Ax * Ax + (i64)Ay * Ay;
    i64 num = 2 * (i64)Ay * Ay * R2;
    i64 den = (i64)(5 - Ax) * (5 - Ax) + (i64)Ay * Ay;
    if (den == 0 || num % den != 0) return;
    i64 disc = 2 * num / den - (i64)Ay * Ay;
    if (disc < 0 || !is_square(disc)) return;

    i64 sqrt_disc = isqrt64(disc);
    int Bx = (int)((5 - Ax + sqrt_disc) / 2);
    int Cx = 5 - Ax - Bx;
    i64 By_sq = R2 - (i64)Bx * Bx;
    if (By_sq < 0 || !is_square(By_sq)) return;
    int By = (int)isqrt64(By_sq);

    /* Check if (Cx, -Ay-By) is on circumcircle */
    if ((i64)Cx * Cx + (i64)(Ay + By) * (Ay + By) != R2)
        By = -By;

    int p1x = Ax, p1y = Ay;
    int p2x = Bx, p2y = By;
    int p3x = Cx, p3y = -(Ay + By);

    /* Check non-degenerate (twice area != 0) */
    i64 area2 = (i64)p1x * (p2y - p3y) + (i64)p2x * (p3y - p1y) + (i64)p3x * (p1y - p2y);
    if (area2 < 0) area2 = -area2;
    if (area2 == 0) return;

    double perim = sqrt((double)((i64)(p2x-p1x)*(p2x-p1x) + (i64)(p2y-p1y)*(p2y-p1y)))
                 + sqrt((double)((i64)(p3x-p2x)*(p3x-p2x) + (i64)(p3y-p2y)*(p3y-p2y)))
                 + sqrt((double)((i64)(p1x-p3x)*(p1x-p3x) + (i64)(p1y-p3y)*(p1y-p3y)));

    if (perim <= (double)N) {
        add_triangle(p1x, p1y, p2x, p2y, p3x, p3y, perim);
        /* Reflected triangle */
        add_triangle(Ax, -Ay, Bx, -By, Cx, Ay + By, perim);
    }
}

int main(void) {
    int N = 100000;
    int max_ax = N / 4;

    init_spf();
    memset(htable, 0, sizeof(htable));

    for (int Ax = 0; Ax <= max_ax; Ax++) {
        i64 D = (i64)(5 - Ax) * (5 - Ax);
        i64 W = 10 * Ax - 25;

        int parity_start = Ax % 2 + 1;
        int ay_parity = parity_start % 2;

        if (D != 0 && W != 0) {
            i64 u = (5 - Ax) >= 0 ? (5 - Ax) : (Ax - 5);
            i64 v = (2 * Ax - 5) >= 0 ? (2 * Ax - 5) : (5 - 2 * Ax);

            /* Factorize product = 2 * 5 * u^2 * v */
            Factors combined;
            combined.cnt = 0;

            /* Start with factor 2 */
            combined.p[0] = 2; combined.e[0] = 1; combined.cnt = 1;
            /* Factor 5 */
            combined.p[1] = 5; combined.e[1] = 1; combined.cnt = 2;

            /* u^2 */
            if (u > 1) {
                Factors uf;
                factorize_spf(u, &uf);
                for (int i = 0; i < uf.cnt; i++) {
                    int found = 0;
                    for (int j = 0; j < combined.cnt; j++) {
                        if (combined.p[j] == uf.p[i]) {
                            combined.e[j] += 2 * uf.e[i];
                            found = 1;
                            break;
                        }
                    }
                    if (!found) {
                        combined.p[combined.cnt] = uf.p[i];
                        combined.e[combined.cnt] = 2 * uf.e[i];
                        combined.cnt++;
                    }
                }
            }

            /* v */
            if (v > 1) {
                Factors vf;
                factorize_spf(v, &vf);
                for (int i = 0; i < vf.cnt; i++) {
                    int found = 0;
                    for (int j = 0; j < combined.cnt; j++) {
                        if (combined.p[j] == vf.p[i]) {
                            combined.e[j] += vf.e[i];
                            found = 1;
                            break;
                        }
                    }
                    if (!found) {
                        combined.p[combined.cnt] = vf.p[i];
                        combined.e[combined.cnt] = vf.e[i];
                        combined.cnt++;
                    }
                }
            }

            gen_divisors(&combined);

            for (int di = 0; di < num_divs; di++) {
                i64 d = divs[di];
                i64 ay_sq = d - D;
                if (ay_sq <= 0) continue;
                if (!is_square(ay_sq)) continue;
                int Ay = (int)isqrt64(ay_sq);
                if (Ay % 2 != ay_parity) continue;
                if (Ay > max_ax) continue;

                process_point(Ax, Ay, N);
            }
        } else if (D == 0) {
            /* Ax = 5 */
            for (int Ay = parity_start; Ay <= max_ax; Ay += 2) {
                process_point(Ax, Ay, N);
            }
        }

        /* Handle Ay = 0 case */
        i64 disc_val = 4 * (i64)Ax * Ax - (i64)(5 - Ax) * (5 - Ax);
        if (is_square(disc_val)) {
            int By = (int)(isqrt64(disc_val) / 2);
            if (By > 0) {
                int p1x = Ax, p1y = 0;
                int p2x = (5 - Ax) / 2, p2y = By;
                int p3x = (5 - Ax) / 2, p3y = -By;
                double perim = sqrt((double)((i64)(p2x-p1x)*(p2x-p1x) + (i64)(p2y-p1y)*(p2y-p1y)))
                             + sqrt((double)((i64)(p3x-p2x)*(p3x-p2x) + (i64)(p3y-p2y)*(p3y-p2y)))
                             + sqrt((double)((i64)(p1x-p3x)*(p1x-p3x) + (i64)(p1y-p3y)*(p1y-p3y)));
                if (perim <= (double)N) {
                    add_triangle(p1x, p1y, p2x, p2y, p3x, p3y, perim);
                }
            }
        }
    }

    /* Sum all perimeters */
    double total = 0.0;
    for (int i = 0; i < hpool_idx; i++) {
        total += hpool[i].perim;
    }

    printf("%.4f\n", total);
    return 0;
}
