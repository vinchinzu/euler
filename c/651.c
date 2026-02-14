/*
 * Project Euler 651 - Patterns of Rectangular Stickers
 * Burnside's Lemma for counting patterns on a cylinder.
 * f(m, a, b) with Fibonacci numbers.
 */
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

typedef long long ll;
#define MOD 1000000007LL

static ll power_mod(ll base, ll exp, ll mod) {
    ll r = 1;
    base = ((base % mod) + mod) % mod;
    while (exp > 0) {
        if (exp & 1) r = r * base % mod;
        base = base * base % mod;
        exp >>= 1;
    }
    return r;
}

static ll mod_inverse(ll a, ll m) {
    return power_mod(a, m - 2, m);
}

/* Euler's totient */
static ll euler_phi(ll n) {
    ll result = n;
    ll temp = n;
    for (ll p = 2; p * p <= temp; p++) {
        if (temp % p == 0) {
            while (temp % p == 0) temp /= p;
            result = result / p * (p - 1);
        }
    }
    if (temp > 1) result = result / temp * (temp - 1);
    return result;
}

/* Get all divisors of n */
static int get_divisors(ll n, ll *divs) {
    int count = 0;
    for (ll i = 1; i * i <= n; i++) {
        if (n % i == 0) {
            divs[count++] = i;
            if (i != n / i)
                divs[count++] = n / i;
        }
    }
    /* Sort */
    for (int i = 0; i < count - 1; i++)
        for (int j = i + 1; j < count; j++)
            if (divs[i] > divs[j]) {
                ll tmp = divs[i]; divs[i] = divs[j]; divs[j] = tmp;
            }
    return count;
}

static ll gcd(ll a, ll b) {
    while (b) { ll t = b; b = a % b; a = t; }
    return a;
}

/* Binomial coefficient table (up to m=40) */
static ll nCr_table[50][50];

static void build_nCr(int max_n) {
    for (int i = 0; i <= max_n; i++) {
        nCr_table[i][0] = 1;
        for (int j = 1; j <= i; j++) {
            nCr_table[i][j] = (nCr_table[i-1][j-1] + nCr_table[i-1][j]) % MOD;
        }
    }
}

/*
 * Cycle structure for rotation of n elements:
 * For each divisor d of n, there are phi(n/d) rotations that produce
 * cycles of length n/d, each with d cycles.
 *
 * For reflection of n elements:
 * If n odd: all n reflections give (n-1)/2 pairs + 1 fixed = cycles of (2, (n-1)/2) and (1, 1)
 * If n even: n/2 reflections give (n/2) pairs = cycles of (2, n/2)
 *            n/2 reflections give (n/2-1) pairs + 2 fixed = cycles of (2, n/2-1) and (1, 2)
 */

/* cycle_lens structure: each entry is (cycle_len, count, multiplicity) */
/* For simplicity, store as arrays of (len, cnt) pairs with a multiplicity */

#define MAX_CYCLE_ENTRIES 200

typedef struct {
    int len;
    int cnt;
} CyclePair;

typedef struct {
    CyclePair pairs[10];
    int npairs;
    ll multiplicity;
} CycleInfo;

static int get_cycle_infos(ll n, int reflect, CycleInfo *infos) {
    int ninfos = 0;
    if (reflect) {
        if (n % 2 == 1) {
            infos[ninfos].npairs = 2;
            infos[ninfos].pairs[0].len = 2; infos[ninfos].pairs[0].cnt = (int)(n / 2);
            infos[ninfos].pairs[1].len = 1; infos[ninfos].pairs[1].cnt = 1;
            infos[ninfos].multiplicity = n;
            ninfos++;
        } else {
            infos[ninfos].npairs = 1;
            infos[ninfos].pairs[0].len = 2; infos[ninfos].pairs[0].cnt = (int)(n / 2);
            infos[ninfos].multiplicity = n / 2;
            ninfos++;

            infos[ninfos].npairs = 2;
            infos[ninfos].pairs[0].len = 2; infos[ninfos].pairs[0].cnt = (int)(n / 2 - 1);
            infos[ninfos].pairs[1].len = 1; infos[ninfos].pairs[1].cnt = 2;
            infos[ninfos].multiplicity = n / 2;
            ninfos++;
        }
    } else {
        ll divs[1000];
        int ndivs = get_divisors(n, divs);
        for (int i = 0; i < ndivs; i++) {
            ll d = divs[i];
            infos[ninfos].npairs = 1;
            infos[ninfos].pairs[0].len = (int)(n / d);
            infos[ninfos].pairs[0].cnt = (int)d;
            infos[ninfos].multiplicity = euler_phi(n / d);
            ninfos++;
        }
    }
    return ninfos;
}

static ll f_func(int m, ll a, ll b) {
    ll result = 0;

    for (int rw = 0; rw < 2; rw++) {
        for (int rb = 0; rb < 2; rb++) {
            CycleInfo infos_a[MAX_CYCLE_ENTRIES];
            CycleInfo infos_b[MAX_CYCLE_ENTRIES];
            int na = get_cycle_infos(a, rw, infos_a);
            int nb = get_cycle_infos(b, rb, infos_b);

            for (int ia = 0; ia < na; ia++) {
                for (int ib = 0; ib < nb; ib++) {
                    ll num_cycles = 0;
                    for (int pa = 0; pa < infos_a[ia].npairs; pa++) {
                        for (int pb = 0; pb < infos_b[ib].npairs; pb++) {
                            ll la = infos_a[ia].pairs[pa].len;
                            ll ca = infos_a[ia].pairs[pa].cnt;
                            ll lb = infos_b[ib].pairs[pb].len;
                            ll cb = infos_b[ib].pairs[pb].cnt;
                            num_cycles += gcd(la, lb) * ca * cb;
                        }
                    }

                    for (int i = 0; i < m; i++) {
                        ll sign = (i % 2 == 0) ? 1 : MOD - 1;
                        ll term = sign % MOD;
                        term = term * nCr_table[m][i] % MOD;
                        term = term * power_mod(m - i, num_cycles, MOD) % MOD;
                        term = term * (infos_a[ia].multiplicity % MOD) % MOD;
                        term = term * (infos_b[ib].multiplicity % MOD) % MOD;
                        result = (result + term) % MOD;
                    }
                }
            }
        }
    }

    ll inv = mod_inverse(4 * (a % MOD) % MOD * (b % MOD) % MOD, MOD);
    return result * inv % MOD;
}

int main(void) {
    int N = 40;
    build_nCr(45);

    /* Fibonacci numbers */
    ll fib[50];
    fib[0] = 0; fib[1] = 1;
    for (int i = 2; i <= N + 1; i++)
        fib[i] = fib[i-1] + fib[i-2];

    ll ans = 0;
    for (int i = 4; i <= N; i++) {
        ans = (ans + f_func(i, fib[i-1], fib[i])) % MOD;
    }

    printf("%lld\n", ans);
    return 0;
}
