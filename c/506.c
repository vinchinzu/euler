/*
 * Project Euler Problem 506: Clock sequence.
 * Uses Berlekamp-Massey + Kitamasa via CRT for composite modulus.
 * M = 123454321 = 41^2 * 271^2.
 */
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <stdint.h>

typedef long long ll;
typedef __int128 lll;

/* ============= Berlekamp-Massey mod prime ============= */
ll power_mod(ll base, ll exp, ll mod) {
    ll result = 1;
    base %= mod;
    if (base < 0) base += mod;
    while (exp > 0) {
        if (exp & 1) result = result * base % mod;
        base = base * base % mod;
        exp >>= 1;
    }
    return result;
}

/* Returns recurrence length L, fills coeffs[1..L] with -C[1..-L] */
int berlekamp_massey(ll *seq, int n, ll *coeffs, ll mod) {
    ll *C = (ll*)calloc(n + 2, sizeof(ll));
    ll *B = (ll*)calloc(n + 2, sizeof(ll));
    ll *T = (ll*)calloc(n + 2, sizeof(ll));
    C[0] = B[0] = 1;
    int L = 0, m = 1;
    ll b = 1;

    for (int i = 0; i < n; i++) {
        ll d = seq[i];
        for (int j = 1; j <= L; j++)
            d = (d + C[j] * seq[i - j]) % mod;
        d = ((d % mod) + mod) % mod;

        if (d == 0) {
            m++;
        } else if (2 * L <= i) {
            memcpy(T, C, (n + 2) * sizeof(ll));
            ll coeff = d * power_mod(b, mod - 2, mod) % mod;
            for (int j = m; j <= n; j++)
                C[j] = ((C[j] - coeff * B[j - m]) % mod + mod) % mod;
            memcpy(B, T, (n + 2) * sizeof(ll));
            L = i + 1 - L;
            b = d;
            m = 1;
        } else {
            ll coeff = d * power_mod(b, mod - 2, mod) % mod;
            for (int j = m; j <= n; j++)
                C[j] = ((C[j] - coeff * B[j - m]) % mod + mod) % mod;
            m++;
        }
    }

    /* coeffs[i] = -C[i] for i=1..L */
    for (int i = 1; i <= L; i++)
        coeffs[i] = (mod - C[i]) % mod;

    free(C); free(B); free(T);
    return L;
}

/* ============= Polynomial multiply mod char poly, mod prime ============= */
void poly_mult_mod(ll *a, ll *b, ll *result, ll *rec, int L, ll mod) {
    ll *raw = (ll*)calloc(2 * L + 1, sizeof(ll));
    for (int i = 0; i < L; i++) {
        if (a[i] == 0) continue;
        for (int j = 0; j < L; j++)
            raw[i + j] = (raw[i + j] + a[i] * b[j]) % mod;
    }

    /* Reduce: x^L = rec[0]*x^{L-1} + rec[1]*x^{L-2} + ... + rec[L-1]*x^0 */
    /* rec is 0-indexed: rec[0]=c1, rec[1]=c2, ..., rec[L-1]=cL */
    for (int i = 2 * L - 2; i >= L; i--) {
        if (raw[i] == 0) continue;
        ll c = raw[i];
        raw[i] = 0;
        for (int j = 0; j < L; j++)
            raw[i - 1 - j] = (raw[i - 1 - j] + c * rec[j]) % mod;
    }

    memcpy(result, raw, L * sizeof(ll));
    free(raw);
}

/* Evaluate recurrence at position n using Kitamasa */
ll eval_recurrence(ll *rec, ll *init, int L, ll n, ll mod) {
    if (n < L) return init[n] % mod;
    if (L == 0) return 0;

    /* rec_rev[i] = rec[L-i] for use in reduction */
    ll *rec_rev = (ll*)calloc(L, sizeof(ll));
    for (int i = 0; i < L; i++) rec_rev[i] = rec[i + 1];
    /* rec_rev is already [c1, c2, ..., cL] = rec[1..L] */

    ll *res = (ll*)calloc(L, sizeof(ll));
    ll *base = (ll*)calloc(L, sizeof(ll));
    ll *tmp = (ll*)calloc(L, sizeof(ll));
    res[0] = 1;
    if (L > 1) base[1] = 1;
    else base[0] = rec[1] % mod;

    ll exp = n;
    while (exp > 0) {
        if (exp & 1) {
            poly_mult_mod(res, base, tmp, rec + 1, L, mod);
            memcpy(res, tmp, L * sizeof(ll));
        }
        poly_mult_mod(base, base, tmp, rec + 1, L, mod);
        memcpy(base, tmp, L * sizeof(ll));
        exp >>= 1;
    }

    ll ans = 0;
    for (int i = 0; i < L; i++)
        ans = (ans + res[i] * (init[i] % mod)) % mod;

    free(rec_rev); free(res); free(base); free(tmp);
    return ans;
}

/* ============= Solve linear system mod prime ============= */
int solve_linear_mod_prime(ll **A, ll *b, ll *x, int n, ll p) {
    ll **aug = (ll**)malloc(n * sizeof(ll*));
    for (int i = 0; i < n; i++) {
        aug[i] = (ll*)malloc((n + 1) * sizeof(ll));
        memcpy(aug[i], A[i], n * sizeof(ll));
        aug[i][n] = b[i];
    }

    for (int col = 0; col < n; col++) {
        int pivot = -1;
        for (int row = col; row < n; row++) {
            if (aug[row][col] % p != 0) { pivot = row; break; }
        }
        if (pivot < 0) {
            for (int i = 0; i < n; i++) free(aug[i]);
            free(aug);
            return 0;
        }
        ll *tmp = aug[col]; aug[col] = aug[pivot]; aug[pivot] = tmp;
        ll inv = power_mod(aug[col][col] % p, p - 2, p);
        for (int j = 0; j <= n; j++)
            aug[col][j] = aug[col][j] * inv % p;
        for (int row = 0; row < n; row++) {
            if (row == col) continue;
            ll factor = ((aug[row][col] % p) + p) % p;
            for (int j = 0; j <= n; j++)
                aug[row][j] = ((aug[row][j] - factor * aug[col][j]) % p + p) % p;
        }
    }
    for (int i = 0; i < n; i++)
        x[i] = (aug[i][n] % p + p) % p;
    for (int i = 0; i < n; i++) free(aug[i]);
    free(aug);
    return 1;
}

/* ============= Find recurrence mod p^e using Hensel lifting ============= */
int find_recurrence_mod_pe(ll *seq, int num_values, int L, ll p, int e, ll *coeffs) {
    ll pe = 1;
    for (int i = 0; i < e; i++) pe *= p;

    /* Build system mod p */
    ll **A = (ll**)malloc(L * sizeof(ll*));
    ll *b_arr = (ll*)malloc(L * sizeof(ll));
    for (int i = 0; i < L; i++) {
        A[i] = (ll*)malloc(L * sizeof(ll));
        for (int j = 0; j < L; j++)
            A[i][j] = ((seq[i + L - 1 - j] % p) + p) % p;
        b_arr[i] = ((seq[i + L] % p) + p) % p;
    }

    ll *coeffs_cur = (ll*)malloc(L * sizeof(ll));
    if (!solve_linear_mod_prime(A, b_arr, coeffs_cur, L, p)) {
        for (int i = 0; i < L; i++) free(A[i]);
        free(A); free(b_arr); free(coeffs_cur);
        return 0;
    }

    /* Hensel lifting */
    ll cur_mod = p;
    for (int step = 1; step < e; step++) {
        ll next_mod = cur_mod * p;
        ll *residuals = (ll*)malloc(L * sizeof(ll));
        for (int i = 0; i < L; i++) {
            lll r = ((lll)(seq[i + L] % next_mod) + next_mod) % next_mod;
            for (int j = 0; j < L; j++)
                r = (r - (lll)coeffs_cur[j] * ((seq[i + L - 1 - j] % next_mod + next_mod) % next_mod)) % next_mod;
            r = ((r % next_mod) + next_mod) % next_mod;
            residuals[i] = (ll)((r / cur_mod) % p);
        }

        ll *delta = (ll*)malloc(L * sizeof(ll));
        if (!solve_linear_mod_prime(A, residuals, delta, L, p)) {
            free(residuals); free(delta);
            for (int i = 0; i < L; i++) free(A[i]);
            free(A); free(b_arr); free(coeffs_cur);
            return 0;
        }

        for (int j = 0; j < L; j++)
            coeffs_cur[j] = (coeffs_cur[j] + cur_mod * delta[j]) % next_mod;

        free(residuals); free(delta);
        cur_mod = next_mod;
    }

    for (int i = 0; i < L; i++)
        coeffs[i + 1] = (coeffs_cur[i] % pe + pe) % pe;

    for (int i = 0; i < L; i++) free(A[i]);
    free(A); free(b_arr); free(coeffs_cur);
    return 1;
}

/* ============= Extended GCD and CRT ============= */
ll extended_gcd(ll a, ll b, ll *x, ll *y) {
    if (b == 0) { *x = 1; *y = 0; return a; }
    ll x1, y1;
    ll g = extended_gcd(b, a % b, &x1, &y1);
    *x = y1;
    *y = x1 - (a / b) * y1;
    return g;
}

ll crt2(ll r1, ll m1, ll r2, ll m2) {
    ll x, y;
    ll g = extended_gcd(m1, m2, &x, &y);
    ll lcm = m1 / g * m2;
    ll diff = (r2 - r1) / g;
    ll r = (r1 + (lll)m1 * ((lll)diff * x % (m2 / g))) % lcm;
    if (r < 0) r += lcm;
    return r;
}

/* ============= Digit sum helper ============= */
int sum_digits_of(const char *s, int len) {
    int sum = 0;
    for (int i = 0; i < len; i++) sum += s[i] - '0';
    return sum;
}

int main() {
    const char *DIGITS = "123432";
    ll N_target = 100000000000000LL; /* 10^14 */
    ll M = 123454321LL;

    int num_values = 500;
    ll *values = (ll*)malloc(num_values * sizeof(ll));

    /* Compute cumulative sums */
    ll cum_sum = 0;
    int digit_idx = 0;
    char v_buf[1000];
    for (int term_num = 1; term_num <= num_values; term_num++) {
        int v_len = 0;
        while (sum_digits_of(v_buf, v_len) < term_num) {
            v_buf[v_len++] = DIGITS[digit_idx % 6];
            digit_idx++;
        }
        /* Compute int(v_buf) mod M */
        ll val = 0;
        for (int i = 0; i < v_len; i++)
            val = (val * 10 + (v_buf[i] - '0')) % M;
        cum_sum = (cum_sum + val) % M;
        values[term_num - 1] = cum_sum;
    }

    /* M = 41^2 * 271^2 */
    int primes[] = {41, 271};
    int exps[] = {2, 2};
    int n_factors = 2;

    ll results_r[2], results_m[2];
    for (int fi = 0; fi < n_factors; fi++) {
        ll p = primes[fi];
        int e = exps[fi];
        ll pe = 1;
        for (int i = 0; i < e; i++) pe *= p;

        /* Find recurrence order via BM mod p */
        ll *seq_p = (ll*)malloc(num_values * sizeof(ll));
        for (int i = 0; i < num_values; i++)
            seq_p[i] = ((values[i] % p) + p) % p;

        ll *bm_coeffs = (ll*)calloc(num_values + 2, sizeof(ll));
        int L = berlekamp_massey(seq_p, num_values, bm_coeffs, p);
        free(bm_coeffs);
        free(seq_p);

        if (L == 0) {
            results_r[fi] = 0;
            results_m[fi] = pe;
            continue;
        }

        /* Find recurrence mod p^e */
        ll *coeffs = (ll*)calloc(L + 2, sizeof(ll));
        find_recurrence_mod_pe(values, num_values, L, p, e, coeffs);

        /* Evaluate at N_target - 1 (0-indexed) */
        ll *init = (ll*)malloc(L * sizeof(ll));
        for (int i = 0; i < L; i++)
            init[i] = ((values[i] % pe) + pe) % pe;

        ll val = eval_recurrence(coeffs, init, L, N_target - 1, pe);
        results_r[fi] = val % pe;
        results_m[fi] = pe;

        free(coeffs);
        free(init);
    }

    /* CRT */
    ll r = crt2(results_r[0], results_m[0], results_r[1], results_m[1]);
    printf("%lld\n", r % M);

    free(values);
    return 0;
}
