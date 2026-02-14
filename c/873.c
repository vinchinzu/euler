#include <stdio.h>

/*
 * Project Euler 873 - W(p,q,r) words with separation constraint
 *
 * Every A separated from every B by at least two Cs.
 * W(10^6, 10^7, 10^8) mod 10^9+7.
 *
 * Uses stars-and-bars approach with iterating over run counts.
 */

typedef long long ll;

#define MOD 1000000007LL

ll power(ll base, ll exp, ll mod) {
    ll res = 1;
    base %= mod;
    while (exp > 0) {
        if (exp & 1) res = res * base % mod;
        base = base * base % mod;
        exp >>= 1;
    }
    return res;
}

ll mod_inv(ll a, ll mod) {
    return power(a, mod - 2, mod);
}

int main(void) {
    ll p = 1000000, q = 10000000, r = 100000000;

    if (r < 2) {
        printf("0\n");
        return 0;
    }

    ll K_sb = p + q;
    ll inv10_not_needed = 0; /* placeholder */

    /* Initial SB: binom(r-2+p+q, p+q) */
    ll curr_N_sb = r - 2 + p + q;

    /* Compute initial binom(curr_N_sb, K_sb) */
    ll sb_num = 1, sb_den = 1;
    for (ll i = 0; i < K_sb; i++) {
        sb_num = sb_num % MOD * ((curr_N_sb - i) % MOD + MOD) % MOD;
        sb_den = sb_den % MOD * ((i + 1) % MOD) % MOD;
    }
    ll curr_sb_val = sb_num % MOD * mod_inv(sb_den, MOD) % MOD;

    /* Initialize combinations for runs */
    ll comb_p = 1; /* binom(p-1, 0) */
    ll comb_q = 1; /* binom(q-1, 0) */

    ll ans = 0;
    ll m = 1;

    while (1) {
        /* Case k = 2m: Runs (m, m) */
        if (m <= p && m <= q) {
            ll term = 2 * comb_p % MOD * comb_q % MOD;
            term = term * curr_sb_val % MOD;
            ans = (ans + term) % MOD;
        }

        /* Update SB value: decrease N by 2 */
        /* Step 1 */
        if (curr_N_sb - K_sb < 0) {
            curr_sb_val = 0;
        } else {
            ll factor = (curr_N_sb - K_sb) % MOD;
            if (factor < 0) factor += MOD;
            factor = factor * mod_inv(curr_N_sb % MOD, MOD) % MOD;
            curr_sb_val = curr_sb_val * factor % MOD;
            curr_N_sb--;

            /* Step 2 */
            if (curr_N_sb == 0 && K_sb > 0) {
                curr_sb_val = 0;
            } else {
                factor = (curr_N_sb - K_sb) % MOD;
                if (factor < 0) factor += MOD;
                factor = factor * mod_inv(curr_N_sb % MOD, MOD) % MOD;
                curr_sb_val = curr_sb_val * factor % MOD;
                curr_N_sb--;
            }
        }

        if (curr_sb_val == 0) break;

        /* Case k = 2m+1 */
        ll inv_m = mod_inv(m % MOD, MOD);

        ll next_comb_p = 0;
        if (m <= p - 1) {
            next_comb_p = comb_p * ((p - m) % MOD + MOD) % MOD;
            next_comb_p = next_comb_p * inv_m % MOD;
        }

        ll next_comb_q = 0;
        if (m <= q - 1) {
            next_comb_q = comb_q * ((q - m) % MOD + MOD) % MOD;
            next_comb_q = next_comb_q * inv_m % MOD;
        }

        ll term_odd = 0;

        /* Subcase 1: A-runs=m+1, B-runs=m */
        if (m + 1 <= p && m <= q) {
            ll t = next_comb_p * comb_q % MOD;
            term_odd = (term_odd + t) % MOD;
        }

        /* Subcase 2: A-runs=m, B-runs=m+1 */
        if (m <= p && m + 1 <= q) {
            ll t = comb_p * next_comb_q % MOD;
            term_odd = (term_odd + t) % MOD;
        }

        term_odd = term_odd * curr_sb_val % MOD;
        ans = (ans + term_odd) % MOD;

        /* Update SB value for next m */
        if (curr_N_sb <= 0) {
            curr_sb_val = 0;
        } else {
            if (curr_N_sb - K_sb < 0) {
                curr_sb_val = 0;
            } else {
                ll factor = (curr_N_sb - K_sb) % MOD;
                if (factor < 0) factor += MOD;
                factor = factor * mod_inv(curr_N_sb % MOD, MOD) % MOD;
                curr_sb_val = curr_sb_val * factor % MOD;
            }
            curr_N_sb--;

            if (curr_N_sb <= 0) {
                curr_sb_val = 0;
            } else {
                if (curr_N_sb - K_sb < 0) {
                    curr_sb_val = 0;
                } else {
                    ll factor = (curr_N_sb - K_sb) % MOD;
                    if (factor < 0) factor += MOD;
                    factor = factor * mod_inv(curr_N_sb % MOD, MOD) % MOD;
                    curr_sb_val = curr_sb_val * factor % MOD;
                }
                curr_N_sb--;
            }
        }

        comb_p = next_comb_p;
        comb_q = next_comb_q;
        m++;

        if (comb_p == 0 && comb_q == 0) break;
        if (m > p && m > q) break;
    }

    printf("%lld\n", ans);
    return 0;
}
