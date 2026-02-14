/* Project Euler Problem 919 - Fortunate Triangles
 * S(P) = sum of a+b+c over all fortunate triangles with perimeter <= P.
 * Two generators based on quadratic forms.
 */
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <math.h>

typedef long long ll;

ll gcd_ll(ll a, ll b) {
    if (a < 0) a = -a;
    if (b < 0) b = -b;
    while (b) { ll t = b; b = a % b; a = t; }
    return a;
}

/* Hash set for storing primitive triangles (a,b,c) packed as ll */
#define HSIZE (1 << 22)
#define HMASK (HSIZE - 1)

typedef struct HSEntry { ll key; struct HSEntry *next; } HSEntry;
static HSEntry *hset[HSIZE];
#define HSPOOL (1 << 22)
static HSEntry hspool[HSPOOL];
static int hspool_idx = 0;

static int hset_contains(ll key) {
    unsigned int h = (unsigned int)((unsigned long long)key * 2654435761ULL) & HMASK;
    HSEntry *e = hset[h];
    while (e) { if (e->key == key) return 1; e = e->next; }
    return 0;
}

static void hset_add(ll key) {
    unsigned int h = (unsigned int)((unsigned long long)key * 2654435761ULL) & HMASK;
    HSEntry *e = hset[h];
    while (e) { if (e->key == key) return; e = e->next; }
    if (hspool_idx >= HSPOOL) return;
    e = &hspool[hspool_idx++];
    e->key = key;
    e->next = hset[h];
    hset[h] = e;
}

/* Pack triangle (a,b,c) where a<=b<=c, a,b,c < 10^7 */
ll pack_tri(ll a, ll b, ll c) {
    /* Sort */
    if (a > b) { ll t = a; a = b; b = t; }
    if (b > c) { ll t = b; b = c; c = t; }
    if (a > b) { ll t = a; a = b; b = t; }
    return a * 10000001LL * 10000001LL + b * 10000001LL + c;
}

void add_primitive(ll a, ll b, ll c, ll limit) {
    if (a <= 0 || b <= 0 || c <= 0) return;
    /* Sort */
    if (a > b) { ll t = a; a = b; b = t; }
    if (b > c) { ll t = b; b = c; c = t; }
    if (a > b) { ll t = a; a = b; b = t; }
    if (a + b <= c) return; /* Invalid triangle */
    ll g = gcd_ll(a, gcd_ll(b, c));
    a /= g; b /= g; c /= g;
    if (a + b + c > limit) return;
    ll key = pack_tri(a, b, c);
    hset_add(key);
}

/* Store all primitives for final summation */
typedef struct { ll a, b, c; } Tri;
static Tri all_tris[5000000];
static int ntris = 0;

void collect_primitives(ll limit) {
    ntris = 0;
    for (int h = 0; h < HSIZE; h++) {
        HSEntry *e = hset[h];
        while (e) {
            ll key = e->key;
            ll c = key % 10000001LL;
            key /= 10000001LL;
            ll b = key % 10000001LL;
            ll a = key / 10000001LL;
            if (ntris < 5000000) {
                all_tris[ntris].a = a;
                all_tris[ntris].b = b;
                all_tris[ntris].c = c;
                ntris++;
            }
            e = e->next;
        }
    }
}

int main(void) {
    ll limit = 10000000LL; /* 10^7 */

    memset(hset, 0, sizeof(hset));

    /* Generator 1: u^2 + 15v^2 */
    int max_v1 = (int)(sqrt(2.5 * limit / 15.0)) + 2;
    int max_u1 = (int)(sqrt(2.5 * limit)) + 2;

    for (int v = 1; v < max_v1; v++) {
        for (int u = 1; u < max_u1; u++) {
            if (gcd_ll(u, v) != 1) continue;

            if ((u % 2 != 0) && (v % 2 != 0)) {
                ll c_val = (ll)u*u + 15LL*v*v;
                if (c_val % 4 != 0) continue;
                ll c = c_val / 4;
                ll b = 2LL * u * v;

                ll val1 = 15LL*v*v - (ll)u*u + 2LL*u*v;
                if (val1 % 4 == 0) {
                    ll a1 = val1 < 0 ? -val1 / 4 : val1 / 4;
                    if (a1 > 0) add_primitive(a1, b, c, limit);
                }

                ll val2 = 15LL*v*v - (ll)u*u - 2LL*u*v;
                if (val2 % 4 == 0) {
                    ll a2 = val2 < 0 ? -val2 / 4 : val2 / 4;
                    if (a2 > 0) add_primitive(a2, b, c, limit);
                }
            } else {
                ll c = (ll)u*u + 15LL*v*v;
                ll b = 8LL * u * v;
                ll val1 = 15LL*v*v - (ll)u*u + 2LL*u*v;
                ll a1 = val1 < 0 ? -val1 : val1;
                if (a1 > 0) add_primitive(a1, b, c, limit);

                ll val2 = 15LL*v*v - (ll)u*u - 2LL*u*v;
                ll a2 = val2 < 0 ? -val2 : val2;
                if (a2 > 0) add_primitive(a2, b, c, limit);
            }
        }
    }

    /* Generator 2: 3u^2 + 5v^2 */
    int max_v2 = (int)(sqrt(2.5 * limit / 5.0)) + 2;
    int max_u2 = (int)(sqrt(2.5 * limit / 3.0)) + 2;

    for (int v = 1; v < max_v2; v++) {
        for (int u = 1; u < max_u2; u++) {
            if (gcd_ll(u, v) != 1) continue;

            if ((u % 2 != 0) && (v % 2 != 0)) {
                ll c_val = 3LL*u*u + 5LL*v*v;
                if (c_val % 4 != 0) continue;
                ll c = c_val / 4;
                ll b = 2LL * u * v;

                ll val1 = 5LL*v*v - 3LL*u*u + 2LL*u*v;
                if (val1 % 4 == 0) {
                    ll a1 = val1 < 0 ? -val1 / 4 : val1 / 4;
                    if (a1 > 0) add_primitive(a1, b, c, limit);
                }

                ll val2 = 5LL*v*v - 3LL*u*u - 2LL*u*v;
                if (val2 % 4 == 0) {
                    ll a2 = val2 < 0 ? -val2 / 4 : val2 / 4;
                    if (a2 > 0) add_primitive(a2, b, c, limit);
                }
            } else {
                ll c = 3LL*u*u + 5LL*v*v;
                ll b = 8LL * u * v;
                ll val1 = 5LL*v*v - 3LL*u*u + 2LL*u*v;
                ll a1 = val1 < 0 ? -val1 : val1;
                if (a1 > 0) add_primitive(a1, b, c, limit);

                ll val2 = 5LL*v*v - 3LL*u*u - 2LL*u*v;
                ll a2 = val2 < 0 ? -val2 : val2;
                if (a2 > 0) add_primitive(a2, b, c, limit);
            }
        }
    }

    /* Collect and sum */
    collect_primitives(limit);

    ll ans = 0;
    for (int i = 0; i < ntris; i++) {
        ll p = all_tris[i].a + all_tris[i].b + all_tris[i].c;
        if (p > limit) continue;
        ll count = limit / p;
        ans += p * count * (count + 1) / 2;
    }

    printf("%lld\n", ans);
    return 0;
}
