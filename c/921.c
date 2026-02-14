/* Project Euler 921 - Fibonacci-based sum with Pisano period and matrix exponentiation
 * M = 398874989 (prime), compute S = sum_{i=2}^{1618034} (p^5 + q^5) mod M
 * where p = F_K/2, q = L_K/2, K = 3 * 5^{F_i mod L} mod pi_M
 * pi_M = Pisano period of M, L = order of 5 mod pi_M
 */
#include <stdio.h>
#include <stdlib.h>

typedef long long ll;
typedef __int128 lll;
#define M_VAL 398874989LL

ll mod_pow(ll base, ll exp, ll mod) {
    ll result = 1;
    base %= mod;
    if (base < 0) base += mod;
    while (exp > 0) {
        if (exp & 1) result = (lll)result * base % mod;
        base = (lll)base * base % mod;
        exp >>= 1;
    }
    return result;
}

/* Fibonacci mod m using matrix exponentiation */
/* Returns (F_n mod m, F_{n+1} mod m) */
void fib_mod(ll n, ll m, ll *fn, ll *fn1) {
    if (n == 0) { *fn = 0; *fn1 = 1; return; }
    /* Matrix [[1,1],[1,0]]^n */
    ll a00=1, a01=1, a10=1, a11=0; /* matrix A */
    ll r00=1, r01=0, r10=0, r11=1; /* result = identity */
    ll p = n;
    while (p > 0) {
        if (p & 1) {
            ll t00 = ((lll)r00*a00 + (lll)r01*a10) % m;
            ll t01 = ((lll)r00*a01 + (lll)r01*a11) % m;
            ll t10 = ((lll)r10*a00 + (lll)r11*a10) % m;
            ll t11 = ((lll)r10*a01 + (lll)r11*a11) % m;
            r00=t00; r01=t01; r10=t10; r11=t11;
        }
        ll t00 = ((lll)a00*a00 + (lll)a01*a10) % m;
        ll t01 = ((lll)a00*a01 + (lll)a01*a11) % m;
        ll t10 = ((lll)a10*a00 + (lll)a11*a10) % m;
        ll t11 = ((lll)a10*a01 + (lll)a11*a11) % m;
        a00=t00; a01=t01; a10=t10; a11=t11;
        p >>= 1;
    }
    /* Result matrix R = M^n. F_n = R[1][0], F_{n+1} = R[0][0] */
    *fn = r10;
    *fn1 = r00;
}

/* Find Pisano period of prime p */
ll find_pisano_period(ll p) {
    ll candidate;
    if (p % 5 == 1 || p % 5 == 4)
        candidate = p - 1;
    else
        candidate = 2 * (p + 1);

    /* Find divisors of candidate, check smallest */
    int ndivs = 0;
    ll *divs = (ll *)malloc(100000 * sizeof(ll));
    for (ll i = 1; i * i <= candidate; i++) {
        if (candidate % i == 0) {
            divs[ndivs++] = i;
            if (i * i != candidate) divs[ndivs++] = candidate / i;
        }
    }
    /* Sort divisors */
    for (int i = 0; i < ndivs - 1; i++)
        for (int j = i + 1; j < ndivs; j++)
            if (divs[i] > divs[j]) { ll t = divs[i]; divs[i] = divs[j]; divs[j] = t; }

    for (int i = 0; i < ndivs; i++) {
        ll d = divs[i];
        ll fn, fn1;
        fib_mod(d, p, &fn, &fn1);
        ll fn1b, fn2;
        fib_mod(d + 1, p, &fn1b, &fn2);
        if (fn == 0 && fn1b == 1) {
            free(divs);
            return d;
        }
    }
    free(divs);
    return candidate;
}

/* Find order of a mod m by checking divisors of phi(m) */
ll find_order(ll a, ll m, ll phi_m) {
    /* Factor phi_m */
    ll temp = phi_m;
    ll pfactors[64];
    int npf = 0;
    ll d = 2;
    while (d * d <= temp) {
        if (temp % d == 0) {
            pfactors[npf++] = d;
            while (temp % d == 0) temp /= d;
        }
        d++;
    }
    if (temp > 1) pfactors[npf++] = temp;

    ll order = phi_m;
    for (int i = 0; i < npf; i++) {
        while (order % pfactors[i] == 0 && mod_pow(a, order / pfactors[i], m) == 1)
            order /= pfactors[i];
    }
    return order;
}

int main(void) {
    ll pi_M = find_pisano_period(M_VAL);
    /* pi_M = 199437494 */

    /* Factor pi_M to find phi(pi_M) */
    ll temp = pi_M;
    ll factors[32];
    int exps[32];
    int nfactors = 0;
    ll d = 2;
    while (d * d <= temp) {
        if (temp % d == 0) {
            factors[nfactors] = d;
            exps[nfactors] = 0;
            while (temp % d == 0) { exps[nfactors]++; temp /= d; }
            nfactors++;
        }
        d++;
    }
    if (temp > 1) { factors[nfactors] = temp; exps[nfactors] = 1; nfactors++; }

    ll phi_pi = 1;
    for (int i = 0; i < nfactors; i++) {
        ll pw = 1;
        for (int j = 0; j < exps[i] - 1; j++) pw *= factors[i];
        phi_pi *= (factors[i] - 1) * pw;
    }

    ll L = find_order(5, pi_M, phi_pi);

    ll inv_2 = (M_VAL + 1) / 2;
    ll total_S = 0;
    ll a_fib = 1; /* F_1 mod L */
    ll b_fib = 1; /* F_2 mod L */
    int m_limit = 1618034;

    for (int i = 2; i <= m_limit; i++) {
        ll K = 3 * mod_pow(5, b_fib, pi_M) % pi_M;
        ll F_K, F_K1;
        fib_mod(K, M_VAL, &F_K, &F_K1);
        /* L_K = F_{K-1} + F_{K+1} = 2*F_{K+1} - F_K (since F_{K+1} = F_K + F_{K-1}) */
        /* Actually L_n = F_{n-1} + F_{n+1}. F_{n+1} = F_K1 (from fib_mod(K,...)).
         * F_{n-1} = F_K1 - F_K. So L_K = (F_K1 - F_K) + next?
         * Wait: fib_mod(K, m, &fn, &fn1) gives fn=F_K, fn1=F_{K+1}?
         * No. Let me re-check. fib_mod returns R[1][0] = F_n and R[0][0] = F_{n+1}?
         * Actually with M^n, M=[[1,1],[1,0]], M^n = [[F_{n+1}, F_n], [F_n, F_{n-1}]]
         * So r10 = F_n, r00 = F_{n+1}. Correct.
         * L_n = F_{n-1} + F_{n+1}. F_{n-1} = F_{n+1} - F_n = F_K1 - F_K.
         * L_K = (F_K1 - F_K + M_VAL) % M_VAL + F_K1 = (2*F_K1 - F_K + M_VAL) % M_VAL
         */
        ll L_K = (2 * F_K1 % M_VAL - F_K + M_VAL) % M_VAL;

        ll p = (lll)F_K * inv_2 % M_VAL;
        ll q = (lll)L_K * inv_2 % M_VAL;

        ll term = (mod_pow(p, 5, M_VAL) + mod_pow(q, 5, M_VAL)) % M_VAL;
        total_S = (total_S + term) % M_VAL;

        ll new_b = (a_fib + b_fib) % L;
        a_fib = b_fib;
        b_fib = new_b;
    }

    printf("%lld\n", total_S);
    return 0;
}
