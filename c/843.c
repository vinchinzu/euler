/*
 * Project Euler 843: Circle of Absolute Differences
 *
 * S(100) = sum of all possible periods for circles of n integers, n=3..100.
 *
 * Uses GF(2) polynomial arithmetic and cyclotomic factorization.
 * The period structure depends on the factorization of x + x^(n-1) over GF(2)
 * modulo cyclotomic factors.
 *
 * This is a direct C translation of the Python solution using bitwise
 * polynomial arithmetic in GF(2).
 */
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <math.h>

typedef unsigned long long ull;

/* GF(2) polynomial represented as array of ull words (little-endian bits) */
#define MAX_WORDS 64  /* supports polynomials up to degree ~4096 */

typedef struct {
    ull w[MAX_WORDS];
    int deg; /* degree of polynomial, -1 for zero */
} GF2Poly;

static GF2Poly gf2_zero(void) {
    GF2Poly p;
    memset(p.w, 0, sizeof(p.w));
    p.deg = -1;
    return p;
}

static GF2Poly gf2_one(void) {
    GF2Poly p = gf2_zero();
    p.w[0] = 1;
    p.deg = 0;
    return p;
}

static int gf2_is_zero(GF2Poly *a) { return a->deg < 0; }
static int gf2_is_one(GF2Poly *a) { return a->deg == 0 && a->w[0] == 1; }

static int gf2_get_bit(GF2Poly *a, int i) {
    return (a->w[i / 64] >> (i % 64)) & 1;
}

static void gf2_set_bit(GF2Poly *a, int i) {
    a->w[i / 64] |= (1ULL << (i % 64));
}

static void gf2_flip_bit(GF2Poly *a, int i) {
    a->w[i / 64] ^= (1ULL << (i % 64));
}

static void gf2_update_deg(GF2Poly *a) {
    for (int i = MAX_WORDS - 1; i >= 0; i--) {
        if (a->w[i]) {
            int bit = 63;
            while (bit >= 0 && !((a->w[i] >> bit) & 1)) bit--;
            a->deg = i * 64 + bit;
            return;
        }
    }
    a->deg = -1;
}

static GF2Poly gf2_xor(GF2Poly a, GF2Poly b) {
    GF2Poly r;
    for (int i = 0; i < MAX_WORDS; i++) r.w[i] = a.w[i] ^ b.w[i];
    gf2_update_deg(&r);
    return r;
}

static int gf2_equal(GF2Poly *a, GF2Poly *b) {
    if (a->deg != b->deg) return 0;
    for (int i = 0; i < MAX_WORDS; i++)
        if (a->w[i] != b->w[i]) return 0;
    return 1;
}

/* Multiply two GF(2) polynomials */
static GF2Poly gf2_mul(GF2Poly a, GF2Poly b) {
    GF2Poly r = gf2_zero();
    if (a.deg < 0 || b.deg < 0) return r;
    for (int i = 0; i <= a.deg; i++) {
        if (gf2_get_bit(&a, i)) {
            /* XOR b shifted by i into r */
            for (int j = 0; j <= b.deg; j++) {
                if (gf2_get_bit(&b, j)) {
                    gf2_flip_bit(&r, i + j);
                }
            }
        }
    }
    gf2_update_deg(&r);
    return r;
}

/* GF(2) polynomial division: returns quotient, sets *rem to remainder */
static GF2Poly gf2_divmod(GF2Poly a, GF2Poly b, GF2Poly *rem) {
    GF2Poly q = gf2_zero();
    if (a.deg < b.deg) { *rem = a; return q; }
    GF2Poly r = a;
    int db = b.deg;
    while (r.deg >= db) {
        int shift = r.deg - db;
        gf2_set_bit(&q, shift);
        /* r ^= b << shift */
        for (int i = 0; i <= db; i++) {
            if (gf2_get_bit(&b, i))
                gf2_flip_bit(&r, i + shift);
        }
        gf2_update_deg(&r);
    }
    *rem = r;
    return q;
}

static GF2Poly gf2_mod(GF2Poly a, GF2Poly b) {
    GF2Poly rem;
    gf2_divmod(a, b, &rem);
    return rem;
}

/* GF(2) polynomial power mod */
static GF2Poly gf2_powmod(GF2Poly base, ull exp, GF2Poly mod) {
    GF2Poly result = gf2_one();
    GF2Poly cur = gf2_mod(base, mod);
    while (exp > 0) {
        if (exp & 1)
            result = gf2_mod(gf2_mul(result, cur), mod);
        cur = gf2_mod(gf2_mul(cur, cur), mod);
        exp >>= 1;
    }
    return result;
}

/* Compute cyclotomic polynomial Phi_d(x) over Z, then reduce mod 2 */
/* Using the standard recurrence: x^d - 1 = prod_{k|d} Phi_k(x) */
/* Store integer polynomials temporarily */

#define MAX_DEG 200
typedef struct { int c[MAX_DEG + 1]; int deg; } ZPoly;

static ZPoly zpoly_one(void) {
    ZPoly p; memset(p.c, 0, sizeof(p.c)); p.c[0] = 1; p.deg = 0; return p;
}

static ZPoly zpoly_xn_minus_1(int n) {
    ZPoly p; memset(p.c, 0, sizeof(p.c));
    p.c[n] = 1; p.c[0] = -1; p.deg = n;
    return p;
}

/* Integer polynomial division (exact) */
static ZPoly zpoly_div(ZPoly a, ZPoly b) {
    ZPoly q; memset(q.c, 0, sizeof(q.c)); q.deg = a.deg - b.deg;
    if (q.deg < 0) { q.deg = 0; return q; }
    int ac[MAX_DEG + 1];
    memcpy(ac, a.c, sizeof(ac));
    for (int i = a.deg; i >= b.deg; i--) {
        if (ac[i] == 0) continue;
        int coeff = ac[i] / b.c[b.deg]; /* exact division */
        q.c[i - b.deg] = coeff;
        for (int j = 0; j <= b.deg; j++)
            ac[i - b.deg + j] -= coeff * b.c[j];
    }
    return q;
}

/* Get divisors of n */
static int get_divisors(int n, int *divs) {
    int nd = 0;
    for (int i = 1; i * i <= n; i++) {
        if (n % i == 0) {
            divs[nd++] = i;
            if (i != n / i) divs[nd++] = n / i;
        }
    }
    /* Sort */
    for (int i = 0; i < nd - 1; i++)
        for (int j = i + 1; j < nd; j++)
            if (divs[i] > divs[j]) { int t = divs[i]; divs[i] = divs[j]; divs[j] = t; }
    return nd;
}

static ZPoly cyclotomic_polys[MAX_DEG + 1];
static int cyclo_computed[MAX_DEG + 1];

static ZPoly get_cyclotomic(int d) {
    if (cyclo_computed[d]) return cyclotomic_polys[d];
    /* Phi_d(x) = (x^d - 1) / prod_{k|d, k<d} Phi_k(x) */
    ZPoly num = zpoly_xn_minus_1(d);
    int divs[200]; int nd = get_divisors(d, divs);
    for (int i = 0; i < nd; i++) {
        if (divs[i] < d) {
            ZPoly pk = get_cyclotomic(divs[i]);
            num = zpoly_div(num, pk);
        }
    }
    cyclotomic_polys[d] = num;
    cyclo_computed[d] = 1;
    return num;
}

/* Factor Phi_d(x) mod 2: all irreducible factors have the same degree */
/* Degree of each factor = ord_d(2) = multiplicative order of 2 mod d */
static int ord2_mod(int d) {
    if (d <= 1) return 1;
    int r = 1;
    ull x = 2;
    for (int i = 1; i <= d; i++) {
        if (x % d == 0) { x = 0; break; }
        if (x == 1) return i;
        x = (x * 2) % d;
    }
    return d; /* shouldn't happen for odd d */
}

/* Convert ZPoly to GF2Poly */
static GF2Poly zpoly_to_gf2(ZPoly *zp) {
    GF2Poly p = gf2_zero();
    for (int i = 0; i <= zp->deg; i++) {
        if (zp->c[i] & 1)
            gf2_set_bit(&p, i);
    }
    gf2_update_deg(&p);
    return p;
}

/* Factor a GF2 polynomial that is a product of k irreducible polys all of degree fd */
/* Returns one irreducible factor */
static GF2Poly gf2_find_factor(GF2Poly poly, int fd) {
    if (poly.deg == fd) return poly;
    /* Try x, x+1, x^2, x^2+1, x^2+x, x^2+x+1, etc. as GCD separators */
    /* Use Berlekamp or Cantor-Zassenhaus */
    /* Simple approach for small degrees: try gcd(poly, x^(2^i) + x) for i = fd, 2*fd, ... */
    /* Actually: gcd(poly, x^(2^fd) - x) should split it */
    /* In GF(2), x^(2^m) - x = x^(2^m) + x */

    /* Compute x^(2^fd) mod poly, then add x, then gcd */
    GF2Poly x_poly = gf2_zero();
    gf2_set_bit(&x_poly, 1);
    x_poly.deg = 1;

    GF2Poly xpow = x_poly;
    for (int i = 0; i < fd; i++) {
        xpow = gf2_powmod(xpow, 2, poly);
    }
    /* xpow = x^(2^fd) mod poly */
    GF2Poly diff = gf2_xor(xpow, x_poly); /* x^(2^fd) + x */

    /* GCD */
    GF2Poly a = poly, b = diff;
    while (!gf2_is_zero(&b)) {
        GF2Poly rem;
        gf2_divmod(a, b, &rem);
        a = b;
        b = rem;
    }
    if (a.deg > 0 && a.deg < poly.deg) {
        return gf2_find_factor(a, fd);
    }

    /* If that didn't work, try random elements */
    /* For small cases, just try shifting */
    for (int offset = 2; offset < 100; offset++) {
        GF2Poly test = gf2_zero();
        gf2_set_bit(&test, offset);
        gf2_update_deg(&test);

        xpow = test;
        for (int i = 0; i < fd; i++) {
            xpow = gf2_powmod(xpow, 2, poly);
        }
        diff = gf2_xor(xpow, test);

        a = poly; b = diff;
        while (!gf2_is_zero(&b)) {
            GF2Poly rem;
            gf2_divmod(a, b, &rem);
            a = b;
            b = rem;
        }
        if (a.deg > 0 && a.deg < poly.deg) {
            return gf2_find_factor(a, fd);
        }
    }

    return poly; /* fallback */
}

/* Factorize n */
typedef struct { int p; int e; } PF;
static int factorize_small(int n, PF *pf) {
    int nf = 0;
    for (int p = 2; p * p <= n; p++) {
        if (n % p == 0) {
            pf[nf].p = p; pf[nf].e = 0;
            while (n % p == 0) { n /= p; pf[nf].e++; }
            nf++;
        }
    }
    if (n > 1) { pf[nf].p = n; pf[nf].e = 1; nf++; }
    return nf;
}

/* Get component periods: find order of elem mod f, then all powers of 2 of that order
 * that give elem^(2^i * order) == 1 mod mod_poly */
static void get_component_periods(GF2Poly elem, GF2Poly mod_poly, GF2Poly f,
                                   ull *periods, int *np) {
    int d_f = f.deg;
    ull M = (1ULL << d_f) - 1;

    /* Find order of elem in (GF(2)[x]/f)^* */
    /* Order divides M = 2^d_f - 1 */
    PF pf[30]; int nf = factorize_small((int)M, pf);

    ull m = M;
    for (int i = 0; i < nf; i++) {
        for (int j = 0; j < pf[i].e; j++) {
            GF2Poly test = gf2_powmod(elem, m / pf[i].p, f);
            if (gf2_is_one(&test)) {
                m /= pf[i].p;
            } else {
                break;
            }
        }
    }

    *np = 0;
    periods[(*np)++] = 1;
    periods[(*np)++] = m;

    ull curr = m;
    GF2Poly one = gf2_one();
    while (1) {
        GF2Poly test = gf2_powmod(elem, curr, mod_poly);
        if (gf2_equal(&test, &one)) break;
        curr *= 2;
        periods[(*np)++] = curr;
        if (*np > 60) break;
    }
}

static ull gcd_ull(ull a, ull b) { while (b) { ull t = b; b = a % b; a = t; } return a; }
static ull lcm_ull(ull a, ull b) { return a / gcd_ull(a, b) * b; }

/* Set operations for periods - we need all possible LCMs from Cartesian product */
#define MAX_PERIODS 100000

static ull solve_n(int n) {
    int n_odd = n, s = 0;
    while (n_odd % 2 == 0) { n_odd /= 2; s++; }

    int divs[200]; int nd = get_divisors(n_odd, divs);

    /* For each divisor d of n_odd, compute a set of possible periods */
    ull *all_sets[200];
    int set_sizes[200];

    for (int di = 0; di < nd; di++) {
        int d = divs[di];
        all_sets[di] = (ull *)malloc(100 * sizeof(ull));

        if (d == 1) {
            all_sets[di][0] = 1;
            set_sizes[di] = 1;
            continue;
        }

        /* Get cyclotomic poly Phi_d, factor mod 2, get one irreducible factor f */
        /* Initialize cyclotomic for d=1 */
        if (!cyclo_computed[1]) {
            cyclotomic_polys[1].deg = 1;
            cyclotomic_polys[1].c[0] = -1;
            cyclotomic_polys[1].c[1] = 1;
            cyclo_computed[1] = 1;
        }

        ZPoly phi_d = get_cyclotomic(d);
        GF2Poly phi_gf2 = zpoly_to_gf2(&phi_d);

        int fd = ord2_mod(d);
        GF2Poly f = gf2_find_factor(phi_gf2, fd);

        int exponent = 1 << s; /* 2^s */
        /* mod_poly = f^exponent */
        GF2Poly mod_poly = gf2_one();
        for (int i = 0; i < exponent; i++) {
            mod_poly = gf2_mul(mod_poly, f);
        }

        /* P(x) = x + x^(n-1) */
        GF2Poly P = gf2_zero();
        gf2_set_bit(&P, 1);
        gf2_set_bit(&P, n - 1);
        gf2_update_deg(&P);

        GF2Poly base_elem = gf2_mod(P, mod_poly);

        if (gf2_is_zero(&base_elem)) {
            all_sets[di][0] = 1;
            set_sizes[di] = 1;
        } else {
            ull periods[100]; int np;
            get_component_periods(base_elem, mod_poly, f, periods, &np);
            memcpy(all_sets[di], periods, np * sizeof(ull));
            set_sizes[di] = np;
        }
    }

    /* Compute all possible LCMs from Cartesian product of all sets */
    /* Use a running set of possible LCMs */
    ull *current = (ull *)malloc(MAX_PERIODS * sizeof(ull));
    int cur_size = 1;
    current[0] = 1;

    for (int di = 0; di < nd; di++) {
        ull *next = (ull *)malloc(MAX_PERIODS * sizeof(ull));
        int next_size = 0;

        for (int i = 0; i < cur_size; i++) {
            for (int j = 0; j < set_sizes[di]; j++) {
                ull val = lcm_ull(current[i], all_sets[di][j]);
                /* Check if already in next */
                int found = 0;
                for (int k = 0; k < next_size; k++) {
                    if (next[k] == val) { found = 1; break; }
                }
                if (!found && next_size < MAX_PERIODS)
                    next[next_size++] = val;
            }
        }

        free(current);
        current = next;
        cur_size = next_size;
    }

    /* Sum all unique periods across all n values (returned to caller) */
    ull result = 0;
    /* Actually we need to return the set, not the sum. Let caller handle. */
    /* For this problem, we accumulate into a global set */

    for (int di = 0; di < nd; di++) free(all_sets[di]);

    /* Return the set via globals */
    /* Actually, let's restructure: we collect all unique periods into a master set */
    /* and sum at the end */

    /* Store result in a static buffer */
    static ull result_set[MAX_PERIODS];
    static int result_size;
    result_size = cur_size;
    memcpy(result_set, current, cur_size * sizeof(ull));
    free(current);

    return 0; /* dummy - we use result_set/result_size */
}

int main(void) {
    memset(cyclo_computed, 0, sizeof(cyclo_computed));

    /* Initialize Phi_1 */
    cyclotomic_polys[1].deg = 1;
    cyclotomic_polys[1].c[0] = -1;
    cyclotomic_polys[1].c[1] = 1;
    cyclo_computed[1] = 1;

    /* Collect all periods from all n=3..100 into a master set */
    ull *master = (ull *)malloc(1000000 * sizeof(ull));
    int master_size = 0;

    for (int n = 3; n <= 100; n++) {
        /* Compute periods for this n */
        int n_odd = n, s = 0;
        while (n_odd % 2 == 0) { n_odd /= 2; s++; }

        int divs[200]; int nd = get_divisors(n_odd, divs);

        ull *all_sets[200];
        int set_sizes[200];

        for (int di = 0; di < nd; di++) {
            int d = divs[di];
            all_sets[di] = (ull *)malloc(200 * sizeof(ull));

            if (d == 1) {
                all_sets[di][0] = 1;
                set_sizes[di] = 1;
                continue;
            }

            ZPoly phi_d = get_cyclotomic(d);
            GF2Poly phi_gf2 = zpoly_to_gf2(&phi_d);

            int fd = ord2_mod(d);
            GF2Poly f = gf2_find_factor(phi_gf2, fd);

            int exponent = 1 << s;
            GF2Poly mod_poly = gf2_one();
            for (int i = 0; i < exponent; i++) {
                mod_poly = gf2_mul(mod_poly, f);
            }

            GF2Poly P = gf2_zero();
            gf2_set_bit(&P, 1);
            gf2_set_bit(&P, n - 1);
            gf2_update_deg(&P);

            GF2Poly base_elem = gf2_mod(P, mod_poly);

            if (gf2_is_zero(&base_elem)) {
                all_sets[di][0] = 1;
                set_sizes[di] = 1;
            } else {
                ull periods[200]; int np;
                get_component_periods(base_elem, mod_poly, f, periods, &np);
                memcpy(all_sets[di], periods, np * sizeof(ull));
                set_sizes[di] = np;
            }
        }

        /* Compute all LCMs */
        ull *current = (ull *)malloc(MAX_PERIODS * sizeof(ull));
        int cur_size = 1;
        current[0] = 1;

        for (int di = 0; di < nd; di++) {
            ull *next = (ull *)malloc(MAX_PERIODS * sizeof(ull));
            int next_size = 0;

            for (int i = 0; i < cur_size; i++) {
                for (int j = 0; j < set_sizes[di]; j++) {
                    ull val = lcm_ull(current[i], all_sets[di][j]);
                    int found = 0;
                    for (int k = 0; k < next_size; k++) {
                        if (next[k] == val) { found = 1; break; }
                    }
                    if (!found && next_size < MAX_PERIODS)
                        next[next_size++] = val;
                }
            }

            free(current);
            current = next;
            cur_size = next_size;
        }

        /* Add to master set */
        for (int i = 0; i < cur_size; i++) {
            int found = 0;
            for (int j = 0; j < master_size; j++) {
                if (master[j] == current[i]) { found = 1; break; }
            }
            if (!found) master[master_size++] = current[i];
        }

        free(current);
        for (int di = 0; di < nd; di++) free(all_sets[di]);
    }

    /* Sum all unique periods */
    ull total = 0;
    for (int i = 0; i < master_size; i++) total += master[i];
    free(master);

    printf("%llu\n", total);
    return 0;
}
