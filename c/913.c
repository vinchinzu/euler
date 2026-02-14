/* Project Euler Problem 913 - Matrix Shuffles
 * For (n,m) with 2<=n<=m<=100, compute S(n,m) for the n^4,m^4 case.
 * S = (nm)^4 - cycle_count - 1
 * cycle_count = sum phi(d)/ord_d(multiplier) over divisors of L=(nm)^4-1.
 */
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

typedef long long ll;
typedef unsigned long long ull;
typedef __int128 lll;

ll gcd_ll(ll a, ll b) {
    if (a < 0) a = -a;
    if (b < 0) b = -b;
    while (b) { ll t = b; b = a % b; a = t; }
    return a;
}

ll lcm_ll(ll a, ll b) {
    return a / gcd_ll(a, b) * b;
}

/* Trial division factorization of n into primes */
typedef struct { ll prime; int exp; } Factor;

int factorize(ll n, Factor *factors) {
    int cnt = 0;
    for (ll d = 2; d * d <= n; d++) {
        if (n % d == 0) {
            factors[cnt].prime = d;
            factors[cnt].exp = 0;
            while (n % d == 0) {
                factors[cnt].exp++;
                n /= d;
            }
            cnt++;
        }
    }
    if (n > 1) {
        factors[cnt].prime = n;
        factors[cnt].exp = 1;
        cnt++;
    }
    return cnt;
}

ll pow_mod_ll(ll a, ll b, ll m) {
    ll res = 1;
    a %= m;
    if (a < 0) a += m;
    while (b > 0) {
        if (b & 1) res = (lll)res * a % m;
        a = (lll)a * a % m;
        b >>= 1;
    }
    return res;
}

/* Compute order of a mod p^k */
ll get_order_mod_pk(ll a, ll p, int k) {
    ll pk = 1;
    for (int i = 0; i < k; i++) pk *= p;

    if (p == 2) {
        ll order = 1;
        ll curr = a % pk;
        if (curr < 0) curr += pk;
        if (curr == 1) return 1;
        for (int i = 1; i <= k; i++) {
            order *= 2;
            curr = (lll)curr * curr % pk;
            if (curr == 1) return order;
        }
        return order;
    }

    /* For odd p: find order mod p first */
    ll p_minus_1 = p - 1;
    Factor pf[64];
    int npf = factorize(p_minus_1, pf);

    ll order = p_minus_1;
    for (int i = 0; i < npf; i++) {
        ll q = pf[i].prime;
        while (order % q == 0) {
            ll temp_order = order / q;
            if (pow_mod_ll(a, temp_order, p) == 1)
                order = temp_order;
            else
                break;
        }
    }

    /* Lift to p^k */
    ll total_mod = pk;
    ll curr_order = order;
    while (1) {
        if (pow_mod_ll(a, curr_order, total_mod) == 1)
            return curr_order;
        curr_order *= p;
    }
}

/* Precomputed data per prime factor */
typedef struct {
    int num_entries; /* 0..e entries */
    ll phi_val[64];  /* phi(p^j) for j=0..e */
    ll ord_val[64];  /* ord mod p^j for j=0..e */
} PrimeData;

ll count_cycles(int n, int m) {
    ll X = (ll)n * m;
    /* L = X^4 - 1 = (X-1)(X+1)(X^2+1) */
    /* Factor each part */
    Factor all_factors[256];
    int total_factors = 0;

    ll parts[3] = { X - 1, X + 1, X * X + 1 };
    Factor pf[64];

    /* Merge all prime factors */
    typedef struct { ll prime; int exp; } PF;
    PF merged[256];
    int nmerged = 0;

    for (int pi = 0; pi < 3; pi++) {
        int nf = factorize(parts[pi], pf);
        for (int i = 0; i < nf; i++) {
            int found = 0;
            for (int j = 0; j < nmerged; j++) {
                if (merged[j].prime == pf[i].prime) {
                    merged[j].exp += pf[i].exp;
                    found = 1;
                    break;
                }
            }
            if (!found) {
                merged[nmerged].prime = pf[i].prime;
                merged[nmerged].exp = pf[i].exp;
                nmerged++;
            }
        }
    }

    ll multiplier_raw = (ll)n * n * n * n; /* n^4 */

    /* Precompute data for each prime */
    PrimeData pd[256];
    for (int i = 0; i < nmerged; i++) {
        ll p = merged[i].prime;
        int e = merged[i].exp;
        pd[i].num_entries = e + 1;

        /* j=0: phi=1, ord=1 */
        pd[i].phi_val[0] = 1;
        pd[i].ord_val[0] = 1;

        /* j=1 */
        ll ord_p = get_order_mod_pk(multiplier_raw, p, 1);
        ll curr_ord = ord_p;
        pd[i].phi_val[1] = p - 1;
        pd[i].ord_val[1] = curr_ord;

        ll current_p_pow = p;
        for (int j = 2; j <= e; j++) {
            current_p_pow *= p;
            if (pow_mod_ll(multiplier_raw, curr_ord, current_p_pow) != 1)
                curr_ord *= p;
            ll phi_j = 1;
            ll pp = 1;
            for (int t = 0; t < j - 1; t++) pp *= p;
            phi_j = pp * (p - 1);
            pd[i].phi_val[j] = phi_j;
            pd[i].ord_val[j] = curr_ord;
        }
    }

    /* DFS to sum phi(d)/ord_d(mult) */
    typedef struct { int idx; ll lcm_ord; ll phi_prod; } StackEntry;
    StackEntry stack[1000000];
    int sp = 0;
    stack[sp++] = (StackEntry){0, 1, 1};

    ll total_sum = 0;

    while (sp > 0) {
        StackEntry se = stack[--sp];
        if (se.idx == nmerged) {
            total_sum += se.phi_prod / se.lcm_ord;
            continue;
        }
        int idx = se.idx;
        for (int j = 0; j < pd[idx].num_entries; j++) {
            ll phi_part = pd[idx].phi_val[j];
            ll ord_part = pd[idx].ord_val[j];
            ll g = gcd_ll(se.lcm_ord, ord_part);
            ll new_lcm = se.lcm_ord / g * ord_part;
            stack[sp++] = (StackEntry){idx + 1, new_lcm, se.phi_prod * phi_part};
        }
    }

    return total_sum;
}

int main(void) {
    ll total = 0;
    for (int n = 2; n <= 100; n++) {
        for (int m = n; m <= 100; m++) {
            ll size = 1;
            for (int i = 0; i < 4; i++) size *= (ll)n * m;
            if (size <= 2) continue;
            ll cycle_term = count_cycles(n, m);
            ll val = size - cycle_term - 1;
            total += val;
        }
    }
    printf("%lld\n", total);
    return 0;
}
