/*
 * Project Euler Problem 502: Counting Castles.
 * Uses Berlekamp-Massey to find linear recurrence and Kitamasa to extrapolate.
 * Extracted from embedded C in Python solution.
 */
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <stdint.h>

typedef long long i64;
#define MOD 1000000007LL

i64 mod(i64 x) {
    x %= MOD;
    if (x < 0) x += MOD;
    return x;
}

i64 power(i64 base, i64 exp) {
    i64 result = 1;
    base = mod(base);
    while (exp > 0) {
        if (exp & 1) result = mod(result * base);
        base = mod(base * base);
        exp >>= 1;
    }
    return result;
}

i64 inv(i64 x) {
    return power(mod(x), MOD - 2);
}

i64 numCastles(int w, int h) {
    if (h <= 0) return 0;

    i64 *dp_prev0 = (i64*)calloc(h + 1, sizeof(i64));
    i64 *dp_prev1 = (i64*)calloc(h + 1, sizeof(i64));
    i64 *dp_curr0 = (i64*)calloc(h + 1, sizeof(i64));
    i64 *dp_curr1 = (i64*)calloc(h + 1, sizeof(i64));

    for (int y = 0; y <= h; y += 2)
        dp_prev0[y] = 1;

    for (int x = 1; x <= w; x++) {
        memset(dp_curr0, 0, (h + 1) * sizeof(i64));
        memset(dp_curr1, 0, (h + 1) * sizeof(i64));

        for (int y = 1; y <= h; y++) {
            i64 val0 = 0;
            val0 += dp_prev0[h];
            val0 += dp_prev0[h - 1];
            val0 -= dp_prev0[y - 1];
            val0 += dp_prev1[y - 1];
            if (y >= 2)
                val0 += dp_curr0[y - 2];
            dp_curr0[y] = mod(val0);

            i64 val1 = 0;
            val1 += dp_prev1[h];
            val1 += dp_prev1[h - 1];
            val1 -= dp_prev1[y - 1];
            val1 += dp_prev0[y - 1];
            if (y >= 2)
                val1 += dp_curr1[y - 2];
            dp_curr1[y] = mod(val1);
        }

        i64 *tmp;
        tmp = dp_prev0; dp_prev0 = dp_curr0; dp_curr0 = tmp;
        tmp = dp_prev1; dp_prev1 = dp_curr1; dp_curr1 = tmp;
    }

    i64 result = mod(dp_prev0[h] + dp_prev0[h - 1]);

    free(dp_prev0); free(dp_prev1);
    free(dp_curr0); free(dp_curr1);

    return result;
}

int berlekamp_massey(i64 *S, int n, i64 *C) {
    i64 *B = (i64*)calloc(n + 2, sizeof(i64));
    i64 *T = (i64*)calloc(n + 2, sizeof(i64));

    for (int i = 0; i <= n; i++) C[i] = B[i] = 0;
    C[0] = B[0] = 1;
    int recLen = 0, m = 1;
    i64 b = 1;

    for (int i = 0; i < n; i++) {
        i64 d = S[i];
        for (int j = 1; j <= recLen; j++)
            d = mod(d + C[j] * S[i - j]);

        if (d == 0) {
            m++;
        } else if (2 * recLen <= i) {
            memcpy(T, C, (n + 2) * sizeof(i64));
            i64 coef = mod(d * inv(b));
            for (int j = m; j <= n; j++)
                C[j] = mod(C[j] - coef * B[j - m]);
            memcpy(B, T, (n + 2) * sizeof(i64));
            recLen = i + 1 - recLen;
            b = d;
            m = 1;
        } else {
            i64 coef = mod(d * inv(b));
            for (int j = m; j <= n; j++)
                C[j] = mod(C[j] - coef * B[j - m]);
            m++;
        }
    }

    free(B);
    free(T);
    return recLen;
}

i64 linear_recurrence(i64 *C, int recLen, i64 *a, i64 n) {
    if (n < recLen) return a[n];
    if (recLen == 0) return 0;

    i64 *q = (i64*)calloc(recLen + 1, sizeof(i64));
    i64 *r = (i64*)calloc(recLen + 1, sizeof(i64));
    i64 *tmp = (i64*)calloc(2 * recLen + 2, sizeof(i64));

    q[0] = 1;
    r[1] = 1;

    i64 exp = n;
    while (exp > 0) {
        if (exp & 1) {
            memset(tmp, 0, (2 * recLen + 2) * sizeof(i64));
            for (int i = 0; i < recLen; i++)
                for (int j = 0; j < recLen; j++)
                    tmp[i + j] = mod(tmp[i + j] + q[i] * r[j]);
            for (int i = 2 * recLen - 2; i >= recLen; i--) {
                for (int j = 1; j <= recLen; j++)
                    tmp[i - j] = mod(tmp[i - j] + tmp[i] * C[j]);
                tmp[i] = 0;
            }
            for (int i = 0; i < recLen; i++) q[i] = tmp[i];
        }
        memset(tmp, 0, (2 * recLen + 2) * sizeof(i64));
        for (int i = 0; i < recLen; i++)
            for (int j = 0; j < recLen; j++)
                tmp[i + j] = mod(tmp[i + j] + r[i] * r[j]);
        for (int i = 2 * recLen - 2; i >= recLen; i--) {
            for (int j = 1; j <= recLen; j++)
                tmp[i - j] = mod(tmp[i - j] + tmp[i] * C[j]);
            tmp[i] = 0;
        }
        for (int i = 0; i < recLen; i++) r[i] = tmp[i];
        exp >>= 1;
    }

    i64 result = 0;
    for (int i = 0; i < recLen; i++)
        result = mod(result + q[i] * a[i]);

    free(q);
    free(r);
    free(tmp);
    return result;
}

i64 extrapolate(i64 *values, int n, i64 x) {
    if (x <= n) return values[x - 1];

    i64 *C = (i64*)calloc(n + 2, sizeof(i64));
    int recLen = berlekamp_massey(values, n, C);

    for (int i = 1; i <= recLen; i++)
        C[i] = mod(-C[i]);

    i64 result = linear_recurrence(C, recLen, values, x - 1);

    free(C);
    return result;
}

i64 numCastlesBig(i64 W, i64 H) {
    int L = 500;

    i64 *values = (i64*)malloc((L + 1) * sizeof(i64));

    if (W <= 100) {
        for (int h = 1; h <= L; h++)
            values[h - 1] = numCastles((int)W, h);
        i64 result = extrapolate(values, L, H);
        free(values);
        return result;
    }
    if (H <= 100) {
        for (int w = 1; w <= L; w++)
            values[w - 1] = numCastles(w, (int)H);
        i64 result = extrapolate(values, L, W);
        free(values);
        return result;
    }

    free(values);
    return numCastles((int)W, (int)H);
}

int main() {
    i64 ans = 0;

    i64 W = 1000000000000LL, H = 100;
    ans = mod(ans + numCastlesBig(W, H) - numCastlesBig(W, H - 1));

    W = 10000; H = 10000;
    ans = mod(ans + numCastlesBig(W, H) - numCastlesBig(W, H - 1));

    W = 100; H = 1000000000000LL;
    ans = mod(ans + numCastlesBig(W, H) - numCastlesBig(W, H - 1));

    printf("%lld\n", ans);
    return 0;
}
