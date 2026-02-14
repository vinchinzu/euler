/* Project Euler 972 - Hyperbolic geodesics T(12) = 3575508
 * Points (x,y) with rational coords, denominator <= 12, inside unit disc.
 * Count ordered triples on a common geodesic (diameter or orthogonal circle).
 * 
 * Key insight: for each GEODESIC, count the number of V(12) points on it,
 * then sum s*(s-1)*(s-2) over all geodesics.
 *
 * Geodesics: 
 * 1) Diameters (lines through origin) - enumerate directions
 * 2) Circles orthogonal to unit circle: x^2+y^2-2hx-2ky+1=0
 *    Parameterized by (h,k). Two points determine (h,k) if non-collinear-through-origin.
 *
 * Approach: for each unordered pair of points, compute the unique geodesic.
 * Use a hash map keyed by geodesic to count points per geodesic.
 * Then sum s*(s-1)*(s-2).
 *
 * But we must avoid double-counting: each geodesic with s points is found
 * C(s,2) times (once per pair). So we accumulate points per geodesic and
 * then compute the triples.
 */
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

typedef long long ll;

int gcd_val(int a, int b) {
    if (a < 0) a = -a; if (b < 0) b = -b;
    while (b) { int t = b; b = a % b; a = t; }
    return a;
}

ll gcd_ll(ll a, ll b) {
    if (a < 0) a = -a; if (b < 0) b = -b;
    while (b) { ll t = b; b = a % b; a = t; }
    return a;
}

typedef struct {
    int xn, xd, yn, yd; /* x = xn/xd, y = yn/yd in lowest terms */
} Point;

/* Geodesic representation: 
 * Type 0: diameter, direction (da, db) normalized
 * Type 1: orthogonal circle, (h,k) as exact rationals (hn/hd, kn/kd)
 *   From x^2+y^2+1 = 2hx+2ky, given two points P1,P2:
 *   2h*x1 + 2k*y1 = x1^2+y1^2+1
 *   2h*x2 + 2k*y2 = x2^2+y2^2+1
 *   Solve for (h,k).
 */

/* Hash map for counting points per geodesic */
/* Key: canonical representation of the geodesic */
/* For diameter: (0, da, db, 0, 0, 0) */  
/* For circle: (1, hn, hd, kn, kd, 0) in lowest terms */

#define HASH_SIZE (1 << 20)
#define HASH_MASK (HASH_SIZE - 1)

typedef struct Entry {
    int type; /* 0 = diameter, 1 = circle */
    ll a, b, c, d; /* canonical key */
    int count;
    int *pts; /* list of point indices */
    int cap;
    struct Entry *next;
} Entry;

Entry *htable[HASH_SIZE];

unsigned int hash_key(int type, ll a, ll b, ll c, ll d) {
    unsigned long long h = type * 1000000007ULL + (unsigned long long)(a + 1000000) * 1000003ULL 
                         + (unsigned long long)(b + 1000000) * 999983ULL 
                         + (unsigned long long)(c + 1000000) * 999979ULL
                         + (unsigned long long)(d + 1000000) * 999961ULL;
    return (unsigned int)(h ^ (h >> 17)) & HASH_MASK;
}

Entry *find_or_create(int type, ll a, ll b, ll c, ll d) {
    unsigned int h = hash_key(type, a, b, c, d);
    Entry *e = htable[h];
    while (e) {
        if (e->type == type && e->a == a && e->b == b && e->c == c && e->d == d) return e;
        e = e->next;
    }
    e = (Entry *)calloc(1, sizeof(Entry));
    e->type = type; e->a = a; e->b = b; e->c = c; e->d = d;
    e->count = 0; e->cap = 4;
    e->pts = (int *)malloc(4 * sizeof(int));
    e->next = htable[h];
    htable[h] = e;
    return e;
}

void add_point(Entry *e, int pt_idx) {
    /* Check if already present */
    for (int i = 0; i < e->count; i++) if (e->pts[i] == pt_idx) return;
    if (e->count >= e->cap) {
        e->cap *= 2;
        e->pts = (int *)realloc(e->pts, e->cap * sizeof(int));
    }
    e->pts[e->count++] = pt_idx;
}

int main(void) {
    int N = 12;
    memset(htable, 0, sizeof(htable));
    
    /* Generate all distinct rationals with denominator <= N in (-1,1) */
    int max_rats = 2000;
    int *rn = (int *)malloc(max_rats * sizeof(int));
    int *rd = (int *)malloc(max_rats * sizeof(int));
    int nrats = 0;
    rn[nrats] = 0; rd[nrats] = 1; nrats++;
    for (int q = 1; q <= N; q++) {
        for (int p = -(q-1); p <= q-1; p++) {
            if (p == 0) continue;
            if (gcd_val(abs(p), q) == 1) {
                rn[nrats] = p; rd[nrats] = q; nrats++;
            }
        }
    }
    
    /* Generate all points in V(N) */
    int max_pts = 10000;
    Point *pts = (Point *)malloc(max_pts * sizeof(Point));
    int npts = 0;
    for (int i = 0; i < nrats; i++) {
        for (int j = 0; j < nrats; j++) {
            ll x2 = (ll)rn[i]*rn[i] * (ll)rd[j]*rd[j];
            ll y2 = (ll)rn[j]*rn[j] * (ll)rd[i]*rd[i];
            ll r2 = (ll)rd[i]*rd[i] * (ll)rd[j]*rd[j];
            if (x2 + y2 < r2) {
                pts[npts].xn = rn[i]; pts[npts].xd = rd[i];
                pts[npts].yn = rn[j]; pts[npts].yd = rd[j];
                npts++;
            }
        }
    }
    
    /* For each pair of distinct points, determine their geodesic */
    for (int i = 0; i < npts; i++) {
        /* x1 = xn1/xd1, y1 = yn1/yd1 */
        ll xn1 = pts[i].xn, xd1 = pts[i].xd;
        ll yn1 = pts[i].yn, yd1 = pts[i].yd;
        /* A1 = xn1*yd1, B1 = yn1*xd1, D1 = xd1*yd1 */
        
        for (int j = i+1; j < npts; j++) {
            ll xn2 = pts[j].xn, xd2 = pts[j].xd;
            ll yn2 = pts[j].yn, yd2 = pts[j].yd;
            
            /* Check if collinear through origin: x1*y2 = x2*y1 */
            /* xn1/xd1 * yn2/yd2 = xn2/xd2 * yn1/yd1 */
            /* xn1*yn2*xd2*yd1 = xn2*yn1*xd1*yd2 */
            ll lhs = xn1 * yn2 * xd2 * yd1;
            ll rhs = xn2 * yn1 * xd1 * yd2;
            
            if (lhs == rhs) {
                /* On a diameter. Compute direction. */
                /* Direction of P1 from origin: (xn1*yd1, yn1*xd1) = (A1, B1) */
                /* But both points are on the same line through origin */
                /* Use direction of P1 (or P2 if P1 is origin) */
                ll da, db;
                if (xn1 != 0 || yn1 != 0) {
                    da = xn1 * yd1; db = yn1 * xd1;
                } else {
                    da = xn2 * yd2; db = yn2 * xd2;
                }
                /* Normalize direction */
                if (da == 0 && db == 0) continue; /* both origin? shouldn't happen if distinct */
                ll g = gcd_ll(da < 0 ? -da : da, db < 0 ? -db : db);
                da /= g; db /= g;
                if (da < 0 || (da == 0 && db < 0)) { da = -da; db = -db; }
                
                Entry *e = find_or_create(0, da, db, 0, 0);
                add_point(e, i);
                add_point(e, j);
            } else {
                /* Determine the orthogonal circle through P1 and P2 */
                /* 2h*x1 + 2k*y1 = x1^2 + y1^2 + 1 ... (i)
                   2h*x2 + 2k*y2 = x2^2 + y2^2 + 1 ... (ii) */
                /* Let s1 = x1^2+y1^2+1 = xn1^2/xd1^2 + yn1^2/yd1^2 + 1
                         = (xn1^2*yd1^2 + yn1^2*xd1^2 + xd1^2*yd1^2) / (xd1^2*yd1^2)
                   Let s1_num = xn1*xn1*yd1*yd1 + yn1*yn1*xd1*xd1 + xd1*xd1*yd1*yd1
                   Let s1_den = xd1*xd1*yd1*yd1
                   Similarly for s2. */
                ll D1 = xd1*yd1, D2 = xd2*yd2;
                ll A1 = xn1*yd1, B1 = yn1*xd1; /* x1 = A1/D1, y1 = B1/D1 */
                ll A2 = xn2*yd2, B2 = yn2*xd2;
                ll s1_num = A1*A1 + B1*B1 + D1*D1; /* s1 = s1_num / D1^2 */
                ll s2_num = A2*A2 + B2*B2 + D2*D2;
                
                /* System: 2h*(A1/D1) + 2k*(B1/D1) = s1_num/D1^2
                   → 2h*A1*D1 + 2k*B1*D1 = s1_num
                   → 2h*A1 + 2k*B1 = s1_num/D1   ... but D1 might not divide s1_num cleanly
                   Actually: 2h*A1/D1 + 2k*B1/D1 = s1_num/D1^2
                   Multiply by D1: 2h*A1 + 2k*B1 = s1_num/D1
                   Multiply by D1 again: 2h*A1*D1 + 2k*B1*D1 = s1_num
                   Similarly: 2h*A2*D2 + 2k*B2*D2 = s2_num
                   
                   Let a11 = A1*D1, a12 = B1*D1, b_1 = s1_num
                       a21 = A2*D2, a22 = B2*D2, b_2 = s2_num
                   Solve 2*[a11 a12; a21 a22]*[h;k] = [b1;b2]
                   det = a11*a22 - a12*a21
                   h = (b1*a22 - b2*a12) / (2*det)
                   k = (a11*b2 - a21*b1) / (2*det) */
                
                ll a11 = A1*D1, a12 = B1*D1;
                ll a21 = A2*D2, a22 = B2*D2;
                ll det = a11*a22 - a12*a21;
                if (det == 0) continue; /* shouldn't happen since not collinear through origin... but could be parallel */
                
                ll h_num = s1_num*a22 - s2_num*a12; /* h = h_num / (2*det) */
                ll k_num = a11*s2_num - a21*s1_num;
                ll h_den = 2*det;
                ll k_den = 2*det;
                
                /* Reduce h_num/h_den and k_num/k_den */
                ll g1 = gcd_ll(h_num < 0 ? -h_num : h_num, h_den < 0 ? -h_den : h_den);
                if (g1 == 0) g1 = 1;
                h_num /= g1; h_den /= g1;
                if (h_den < 0) { h_num = -h_num; h_den = -h_den; }
                
                ll g2 = gcd_ll(k_num < 0 ? -k_num : k_num, k_den < 0 ? -k_den : k_den);
                if (g2 == 0) g2 = 1;
                k_num /= g2; k_den /= g2;
                if (k_den < 0) { k_num = -k_num; k_den = -k_den; }
                
                Entry *e = find_or_create(1, h_num, h_den, k_num, k_den);
                add_point(e, i);
                add_point(e, j);
            }
        }
    }
    
    /* Sum s*(s-1)*(s-2) over all geodesics */
    ll total = 0;
    for (int h = 0; h < HASH_SIZE; h++) {
        for (Entry *e = htable[h]; e; e = e->next) {
            int s = e->count;
            if (s >= 3) total += (ll)s * (s-1) * (s-2);
        }
    }
    
    printf("%lld\n", total);
    
    free(pts); free(rn); free(rd);
    return 0;
}
